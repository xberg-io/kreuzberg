use image::RgbImage;
use ndarray::Array4;

/// ImageNet normalization constants.
const IMAGENET_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const IMAGENET_STD: [f32; 3] = [0.229, 0.224, 0.225];

/// Preprocess an image for models using ImageNet normalization (e.g., RT-DETR).
///
/// Pipeline: resize to target_size x target_size (bilinear) -> rescale /255 -> ImageNet normalize -> NCHW f32.
///
/// Uses a single vectorized pass over contiguous pixel data for maximum throughput.
pub(crate) fn preprocess_imagenet(img: &RgbImage, target_size: u32) -> Array4<f32> {
    let resized = image::imageops::resize(img, target_size, target_size, image::imageops::FilterType::Triangle);
    let pixels = resized.as_raw();
    let hw = (target_size * target_size) as usize;

    // Pre-compute reciprocals to avoid repeated division.
    let inv_std_r = 1.0 / IMAGENET_STD[0];
    let inv_std_g = 1.0 / IMAGENET_STD[1];
    let inv_std_b = 1.0 / IMAGENET_STD[2];

    // Allocate contiguous NCHW buffer: [R plane | G plane | B plane].
    let mut data = vec![0.0f32; 3 * hw];

    for i in 0..hw {
        let r = pixels[i * 3] as f32 * (1.0 / 255.0);
        let g = pixels[i * 3 + 1] as f32 * (1.0 / 255.0);
        let b = pixels[i * 3 + 2] as f32 * (1.0 / 255.0);
        data[i] = (r - IMAGENET_MEAN[0]) * inv_std_r;
        data[hw + i] = (g - IMAGENET_MEAN[1]) * inv_std_g;
        data[2 * hw + i] = (b - IMAGENET_MEAN[2]) * inv_std_b;
    }

    Array4::from_shape_vec((1, 3, target_size as usize, target_size as usize), data)
        .expect("shape mismatch in preprocess_imagenet")
}

/// Preprocess with aspect-preserving letterbox and ImageNet normalization.
///
/// Pipeline: letterbox-resize to target_size × target_size (Lanczos3, aspect-preserving)
///           → rescale /255 → ImageNet normalize → NCHW f32.
///
/// Unlike `preprocess_imagenet` which squashes the image to a square (distorting
/// aspect ratio), this preserves the original proportions and pads with the ImageNet
/// mean color. This produces more accurate detection coordinates because the model
/// sees undistorted geometry.
///
/// Returns `(tensor, scale, pad_x, pad_y)`:
/// - `scale`: resize factor applied (for mapping detections back)
/// - `pad_x`, `pad_y`: top-left offset of the resized image within the padded square
pub(crate) fn preprocess_imagenet_letterbox(img: &RgbImage, target_size: u32) -> (Array4<f32>, f32, u32, u32) {
    let (orig_w, orig_h) = (img.width() as f32, img.height() as f32);
    let scale = (target_size as f32 / orig_w).min(target_size as f32 / orig_h);
    let new_w = (orig_w * scale).round() as u32;
    let new_h = (orig_h * scale).round() as u32;

    // Resize with CatmullRom — good quality/speed trade-off for document images.
    // Lanczos3 is ~2x slower with minimal quality gain for text-heavy pages.
    let resized = image::imageops::resize(img, new_w, new_h, image::imageops::FilterType::CatmullRom);

    // Center the resized image in the target square
    let pad_x = (target_size - new_w) / 2;
    let pad_y = (target_size - new_h) / 2;

    let ts = target_size as usize;
    let hw = ts * ts;

    // Pre-compute reciprocals
    let inv_std_r = 1.0 / IMAGENET_STD[0];
    let inv_std_g = 1.0 / IMAGENET_STD[1];
    let inv_std_b = 1.0 / IMAGENET_STD[2];

    // Fill with normalized ImageNet mean (what a "blank" pixel looks like after normalization)
    let pad_r = (0.5 - IMAGENET_MEAN[0]) * inv_std_r;
    let pad_g = (0.5 - IMAGENET_MEAN[1]) * inv_std_g;
    let pad_b = (0.5 - IMAGENET_MEAN[2]) * inv_std_b;

    let mut data = vec![0.0f32; 3 * hw];
    // Fill padding
    for i in 0..hw {
        data[i] = pad_r;
        data[hw + i] = pad_g;
        data[2 * hw + i] = pad_b;
    }

    // Copy resized pixels into the center
    let rw = new_w as usize;
    let rh = new_h as usize;
    let resized_pixels = resized.as_raw();
    let px = pad_x as usize;
    let py = pad_y as usize;

    for y in 0..rh {
        for x in 0..rw {
            let src_idx = (y * rw + x) * 3;
            let dst_idx = (y + py) * ts + (x + px);
            let r = resized_pixels[src_idx] as f32 * (1.0 / 255.0);
            let g = resized_pixels[src_idx + 1] as f32 * (1.0 / 255.0);
            let b = resized_pixels[src_idx + 2] as f32 * (1.0 / 255.0);
            data[dst_idx] = (r - IMAGENET_MEAN[0]) * inv_std_r;
            data[hw + dst_idx] = (g - IMAGENET_MEAN[1]) * inv_std_g;
            data[2 * hw + dst_idx] = (b - IMAGENET_MEAN[2]) * inv_std_b;
        }
    }

    let tensor = Array4::from_shape_vec((1, 3, ts, ts), data).expect("shape mismatch in preprocess_imagenet_letterbox");

    (tensor, scale, pad_x, pad_y)
}

