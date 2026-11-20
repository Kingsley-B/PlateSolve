//! Loading photos: JPEG decode, conversion to brightness, EXIF.
//!
//! Suggested crates (add to Cargo.toml):
//!   image        - JPEG decoding
//!   kamadak-exif - EXIF reading (imported as `exif`)

use std::path::Path;

use super::types::{GrayImage, ImageMeta, SolveError};

/// Read and decode a JPEG file.
pub fn load_jpeg(path: &Path) -> Result<(GrayImage, ImageMeta), SolveError> {
    // TODO: std::fs::read, then decode_jpeg.
    let _ = path;
    todo!()
}

/// Decode JPEG bytes into a brightness image plus metadata.
pub fn decode_jpeg(bytes: &[u8]) -> Result<(GrayImage, ImageMeta), SolveError> {
    // TODO:
    // 1. decode to 8-bit RGB
    // 2. rgb_to_linear_luminance
    // 3. read_exif (missing EXIF is fine - fields stay None)
    let _ = bytes;
    todo!()
}

/// Undo the camera's sRGB tone curve and combine channels into brightness.
/// Linear values make star fluxes comparable (bright vs faint).
pub fn rgb_to_linear_luminance(rgb: &[u8], width: u32, height: u32) -> GrayImage {
    let _ = (rgb, width, height);
    todo!()
}

/// Pull make, model, focal length and capture time from EXIF.
pub fn read_exif(bytes: &[u8]) -> ImageMeta {
    let _ = bytes;
    todo!()
}

/// Average 2x2 blocks into one pixel. Optional speed-up for full-resolution
/// (6240 px) files; the phone-transfer size (~3000 px) may not need it.
pub fn bin2x2(img: &GrayImage) -> GrayImage {
    let _ = img;
    todo!()
}
