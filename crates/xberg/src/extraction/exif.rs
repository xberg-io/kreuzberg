//! EXIF metadata extraction, powered by `nom-exif`.
//!
//! Pure Rust — works on every target (including `wasm-target` and
//! `android-target`). Compiled under any of the `ocr`, `ocr-wasm`, or `heic`
//! features. Without those features, `extract_exif_data` is a no-op stub so
//! the rest of the image extraction path keeps the same signature.

use std::collections::HashMap;

/// Extract EXIF data from image bytes.
///
/// Returns a HashMap of EXIF tag names to display strings. Empty when EXIF is
/// absent, malformed, or the container is unrecognised.
///
/// Backed by `nom-exif`, which supports EXIF blocks across JPEG, PNG, TIFF,
/// HEIC/HEIF, AVIF and several video containers in a single pure-Rust API.
#[cfg(any(feature = "ocr", feature = "ocr-wasm", feature = "heic"))]
pub(crate) fn extract_exif_data(bytes: &[u8]) -> HashMap<String, String> {
    use nom_exif::{Exif, ExifIter, ExifTag, MediaParser, MediaSource};

    let mut exif_map = HashMap::new();

    let bytes_owned = bytes::Bytes::copy_from_slice(bytes);
    let Ok(ms) = MediaSource::from_memory(bytes_owned) else {
        return exif_map;
    };

    let mut parser = MediaParser::new();
    let Ok(iter): nom_exif::Result<ExifIter> = parser.parse_exif(ms) else {
        return exif_map;
    };
    let exif: Exif = iter.into();

    // Extensive tag coverage: identity, timestamps, exposure, lens, GPS, thumbnail,
    // colour space, and provenance.
    const TAGS: &[(ExifTag, &str)] = &[
        // Identity / provenance
        (ExifTag::Make, "Make"),
        (ExifTag::Model, "Model"),
        (ExifTag::Software, "Software"),
        (ExifTag::HostComputer, "HostComputer"),
        (ExifTag::ImageDescription, "ImageDescription"),
        (ExifTag::Copyright, "Copyright"),
        (ExifTag::CameraSerialNumber, "CameraSerialNumber"),
        (ExifTag::ImageUniqueID, "ImageUniqueID"),
        (ExifTag::ExifVersion, "ExifVersion"),
        // Timestamps
        (ExifTag::ModifyDate, "DateTime"),
        (ExifTag::DateTimeOriginal, "DateTimeOriginal"),
        (ExifTag::CreateDate, "DateTimeDigitized"),
        (ExifTag::OffsetTime, "OffsetTime"),
        (ExifTag::OffsetTimeOriginal, "OffsetTimeOriginal"),
        (ExifTag::OffsetTimeDigitized, "OffsetTimeDigitized"),
        (ExifTag::SubSecTime, "SubSecTime"),
        (ExifTag::SubSecTimeOriginal, "SubSecTimeOriginal"),
        (ExifTag::SubSecTimeDigitized, "SubSecTimeDigitized"),
        // Image geometry / resolution
        (ExifTag::ImageWidth, "ImageWidth"),
        (ExifTag::ImageHeight, "ImageHeight"),
        (ExifTag::ExifImageWidth, "ExifImageWidth"),
        (ExifTag::ExifImageHeight, "ExifImageHeight"),
        (ExifTag::Orientation, "Orientation"),
        (ExifTag::XResolution, "XResolution"),
        (ExifTag::YResolution, "YResolution"),
        (ExifTag::ResolutionUnit, "ResolutionUnit"),
        (ExifTag::ColorSpace, "ColorSpace"),
        // Exposure
        (ExifTag::ExposureTime, "ExposureTime"),
        (ExifTag::FNumber, "FNumber"),
        (ExifTag::ApertureValue, "ApertureValue"),
        (ExifTag::ShutterSpeedValue, "ShutterSpeedValue"),
        (ExifTag::ExposureProgram, "ExposureProgram"),
        (ExifTag::ExposureMode, "ExposureMode"),
        (ExifTag::ExposureBiasValue, "ExposureBiasValue"),
        (ExifTag::ISOSpeedRatings, "ISO"),
        (ExifTag::SensitivityType, "SensitivityType"),
        (ExifTag::MeteringMode, "MeteringMode"),
        (ExifTag::LightSource, "LightSource"),
        (ExifTag::Flash, "Flash"),
        (ExifTag::WhiteBalanceMode, "WhiteBalance"),
        (ExifTag::SceneCaptureType, "SceneCaptureType"),
        (ExifTag::SubjectDistance, "SubjectDistance"),
        (ExifTag::SubjectDistanceRange, "SubjectDistanceRange"),
        (ExifTag::SubjectArea, "SubjectArea"),
        (ExifTag::DigitalZoomRatio, "DigitalZoomRatio"),
        (ExifTag::Contrast, "Contrast"),
        (ExifTag::Saturation, "Saturation"),
        (ExifTag::Sharpness, "Sharpness"),
        // Lens
        (ExifTag::FocalLength, "FocalLength"),
        (ExifTag::FocalLengthIn35mmFilm, "FocalLengthIn35mmFilm"),
        (ExifTag::LensMake, "LensMake"),
        (ExifTag::LensModel, "LensModel"),
        (ExifTag::LensSpecification, "LensSpecification"),
        (ExifTag::LensSerialNumber, "LensSerialNumber"),
        // GPS
        (ExifTag::GPSLatitudeRef, "GPSLatitudeRef"),
        (ExifTag::GPSLatitude, "GPSLatitude"),
        (ExifTag::GPSLongitudeRef, "GPSLongitudeRef"),
        (ExifTag::GPSLongitude, "GPSLongitude"),
        (ExifTag::GPSAltitudeRef, "GPSAltitudeRef"),
        (ExifTag::GPSAltitude, "GPSAltitude"),
        (ExifTag::GPSTimeStamp, "GPSTimeStamp"),
        (ExifTag::GPSDateStamp, "GPSDateStamp"),
        (ExifTag::GPSSpeed, "GPSSpeed"),
        (ExifTag::GPSSpeedRef, "GPSSpeedRef"),
        (ExifTag::GPSTrack, "GPSTrack"),
        (ExifTag::GPSTrackRef, "GPSTrackRef"),
        (ExifTag::GPSImgDirection, "GPSImgDirection"),
        (ExifTag::GPSImgDirectionRef, "GPSImgDirectionRef"),
        (ExifTag::GPSMapDatum, "GPSMapDatum"),
        (ExifTag::GPSProcessingMethod, "GPSProcessingMethod"),
        // Thumbnail
        (ExifTag::ThumbnailOffset, "ThumbnailOffset"),
        (ExifTag::ThumbnailLength, "ThumbnailLength"),
    ];

    for (tag, field_name) in TAGS {
        if let Some(value) = exif.get(*tag) {
            exif_map.insert((*field_name).to_string(), value.to_string());
        }
    }

    exif_map
}

/// Stub EXIF extraction when no EXIF-capable feature is active.
///
/// Compiled under any feature combination so the rest of the image path keeps
/// the same signature, but reranker-only builds — which drop every caller —
/// would surface it as `dead_code` without the `#[allow]`.
#[cfg(not(any(feature = "ocr", feature = "ocr-wasm", feature = "heic")))]
#[allow(dead_code)]
pub(crate) fn extract_exif_data(_bytes: &[u8]) -> HashMap<String, String> {
    HashMap::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_map_for_non_image_bytes() {
        assert!(extract_exif_data(b"hello world").is_empty());
        assert!(extract_exif_data(&[]).is_empty());
    }
}