/// Preprocess with rescale only (no ImageNet normalization).
///
/// Pipeline: resize to target_size x target_size -> rescale /255 -> NCHW f32.
pub(crate) fn preprocess_rescale(img: &RgbImage, target_size: u32) -> Array4<f32> {
    let resized = image::imageops::resize(img, target_size, target_size, image::imageops::FilterType::Triangle);
    let pixels = resized.as_raw();
    let hw = (target_size * target_size) as usize;

    let mut data = vec![0.0f32; 3 * hw];
    for i in 0..hw {
        data[i] = pixels[i * 3] as f32 * (1.0 / 255.0);
        data[hw + i] = pixels[i * 3 + 1] as f32 * (1.0 / 255.0);
        data[2 * hw + i] = pixels[i * 3 + 2] as f32 * (1.0 / 255.0);
    }

    Array4::from_shape_vec((1, 3, target_size as usize, target_size as usize), data)
        .expect("shape mismatch in preprocess_rescale")
}

/// Letterbox preprocessing for YOLOX-style models.
///
/// Resizes the image to fit within (target_width x target_height) while maintaining
/// aspect ratio, padding the remaining area with value 114.0 (raw pixel value).
/// No normalization — values are 0-255 as YOLOX expects.
///
/// Returns the NCHW tensor and the scale ratio (for rescaling detections back).
pub(crate) fn preprocess_letterbox(img: &RgbImage, target_width: u32, target_height: u32) -> (Array4<f32>, f32) {
    let (orig_w, orig_h) = (img.width() as f32, img.height() as f32);
    let scale = (target_height as f32 / orig_h).min(target_width as f32 / orig_w);
    let new_w = (orig_w * scale) as u32;
    let new_h = (orig_h * scale) as u32;

    let resized = image::imageops::resize(img, new_w, new_h, image::imageops::FilterType::Triangle);

    let tw = target_width as usize;
    let th = target_height as usize;
    let hw = th * tw;
    // Fill with padding value 114.0 (raw pixel value, no normalization).
    let mut data = vec![114.0f32; 3 * hw];

    let rw = new_w as usize;
    let rh = new_h as usize;
    let resized_pixels = resized.as_raw();

    for y in 0..rh {
        for x in 0..rw {
            let src_idx = (y * rw + x) * 3;
            let dst_idx = y * tw + x;
            data[dst_idx] = resized_pixels[src_idx] as f32;
            data[hw + dst_idx] = resized_pixels[src_idx + 1] as f32;
            data[2 * hw + dst_idx] = resized_pixels[src_idx + 2] as f32;
        }
    }

    let tensor = Array4::from_shape_vec((1, 3, th, tw), data).expect("shape mismatch in preprocess_letterbox");

    (tensor, scale)
}
