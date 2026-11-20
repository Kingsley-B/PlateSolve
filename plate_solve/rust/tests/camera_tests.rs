//! Tests for engine::camera: the X-T30 camera model and lens profiles.
//!
//! Expected values come from the truth files written by make_starfield.py
//! (3000 px wide images).
//!
//! Run with:  cargo test --test camera

use std::path::Path;

use rust_lib_plate_solve::engine::camera::{
    lens_profiles, profile_for_focal_length, CameraModel, LensProfile, XT30_SENSOR_H_MM,
    XT30_SENSOR_W_MM,
};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn assert_close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{what}: expected {expected}, got {actual} (diff {:e})",
        (actual - expected).abs()
    );
}

/// The image size the test images (and phone transfers) use.
const W: u32 = 3000;
const H: u32 = 1991;

fn cam16() -> CameraModel {
    CameraModel::xt30(16.0, W, H)
}

fn cam50() -> CameraModel {
    CameraModel::xt30(50.0, W, H)
}

// ---------------------------------------------------------------------------
// CameraModel::xt30
// ---------------------------------------------------------------------------

#[test]
fn xt30_fills_in_sensor_and_image_size() {
    let c = cam16();
    assert_eq!(c.sensor_w_mm, XT30_SENSOR_W_MM);
    assert_eq!(c.sensor_h_mm, XT30_SENSOR_H_MM);
    assert_eq!(c.focal_length_mm, 16.0);
    assert_eq!((c.width_px, c.height_px), (W, H));
}

#[test]
fn sensor_is_aps_c_3_by_2() {
    assert_close(XT30_SENSOR_W_MM, 23.5, 1e-12, "sensor width");
    assert_close(XT30_SENSOR_H_MM, 15.6, 1e-12, "sensor height");
    assert_close(XT30_SENSOR_W_MM / XT30_SENSOR_H_MM, 1.5, 0.01, "aspect ratio");
}

// ---------------------------------------------------------------------------
// focal_length_px
// ---------------------------------------------------------------------------

#[test]
fn focal_length_px_matches_truth_files() {
    assert_close(cam16().focal_length_px(), 2042.553, 0.01, "16 mm");
    assert_close(cam50().focal_length_px(), 6382.979, 0.01, "50 mm");
}

#[test]
fn focal_length_px_scales_with_image_width() {
    // Halving the image size (e.g. 2x2 binning) halves the focal length in pixels
    let full = CameraModel::xt30(16.0, 6000, 3982).focal_length_px();
    let half = CameraModel::xt30(16.0, 3000, 1991).focal_length_px();
    assert_close(full / half, 2.0, 1e-12, "full / half");
}

#[test]
fn focal_length_px_proportional_to_focal_length() {
    assert_close(cam50().focal_length_px() / cam16().focal_length_px(), 50.0 / 16.0, 1e-12, "50/16");
}

// ---------------------------------------------------------------------------
// fov_deg
// ---------------------------------------------------------------------------

#[test]
fn fov_matches_truth_files() {
    // Vertical tolerance is looser: the generator uses the whole-pixel image
    // height (1991 px) rather than the exact 15.6 mm sensor height.
    let (w, h) = cam16().fov_deg();
    assert_close(w, 72.59, 0.01, "16 mm horizontal");
    assert_close(h, 51.97, 0.02, "16 mm vertical");

    let (w, h) = cam50().fov_deg();
    assert_close(w, 26.45, 0.01, "50 mm horizontal");
    assert_close(h, 17.73, 0.02, "50 mm vertical");
}

#[test]
fn fov_does_not_depend_on_image_size() {
    // Resizing the photo doesn't change how much sky it covers
    let (a_w, a_h) = CameraModel::xt30(16.0, 6240, 4160).fov_deg();
    let (b_w, b_h) = CameraModel::xt30(16.0, 1500, 996).fov_deg();
    assert_close(a_w, b_w, 1e-12, "horizontal");
    assert_close(a_h, b_h, 1e-12, "vertical");
}

#[test]
fn fov_agrees_with_focal_length_px() {
    // Horizontal FOV computed from pixels must match the mm-based formula
    for c in [cam16(), cam50()] {
        let from_px = 2.0 * (c.width_px as f64 / 2.0).atan2(c.focal_length_px()).to_degrees();
        assert_close(c.fov_deg().0, from_px, 1e-9, "fov from pixels");
    }
}

