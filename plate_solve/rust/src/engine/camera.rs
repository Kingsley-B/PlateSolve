//! Camera and lens models: turning focal length + sensor size into field of
//! view and pixel scale, and the per-lens settings for your two lenses.

use std::path::PathBuf;

/// Fujifilm X-T30 APS-C sensor size in millimetres.
pub const XT30_SENSOR_W_MM: f64 = 23.5;
pub const XT30_SENSOR_H_MM: f64 = 15.6;

/// The geometry of one photo.
#[derive(Debug, Clone, Copy)]
pub struct CameraModel {
    pub sensor_w_mm: f64,
    pub sensor_h_mm: f64,
    pub focal_length_mm: f64,
    /// Size of the image actually being solved (after any resizing/binning).
    pub width_px: u32,
    pub height_px: u32,
}

impl CameraModel {
    /// An X-T30 frame of the given pixel size taken at `focal_length_mm`.
    pub fn xt30(focal_length_mm: f64, width_px: u32, height_px: u32) -> Self {
        let _ = (focal_length_mm, width_px, height_px);
        todo!()
    }

    /// Focal length expressed in pixels (the key number for projection maths).
    pub fn focal_length_px(&self) -> f64 {
        todo!()
    }

    /// (horizontal, vertical) field of view in degrees.
    pub fn fov_deg(&self) -> (f64, f64) {
        todo!()
    }

    /// Arcseconds of sky per pixel at the image centre.
    pub fn pixel_scale_arcsec(&self) -> f64 {
        todo!()
    }
}


/// Settings for one lens, including which solver database to use.
/// You set your zooms to 16 mm and 50 mm, so there are two profiles.
#[derive(Debug, Clone)]
pub struct LensProfile {
    pub name: String,
    pub focal_length_mm: f64,
    /// Radial distortion (same model as make_starfield.py). Start with the
    /// simulator's values; later measure your real lenses from solved frames.
    pub distortion_k1: f64,
    /// Accept EXIF focal lengths within this many mm as "this lens".
    pub focal_tolerance_mm: f64,
    /// Solver database built for this lens (see database.rs).
    pub database_path: PathBuf,
}

/// The built-in profiles: 16 mm and 50 mm.
pub fn lens_profiles(database_dir: &std::path::Path) -> Vec<LensProfile> {
    // TODO: two LensProfile values; database files e.g. "solver_16mm.bin",
    //       "solver_50mm.bin" inside database_dir.
    let _ = database_dir;
    todo!()
}

/// Pick the profile matching an EXIF focal length (None if no match).
pub fn profile_for_focal_length(profiles: &[LensProfile], focal_mm: f64) -> Option<&LensProfile> {
    let _ = (profiles, focal_mm);
    todo!()
}
