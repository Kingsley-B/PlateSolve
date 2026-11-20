//! Loading photos: JPEG decode, conversion to brightness, EXIF.
//!
//! Suggested crates (add to Cargo.toml):
//!   image        - JPEG decoding
//!   kamadak-exif - EXIF reading (imported as `exif`)

use super::types::{GrayImage, ImageMeta, SolveError};
use image::ImageFormat;
use std::fs;
use std::io::Cursor;
use std::path::Path;

use exif::{Context, Exif, In, Reader, Tag, Value};

/// OffsetTimeOriginal (Exif 2.31): the UTC offset when the photo was taken,
/// e.g. "+11:00". Written by tag number in case the crate has no named constant.
const OFFSET_TIME_ORIGINAL: Tag = Tag(Context::Exif, 0x9011);

/// Read and decode a JPEG file.
pub fn load_jpeg(path: &Path) -> Result<(GrayImage, ImageMeta), SolveError> {
    let bytes: Vec<u8> = fs::read(path)?;
    decode_jpeg(&bytes)
}

/// Decode JPEG bytes into a brightness image plus metadata.
pub fn decode_jpeg(bytes: &[u8]) -> Result<(GrayImage, ImageMeta), SolveError> {
    // Decompress: JPEG bytes -> image in memory
    let decoded = image::load_from_memory_with_format(bytes, ImageFormat::Jpeg)
        .map_err(|e| SolveError::ImageDecode(e.to_string()))?;

    // Make sure we have plain 8-bit RGB, whatever the JPEG stored
    let rgb = decoded.to_rgb8();
    let (width, height) = rgb.dimensions();
    let pixels: Vec<u8> = rgb.into_raw(); // [R, G, B, R, G, B, ...] row by row

    let gray_image = rgb_to_linear_luminance(&pixels, width, height);
    let mut meta = read_exif(bytes);
    meta.width = width;
    meta.height = height;

    Ok((gray_image, meta))
}

/// Undo the camera's sRGB tone curve and combine channels into brightness.
pub fn rgb_to_linear_luminance(rgb: &[u8], width: u32, height: u32) -> GrayImage {
    // Linear value for every possible byte value, computed once
    let mut lut = [0.0f32; 256];
    for (v, out) in lut.iter_mut().enumerate() {
        let c = v as f32 / 255.0;
        *out = if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        };
    }

    let pixels: Vec<f32> = rgb
        .chunks_exact(3)
        .map(|px| {
            0.2126 * lut[px[0] as usize]
                + 0.7152 * lut[px[1] as usize]
                + 0.0722 * lut[px[2] as usize]
        })
        .collect();

    GrayImage {
        width,
        height,
        pixels,
    }
}
/// Pull make, model, focal length and capture time from EXIF.
/// Missing or unreadable EXIF is not an error: the fields just stay None.
pub fn read_exif(bytes: &[u8]) -> ImageMeta {
    let mut meta = ImageMeta::default();

    let exif = match Reader::new().read_from_container(&mut Cursor::new(bytes)) {
        Ok(exif) => exif,
        Err(_) => return meta,
    };

    meta.make = ascii_field(&exif, Tag::Make);
    meta.model = ascii_field(&exif, Tag::Model);
    meta.focal_length_mm = rational_field(&exif, Tag::FocalLength);
    meta.capture_time = ascii_field(&exif, Tag::DateTimeOriginal);
    meta.capture_time_offset = ascii_field(&exif, OFFSET_TIME_ORIGINAL);
    meta
}

/// A text field, e.g. Make = "FUJIFILM". None if missing, empty or not text.
fn ascii_field(exif: &Exif, tag: Tag) -> Option<String> {
    let field = exif.get_field(tag, In::PRIMARY)?;
    match &field.value {
        Value::Ascii(parts) => parts
            .first()
            .map(|p| {
                String::from_utf8_lossy(p)
                    .trim_matches(char::from(0))
                    .trim()
                    .to_string()
            })
            .filter(|s| !s.is_empty()),
        _ => None,
    }
}

/// A fraction field, e.g. FocalLength = 160/10 -> 16.0. None if missing or invalid.
fn rational_field(exif: &Exif, tag: Tag) -> Option<f64> {
    let field = exif.get_field(tag, In::PRIMARY)?;
    match &field.value {
        Value::Rational(values) => values
            .first()
            .map(|r| r.to_f64())
            .filter(|v| v.is_finite() && *v > 0.0),
        _ => None,
    }
}

/// Average 2x2 blocks into one pixel. Optional speed-up for full-resolution
/// (6240 px) files; the phone-transfer size (~3000 px) may not need it.
pub fn bin2x2(img: &GrayImage) -> GrayImage {
    let _ = img;
    todo!()
}
