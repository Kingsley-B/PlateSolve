//! Camera and lens models: turning focal length + sensor size into field of
//! view and pixel scale, and the per-lens settings for your two lenses.

use std::path::PathBuf;

/// Fujifilm X-T30 APS-C sensor size in millimetres.
pub const XT30_SENSOR_W_MM: f64 = 23.5;
pub const XT30_SENSOR_H_MM: f64 = 15.6;

const LENSES: [f64; 2] = [16.0, 50.0];

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
        Self {sensor_w_mm:XT30_SENSOR_W_MM,sensor_h_mm:XT30_SENSOR_H_MM,focal_length_mm,width_px,height_px}
    }

    /// Focal length expressed in pixels (the key number for projection maths).
    pub fn focal_length_px(&self) -> f64 {
        let pixel_size = self.sensor_w_mm / self.width_px as f64;
        self.focal_length_mm/pixel_size
    }

    /// (horizontal, vertical) field of view in degrees.
    pub fn fov_deg(&self) -> (f64, f64) {
        (2.0*self.sensor_w_mm.atan2(2.0*self.focal_length_mm).to_degrees(),2.0*self.sensor_h_mm.atan2(2.0*self.focal_length_mm).to_degrees())
    }

    /// Arcseconds of sky per pixel at the image centre.
    pub fn pixel_scale_arcsec(&self) -> f64 {
    (1.0 / self.focal_length_px()).atan().to_degrees() * 3600.0
    }
}


/// Settings for one lens, including which solver database to use.
/// You set your zooms to 16 mm and 50 mm, so there are two profiles.
#[derive(Debug, Clone)]
pub struct LensProfile {
    pub name: String,
    pub focal_length_mm: f64,
    /// Solver database built for this lens (see database.rs).
    pub database_path: PathBuf,
}

pub fn lens_profiles(database_dir: &std::path::Path) -> Vec<LensProfile> {
    let mut profiles = Vec::new();
    for &focal in &LENSES {
        profiles.push(LensProfile {
            name: format!("{focal} mm"),
            focal_length_mm: focal,
            database_path: database_dir.join(format!("solver_{focal}mm.bin")),
        });
    }
    profiles
}

/// Pick the profile matching an EXIF focal length (None if no match).
pub fn profile_for_focal_length(profiles: &[LensProfile], focal_mm: f64) -> Option<&LensProfile> {
    let mut profile: Option<&LensProfile> = None;
    let mut diff: f64 = 100000.0;
    //find closest focal length profile
    for prof in profiles {
        let d = (focal_mm-prof.focal_length_mm).abs();
        if d < diff {
            profile = Some(prof);
            diff = d;
        }
    }
    return profile;
}
