use std::mem::MaybeUninit;
use std::os::raw::c_int;
use std::{ptr, slice};

use libheif_sys as lh;

use crate::{
    Channel, ColorProfileNCLX, ColorProfileRaw, ColorProfileType, ColorSpace, HeifError, HeifErrorCode,
    HeifErrorSubCode, Result,
};

const MAX_IMAGE_SIZE: u32 = i32::MAX as _;

pub struct Plane<T> {
    pub data: T,
    pub width: u32,
    pub height: u32,
    pub stride: usize,
    pub bits_per_pixel: u8,
    pub storage_bits_per_pixel: u8,
}

pub struct Planes<T> {
    pub y: Option<Plane<T>>,
    pub cb: Option<Plane<T>>,
    pub cr: Option<Plane<T>>,
    pub r: Option<Plane<T>>,
    pub g: Option<Plane<T>>,
    pub b: Option<Plane<T>>,
    pub a: Option<Plane<T>>,
    pub interleaved: Option<Plane<T>>,
}

pub struct Image {
    pub(crate) inner: *mut lh::heif_image,
}

pub struct ScalingOptions {}

#[allow(unsafe_code)]
impl Image {
    /// Create a new image of the specified resolution and colorspace.
    /// Note: no memory for the actual image data is reserved yet. You have to use
    /// [`Image::create_plane()`] method to add image planes required by your colorspace.
    #[allow(unsafe_code)]
    pub fn new(width: u32, height: u32, color_space: ColorSpace) -> Result<Image> {
        if width > MAX_IMAGE_SIZE || height > MAX_IMAGE_SIZE {
            return Err(HeifError {
                code: HeifErrorCode::UsageError,
                sub_code: HeifErrorSubCode::InvalidBoxSize,
                message: "width or height is greater than MAX_IMAGE_SIZE".to_string(),
            });
        }

        let mut c_image = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_create(
                width as _,
                height as _,
                color_space.heif_color_space(),
                color_space.heif_chroma(),
                c_image.as_mut_ptr(),
            )
        };
        HeifError::from_heif_error(err)?;
        Ok(Image {
            inner: unsafe { c_image.assume_init() },
        })
    }

    #[inline]
    pub(crate) fn from_heif_image(image: *mut lh::heif_image) -> Image {
        Image { inner: image }
    }

    /// Get the width of the main channel (Y in YCbCr, or any in RGB).
    #[allow(unsafe_code)]
    pub fn width(&self) -> u32 {
        unsafe { lh::heif_image_get_primary_width(self.inner).max(0) as u32 }
    }

    /// Get the height of the main channel (Y in YCbCr, or any in RGB).
    #[allow(unsafe_code)]
    pub fn height(&self) -> u32 {
        unsafe { lh::heif_image_get_primary_height(self.inner).max(0) as u32 }
    }

    /// Get width of the given image channel in pixels.
    #[allow(unsafe_code)]
    pub fn channel_width(&self, channel: Channel) -> Option<u32> {
        let value = unsafe { lh::heif_image_get_width(self.inner, channel as _) };
        (value >= 0).then_some(value as _)
    }

    /// Get height of the given image channel in pixels.
    #[allow(unsafe_code)]
    pub fn channel_height(&self, channel: Channel) -> Option<u32> {
        let value = unsafe { lh::heif_image_get_height(self.inner, channel as _) };
        (value >= 0).then_some(value as _)
    }

    /// Get the number of bits per pixel in the given image channel. Returns
    /// `None` if a non-existing channel was given.
    ///
    /// Note that the number of bits per pixel may be different for each color channel.
    /// This function returns the number of bits used for storage of each pixel.
    /// Especially for HDR images, this is probably not what you want. Have a look at
    /// [`Image::bits_per_pixel()`] instead.
    #[allow(unsafe_code)]
    pub fn storage_bits_per_pixel(&self, channel: Channel) -> Option<u8> {
        let value = unsafe { lh::heif_image_get_bits_per_pixel(self.inner, channel as _) };
        (value >= 0).then_some(value as _)
    }

    /// Get the number of bits per pixel in the given image channel. Returns
    /// `None` if a non-existing channel was given.
    ///
    /// This function returns the number of bits used for representing
    /// the pixel value, which might be smaller than the number of bits used
    /// in memory. For example, in 12bit HDR images, this function returns `12`,
    /// while still 16 bits are reserved for storage. For interleaved RGBA with
    /// 12 bit, this function also returns `12`, not `48` or `64`
    /// ([`Image::storage_bits_per_pixel()`] returns `64` in this case).
    #[allow(unsafe_code)]
    pub fn bits_per_pixel(&self, channel: Channel) -> Option<u8> {
        let value = unsafe { lh::heif_image_get_bits_per_pixel_range(self.inner, channel as _) };
        (value >= 0).then_some(value as _)
    }

    #[cfg(feature = "v1_20")]
    #[allow(unsafe_code)]
    fn plane_inner(&self, channel: Channel) -> (*const u8, usize) {
        let mut stride: usize = 1;
        let data = unsafe { lh::heif_image_get_plane_readonly2(self.inner, channel as _, &mut stride) };
        (data, stride)
    }

    #[cfg(not(feature = "v1_20"))]
    #[allow(unsafe_code)]
    fn plane_inner(&self, channel: Channel) -> (*const u8, usize) {
        let mut stride: i32 = 1;
        let data = unsafe { lh::heif_image_get_plane_readonly(self.inner, channel as _, &mut stride) };
        (data, stride as _)
    }

    #[allow(unsafe_code)]
    fn plane(&self, channel: Channel) -> Option<Plane<&[u8]>> {
        let (data, stride) = self.plane_inner(channel);
        if data.is_null() {
            return None;
        }

        let width = self.channel_width(channel).unwrap_or_default();
        let height = self.channel_height(channel).unwrap_or_default();
        let bits_per_pixel = self.bits_per_pixel(channel).unwrap_or_default();
        let storage_bits_per_pixel = self.storage_bits_per_pixel(channel).unwrap_or_default();
        let size = height as usize * stride;
        let bytes = unsafe { slice::from_raw_parts(data, size) };
        Some(Plane {
            data: bytes,
            bits_per_pixel,
            storage_bits_per_pixel,
            width,
            height,
            stride,
        })
    }

    #[cfg(feature = "v1_20")]
    #[allow(unsafe_code)]
    fn plane_mut_inner(&self, channel: Channel) -> (*mut u8, usize) {
        let mut stride: usize = 1;
        let data = unsafe { lh::heif_image_get_plane2(self.inner, channel as _, &mut stride) };
        (data, stride)
    }

    #[cfg(not(feature = "v1_20"))]
    #[allow(unsafe_code)]
    fn plane_mut_inner(&self, channel: Channel) -> (*mut u8, usize) {
        let mut stride: i32 = 1;
        let data = unsafe { lh::heif_image_get_plane(self.inner, channel as _, &mut stride) };
        (data, stride as _)
    }

    #[allow(clippy::mut_from_ref)]
    #[allow(unsafe_code)]
    fn plane_mut(&self, channel: Channel) -> Option<Plane<&mut [u8]>> {
        let (data, stride): (*mut u8, usize) = self.plane_mut_inner(channel);
        if data.is_null() {
            return None;
        }

        let width = self.channel_width(channel).unwrap_or_default();
        let height = self.channel_height(channel).unwrap_or_default();
        let bits_per_pixel = self.bits_per_pixel(channel).unwrap_or_default();
        let storage_bits_per_pixel = self.storage_bits_per_pixel(channel).unwrap_or_default();
        let size = height as usize * stride;
        let bytes = unsafe { slice::from_raw_parts_mut(data, size) };
        Some(Plane {
            data: bytes,
            bits_per_pixel,
            storage_bits_per_pixel,
            width,
            height,
            stride,
        })
    }

    #[allow(unsafe_code)]
    pub fn planes(&self) -> Planes<&[u8]> {
        Planes {
            y: self.plane(Channel::Y),
            cb: self.plane(Channel::Cb),
            cr: self.plane(Channel::Cr),
            r: self.plane(Channel::R),
            g: self.plane(Channel::G),
            b: self.plane(Channel::B),
            a: self.plane(Channel::Alpha),
            interleaved: self.plane(Channel::Interleaved),
        }
    }

    #[allow(unsafe_code)]
    pub fn planes_mut(&mut self) -> Planes<&mut [u8]> {
        Planes {
            y: self.plane_mut(Channel::Y),
            cb: self.plane_mut(Channel::Cb),
            cr: self.plane_mut(Channel::Cr),
            r: self.plane_mut(Channel::R),
            g: self.plane_mut(Channel::G),
            b: self.plane_mut(Channel::B),
            a: self.plane_mut(Channel::Alpha),
            interleaved: self.plane_mut(Channel::Interleaved),
        }
    }

    #[allow(unsafe_code)]
    pub fn has_channel(&self, channel: Channel) -> bool {
        unsafe { lh::heif_image_has_channel(self.inner, channel as _) != 0 }
    }

    //    pub fn channels(&self) -> Vec<Channel> {
    //        let mut res = Vec::from_iter();
    //        for channel in Channel::iter() {
    //            if self.has_channel(channel) {
    //                res.insert(channel);
    //            }
    //        }
    //        res
    //    }

    #[allow(unsafe_code)]
    pub fn color_space(&self) -> Option<ColorSpace> {
        unsafe {
            ColorSpace::from_libheif(
                lh::heif_image_get_colorspace(self.inner),
                lh::heif_image_get_chroma_format(self.inner),
            )
        }
    }

    /// Scale image by "nearest neighbor" method.
    ///
    /// Note: Currently, `_scaling_options` is not used. Pass a `None`.
    #[allow(unsafe_code)]
    pub fn scale(&self, width: u32, height: u32, _scaling_options: Option<ScalingOptions>) -> Result<Image> {
        let mut c_image = MaybeUninit::<_>::uninit();
        let err = unsafe {
            lh::heif_image_scale_image(self.inner, c_image.as_mut_ptr(), width as _, height as _, ptr::null())
        };
        HeifError::from_heif_error(err)?;
        Ok(Image {
            inner: unsafe { c_image.assume_init() },
        })
    }

    /// The indicated bit_depth corresponds to the bit depth per channel.
    /// I.e. for interleaved formats like RRGGBB, the bit_depth would be,
    /// e.g., 10 bit instead of 30 bits or 3*16=48 bits.
    /// For backward compatibility, one can also specify 24bits for RGB and
    /// 32bits for RGBA, instead of the preferred 8 bits.
    #[allow(unsafe_code)]
    pub fn create_plane(&mut self, channel: Channel, width: u32, height: u32, bit_depth: u8) -> Result<()> {
        let err = unsafe {
            lh::heif_image_add_plane(
                self.inner,
                channel as _,
                width as _,
                height as _,
                c_int::from(bit_depth),
            )
        };
        HeifError::from_heif_error(err)
    }

    #[allow(unsafe_code)]
    pub fn set_premultiplied_alpha(&self, is_premultiplied_alpha: bool) {
        unsafe { lh::heif_image_set_premultiplied_alpha(self.inner, is_premultiplied_alpha as _) };
    }

    #[allow(unsafe_code)]
    pub fn is_premultiplied_alpha(&self) -> bool {
        unsafe { lh::heif_image_is_premultiplied_alpha(self.inner) != 0 }
    }

    #[allow(unsafe_code)]
    pub fn color_profile_raw(&self) -> Option<ColorProfileRaw> {
        let size = unsafe { lh::heif_image_get_raw_color_profile_size(self.inner) };
        if size == 0 {
            return None;
        }
        let mut result: Vec<u8> = Vec::with_capacity(size);
        let err = unsafe { lh::heif_image_get_raw_color_profile(self.inner, result.as_ptr() as _) };
        if err.code != 0 {
            // Only one error is possible inside `libheif` - `ColorProfileDoesNotExist`
            return None;
        }
        unsafe {
            result.set_len(size);
        }
        let c_profile_type = unsafe { lh::heif_image_get_color_profile_type(self.inner) };
        // `c_profile_type` on Windows will be i32, so we need to cast it to u32
        let profile_type = ColorProfileType::from(c_profile_type as u32);

        Some(ColorProfileRaw {
            typ: profile_type,
            data: result,
        })
    }

    #[allow(unsafe_code)]
    pub fn set_color_profile_raw(&mut self, profile: &ColorProfileRaw) -> Result<()> {
        let err = unsafe {
            let mut c_profile_type = [0u8; 5];
            c_profile_type[0..4].copy_from_slice(&profile.typ.0);
            lh::heif_image_set_raw_color_profile(
                self.inner,
                c_profile_type.as_ptr() as _,
                profile.data.as_ptr() as _,
                profile.data.len(),
            )
        };
        HeifError::from_heif_error(err)
    }

    #[allow(unsafe_code)]
    pub fn color_profile_nclx(&self) -> Option<ColorProfileNCLX> {
        let mut profile_ptr = MaybeUninit::<_>::uninit();
        let err = unsafe { lh::heif_image_get_nclx_color_profile(self.inner, profile_ptr.as_mut_ptr()) };
        if err.code != 0 {
            // Only one error is possible inside `libheif` - `ColorProfileDoesNotExist`
            return None;
        }
        let profile_ptr = unsafe { profile_ptr.assume_init() };
        Some(ColorProfileNCLX { inner: profile_ptr })
    }

    #[allow(unsafe_code)]
    pub fn set_color_profile_nclx(&mut self, profile: &ColorProfileNCLX) -> Result<()> {
        let err = unsafe { lh::heif_image_set_nclx_color_profile(self.inner, profile.inner) };
        HeifError::from_heif_error(err)
    }

    #[allow(unsafe_code)]
    pub fn pixel_aspect_ratio(&self) -> (u32, u32) {
        let mut aspect_h = 0;
        let mut aspect_v = 0;
        unsafe {
            lh::heif_image_get_pixel_aspect_ratio(self.inner, &mut aspect_h as _, &mut aspect_v as _);
        }
        (aspect_h, aspect_v)
    }

    #[allow(unsafe_code)]
    pub fn set_pixel_aspect_ratio(&mut self, aspect_h: u32, aspect_v: u32) {
        unsafe {
            lh::heif_image_set_pixel_aspect_ratio(self.inner, aspect_h, aspect_v);
        }
    }

    #[cfg(feature = "v1_20")]
    #[allow(unsafe_code)]
    pub fn duration(&self) -> u32 {
        unsafe { lh::heif_image_get_duration(self.inner) }
    }

    #[cfg(feature = "v1_20")]
    #[allow(unsafe_code)]
    pub fn set_duration(&self, duration: u32) {
        unsafe { lh::heif_image_set_duration(self.inner, duration) }
    }
}

#[allow(unsafe_code)]
impl Drop for Image {
    #[allow(unsafe_code)]
    fn drop(&mut self) {
        unsafe { lh::heif_image_release(self.inner) };
    }
}

#[allow(unsafe_code)]
unsafe impl Send for Image {}
