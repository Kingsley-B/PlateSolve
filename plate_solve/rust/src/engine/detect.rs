//! Star detection: find the stars in a photo and measure their centres.
//!
//! Steps:
//!   1. estimate_background  - sky level and noise, per tile (handles
//!                             light-pollution gradients and vignetting)
//!   2. threshold            - mark pixels well above the local sky
//!   3. label_blobs          - group touching marked pixels into blobs
//!   4. measure_blob         - sub-pixel centroid, flux, size
//!   5. filter + sort        - drop hot pixels, giant blobs, edge stars;
//!                             keep the brightest N
//!
//! Reference design worth reading: cedar-detect (Rust).

use super::types::{DetectedStar, DetectionConfig, GrayImage};

/// Run the full detection pipeline. Returns stars brightest first.
pub fn detect_stars(img: &GrayImage, meta: &ImageMeta, cfg: &DetectionConfig) -> Vec<DetectedStar> {
    // TODO: chain the steps below.
    let _ = (img, meta, cfg);
    todo!()
}

/// Sky brightness and noise across the image, sampled on a coarse grid.
#[derive(Debug, Clone)]
pub struct BackgroundMap {
    pub tile_px: u32,
    pub tiles_x: u32,
    pub tiles_y: u32,
    /// Typical sky level per tile (e.g. median).
    pub level: Vec<f32>,
    /// Typical noise per tile (e.g. scaled MAD - robust to stars).
    pub noise: Vec<f32>,
}

impl BackgroundMap {
    /// Measure the sky in tiles, ignoring the stars themselves.
    pub fn estimate_background(img: &GrayImage, meta: &ImageMeta, tile_px: u32) -> Self {
        let 
        
    }

    /// Smoothly interpolated (level, noise) at a pixel.
    pub fn at(&self, x: u32, y: u32) -> (f32, f32) {
        let _ = (x, y);
        todo!()
    }
}



/// true for pixels brighter than background + sigma * noise.
pub fn threshold(img: &GrayImage, bg: &BackgroundMap, sigma: f32) -> Vec<bool> {
    let _ = (img, bg, sigma);
    todo!()
}

/// A connected group of above-threshold pixels.
#[derive(Debug, Clone)]
pub struct Blob {
    /// Pixel coordinates belonging to the blob.
    pub pixels: Vec<(u32, u32)>,
    pub min_x: u32,
    pub min_y: u32,
    pub max_x: u32,
    pub max_y: u32,
}

/// Group touching pixels (8-connectivity) into blobs.
pub fn label_blobs(mask: &[bool], width: u32, height: u32) -> Vec<Blob> {
    let _ = (mask, width, height);
    todo!()
}

/// Background-subtracted, brightness-weighted centroid, flux and size.
/// Remember the +0.5 so centres match the truth-file pixel convention.
pub fn measure_blob(blob: &Blob, img: &GrayImage, bg: &BackgroundMap) -> DetectedStar {
    let _ = (blob, img, bg);
    todo!()
}

/// A single very sharp pixel with no spread is a hot pixel, not a star.
pub fn looks_like_hot_pixel(star: &DetectedStar, blob: &Blob) -> bool {
    let _ = (star, blob);
    todo!()
}
