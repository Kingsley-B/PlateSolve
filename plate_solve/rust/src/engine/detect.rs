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

pub fn median(numbers: Vec<f64>) -> f64 {
    let sorted = numbers.sort_unstable();
    let mid = sorted.len()/2.0;
    if sorted.len()%2 == 0 {
        return (sorted[mid-1]+sorted[mid])/2;
    } else {
        return sorted[mid];
    }
}

/// Run the full detection pipeline. Returns stars brightest first.
pub fn detect_stars(img: &GrayImage, cfg: &DetectionConfig) -> Vec<DetectedStar> {
    // TODO: chain the steps below.
    let _ = (img, meta, cfg);
    todo!()
}

/// Sky brightness and noise across the image, one value per tile.
#[derive(Debug, Clone)]
pub struct BackgroundMap {
    pub tile_px: u32,
    /// Number of complete tiles across and down.
    pub tiles_x: u32,
    pub tiles_y: u32,
    /// Whether there is an extra, narrower tile at the right / bottom edge.
    pub excess_x: bool,
    pub excess_y: bool,
    /// Typical sky level per tile (median), row by row.
    pub level: Vec<f32>,
    /// Typical noise per tile (scaled MAD), row by row.
    pub noise: Vec<f32>,
}

impl BackgroundMap {
    /// (level, noise) of one tile, `w` x `h` pixels, at tile position (tile_x, tile_y).
    fn find_level(img: &GrayImage, tile_x: u32, tile_y: u32, tile_px: u32, w: u32, h: u32) -> (f32, f32) {
        let mut tile: Vec<f32> = Vec::with_capacity((w * h) as usize);
        for y in 0..h {
            for x in 0..w {
                tile.push(img.get(tile_x * tile_px + x, tile_y * tile_px + y));
            }
        }
        let level = median(&mut tile);
        let mut deviations: Vec<f32> = tile.iter().map(|p| (p - level).abs()).collect();
        (level, 1.4826 * median(&mut deviations))
    }

    /// Measure the sky in tiles, ignoring the stars themselves.
    pub fn estimate_background(img: &GrayImage, tile_px: u32) -> Self {
        let (complete_columns, final_column_size) = (img.width / tile_px, img.width % tile_px);
        let (complete_rows, final_row_size) = (img.height / tile_px, img.height % tile_px);
        let mut levels: Vec<f32> = Vec::new();
        let mut noises: Vec<f32> = Vec::new();

        // full rows
        for r in 0..complete_rows {
            for c in 0..complete_columns {
                let (level, noise) = Self::find_level(img, c, r, tile_px, tile_px, tile_px);
                levels.push(level);
                noises.push(noise);
            }
            // narrower final column
            if final_column_size > 0 {
                let (level, noise) = Self::find_level(img, complete_columns, r, tile_px, final_column_size, tile_px);
                levels.push(level);
                noises.push(noise);
            }
        }
        // shorter final row
        if final_row_size > 0 {
            for c in 0..complete_columns {
                let (level, noise) = Self::find_level(img, c, complete_rows, tile_px, tile_px, final_row_size);
                levels.push(level);
                noises.push(noise);
            }
            // bottom-right corner tile
            if final_column_size > 0 {
                let (level, noise) =
                    Self::find_level(img, complete_columns, complete_rows, tile_px, final_column_size, final_row_size);
                levels.push(level);
                noises.push(noise);
            }
        }

        Self {
            tile_px,
            tiles_x: complete_columns,
            tiles_y: complete_rows,
            excess_x: final_column_size > 0,
            excess_y: final_row_size > 0,
            level: levels,
            noise: noises,
        }
    }

    /// Position in `level` / `noise` of the tile containing pixel (x, y).
    pub fn index(&self, x: u32, y: u32) -> usize {
        let row = y / self.tile_px;
        let column = x / self.tile_px;
        let columns_per_row = self.tiles_x + self.excess_x as u32;
        (row * columns_per_row + column) as usize
    }

    /// (level, noise) of the tile containing pixel (x, y).
    pub fn at(&self, x: u32, y: u32) -> (f32, f32) {
        let i = self.index(x, y);
        (self.level[i], self.noise[i])
    }

    /// true for pixels brighter than background + sigma * noise.
    pub fn threshold(&self, img: &GrayImage,sigma: f32) -> Vec<bool> {
        let mut thresh: Vec<bool> = Vec::new();
        for y in 0..img.height {
            for x in 0..img.width {
                let (back,noise) = self.at(x,y);
                thresh.push(back+sigma*noise<img.get(x,y));
            }
        }
        thresh
    }
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