#[test]
fn fov_shrinks_as_focal_length_grows() {
    let mut last = f64::INFINITY;
    for f in [8.0, 16.0, 23.0, 35.0, 50.0, 200.0] {
        let (w, _) = CameraModel::xt30(f, W, H).fov_deg();
        assert!(w < last, "FOV at {f} mm ({w}) not smaller than previous ({last})");
        assert!(w > 0.0 && w < 180.0, "FOV at {f} mm out of range: {w}");
        last = w;
    }
}

// ---------------------------------------------------------------------------
// pixel_scale_arcsec
// ---------------------------------------------------------------------------

#[test]
fn pixel_scale_matches_truth_files() {
    assert_close(cam16().pixel_scale_arcsec(), 100.98, 0.01, "16 mm");
    assert_close(cam50().pixel_scale_arcsec(), 32.31, 0.01, "50 mm");
}

#[test]
fn centre_pixel_scale_is_larger_than_average() {
    // Gnomonic lens: centre pixels cover more sky than edge pixels, so the
    // centre scale is bigger than (FOV / pixel count).
    let c = cam16();
    let average = c.fov_deg().0 * 3600.0 / c.width_px as f64;
    assert!(
        c.pixel_scale_arcsec() > average,
        "centre {} should exceed average {}",
        c.pixel_scale_arcsec(),
        average
    );
}

// ---------------------------------------------------------------------------
// lens_profiles
// ---------------------------------------------------------------------------

#[test]
fn two_profiles_16_and_50() {
    let profiles = lens_profiles(Path::new("assets/solver"));
    assert_eq!(profiles.len(), 2);
    let focals: Vec<f64> = profiles.iter().map(|p| p.focal_length_mm).collect();
    assert!(focals.contains(&16.0), "missing 16 mm: {focals:?}");
    assert!(focals.contains(&50.0), "missing 50 mm: {focals:?}");
}

#[test]
fn profile_names_and_database_paths() {
    let dir = Path::new("assets/solver");
    let profiles = lens_profiles(dir);
    let p16 = find(&profiles, 16.0);
    let p50 = find(&profiles, 50.0);

    assert_eq!(p16.name, "16 mm");
    assert_eq!(p50.name, "50 mm");
    assert_eq!(p16.database_path, dir.join("solver_16mm.bin"));
    assert_eq!(p50.database_path, dir.join("solver_50mm.bin"));
}

fn find(profiles: &[LensProfile], focal: f64) -> &LensProfile {
    profiles
        .iter()
        .find(|p| p.focal_length_mm == focal)
        .unwrap_or_else(|| panic!("no {focal} mm profile"))
}

// ---------------------------------------------------------------------------
// profile_for_focal_length
// ---------------------------------------------------------------------------

fn pick(focal: f64) -> Option<f64> {
    let profiles = lens_profiles(Path::new("assets/solver"));
    profile_for_focal_length(&profiles, focal).map(|p| p.focal_length_mm)
}

#[test]
fn exact_focal_lengths() {
    assert_eq!(pick(16.0), Some(16.0));
    assert_eq!(pick(50.0), Some(50.0));
}

#[test]
fn nearby_focal_lengths_pick_closest() {
    // Zoom ring slop and EXIF rounding
    assert_eq!(pick(15.6), Some(16.0));
    assert_eq!(pick(16.4), Some(16.0));
    assert_eq!(pick(48.5), Some(50.0));
    assert_eq!(pick(52.0), Some(50.0));
}

#[test]
fn far_focal_lengths_still_pick_closest() {
    // No tolerance by design: always returns the nearest lens
    assert_eq!(pick(23.0), Some(16.0));
    assert_eq!(pick(32.0), Some(16.0));
    assert_eq!(pick(34.0), Some(50.0));
    assert_eq!(pick(200.0), Some(50.0));
    assert_eq!(pick(8.0), Some(16.0));
}

#[test]
fn no_profiles_gives_none() {
    let empty: Vec<LensProfile> = Vec::new();
    assert!(profile_for_focal_length(&empty, 16.0).is_none());
}
