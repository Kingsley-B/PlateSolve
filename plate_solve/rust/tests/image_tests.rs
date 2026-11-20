//! Tests for engine::image: JPEG loading, linear brightness, EXIF.
//!
//! The file-loading tests need the generated test images in PlateSolve/testdata
//! (run tools/make_starfield.py --batch first). The brightness tests need nothing.
//!
//! Run with:  cargo test --test image

use std::path::PathBuf;

use rust_lib_plate_solve::engine::image::{
    decode_jpeg, load_jpeg, read_exif, rgb_to_linear_luminance,
};
use rust_lib_plate_solve::engine::SolveError;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn assert_close(actual: f32, expected: f32, tol: f32, what: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{what}: expected {expected}, got {actual} (diff {:e})",
        (actual - expected).abs()
    );
}

/// Path to a generated test image. Built from this crate's folder, so it
/// works whichever directory `cargo test` is started from.
fn testdata(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../testdata")
        .join(name);
    assert!(
        path.exists(),
        "missing test image {}\nGenerate the test set first:\n  \
         python tools/make_starfield.py --catalogue hyg_v42.csv.gz --batch --k1 0 --out testdata",
        path.display()
    );
    path
}

/// Median of a list of values (sorts a copy).
fn median(values: &[f32]) -> f32 {
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

// ---------------------------------------------------------------------------
// rgb_to_linear_luminance (no files needed)
// ---------------------------------------------------------------------------

#[test]
fn luminance_black_and_white() {
    let img = rgb_to_linear_luminance(&[0, 0, 0, 255, 255, 255], 2, 1);
    assert_close(img.pixels[0], 0.0, 1e-7, "black");
    assert_close(img.pixels[1], 1.0, 1e-5, "white");
}

#[test]
fn luminance_mid_grey_is_not_half() {
    // sRGB 128 is about 21.6% of the light of 255, not 50%
    let img = rgb_to_linear_luminance(&[128, 128, 128], 1, 1);
    assert_close(img.pixels[0], 0.2159, 1e-3, "grey 128");
}

#[test]
fn luminance_dark_values_use_linear_segment() {
    // Below 0.04045 (byte 10) the sRGB curve is a straight line: c / 12.92
    let img = rgb_to_linear_luminance(&[10, 10, 10], 1, 1);
    assert_close(img.pixels[0], (10.0 / 255.0) / 12.92, 1e-6, "grey 10");
}

#[test]
fn luminance_channel_weights() {
    // Pure red, green and blue give the Rec.709 weights
    let img = rgb_to_linear_luminance(&[255, 0, 0, 0, 255, 0, 0, 0, 255], 3, 1);
    assert_close(img.pixels[0], 0.2126, 1e-5, "red");
    assert_close(img.pixels[1], 0.7152, 1e-5, "green");
    assert_close(img.pixels[2], 0.0722, 1e-5, "blue");
}

#[test]
fn luminance_increases_with_byte_value() {
    let rgb: Vec<u8> = (0..=255u8).flat_map(|v| [v, v, v]).collect();
    let img = rgb_to_linear_luminance(&rgb, 256, 1);
    for v in 1..256 {
        assert!(
            img.pixels[v] > img.pixels[v - 1],
            "not increasing at {v}: {} <= {}",
            img.pixels[v],
            img.pixels[v - 1]
        );
    }
}

#[test]
fn luminance_keeps_size_and_pixel_order() {
    // 3x2 image; pixel (x, y) must end up at index y * width + x
    let rgb: Vec<u8> = [0u8, 50, 100, 150, 200, 255]
        .iter()
        .flat_map(|&v| [v, v, v])
        .collect();
    let img = rgb_to_linear_luminance(&rgb, 3, 2);
    assert_eq!((img.width, img.height), (3, 2));
    assert_eq!(img.pixels.len(), 6);
    for i in 1..6 {
        assert!(img.pixels[i] > img.pixels[i - 1], "order wrong at index {i}");
    }
}

// ---------------------------------------------------------------------------
// decode_jpeg / read_exif error handling (no files needed)
// ---------------------------------------------------------------------------

#[test]
fn decode_rejects_non_jpeg_bytes() {
    let result = decode_jpeg(b"definitely not a jpeg");
    assert!(
        matches!(result, Err(SolveError::ImageDecode(_))),
        "expected ImageDecode error, got {:?}",
        result.map(|(img, _)| (img.width, img.height))
    );
}

#[test]
fn decode_rejects_empty_input() {
    assert!(matches!(decode_jpeg(&[]), Err(SolveError::ImageDecode(_))));
}

#[test]
fn read_exif_without_exif_gives_empty_meta() {
    // Missing EXIF is not an error: every field stays None
    let meta = read_exif(b"no exif in here");
    assert!(meta.make.is_none());
    assert!(meta.model.is_none());
    assert!(meta.focal_length_mm.is_none());
    assert!(meta.capture_time.is_none());
}

#[test]
fn load_missing_file_is_io_error() {
    let result = load_jpeg(&PathBuf::from("this/file/does/not/exist.jpg"));
    assert!(matches!(result, Err(SolveError::Io(_))));
}

// ---------------------------------------------------------------------------
// Real test images (need ../../testdata)
// ---------------------------------------------------------------------------

#[test]
fn loads_16mm_test_image() {
    let (img, meta) = load_jpeg(&testdata("16mm_carina.jpg")).unwrap();
    assert_eq!((img.width, img.height), (3000, 1991));
    assert_eq!(img.pixels.len(), 3000 * 1991);
    assert_eq!((meta.width, meta.height), (3000, 1991));
    assert_eq!(meta.focal_length_mm, Some(16.0));
}

#[test]
fn loads_50mm_test_image() {
    let (img, meta) = load_jpeg(&testdata("50mm_carina.jpg")).unwrap();
    assert_eq!((img.width, img.height), (3000, 1991));
    assert_eq!(meta.focal_length_mm, Some(50.0));
}

#[test]
fn exif_camera_fields() {
    let (_, meta) = load_jpeg(&testdata("16mm_carina.jpg")).unwrap();
    assert_eq!(meta.make.as_deref(), Some("FUJIFILM"));
    assert_eq!(meta.model.as_deref(), Some("X-T30"));
}

#[test]
fn exif_capture_time_format() {
    // EXIF dates look like "2026:11:17 09:52:06" (colons in the date part)
    let (_, meta) = load_jpeg(&testdata("16mm_carina.jpg")).unwrap();
    let t = meta.capture_time.expect("capture time missing");
    assert_eq!(t.len(), 19, "unexpected format: {t}");
    assert_eq!(&t[4..5], ":", "unexpected format: {t}");
    assert_eq!(&t[10..11], " ", "unexpected format: {t}");
    // The generator writes UTC times
    assert_eq!(meta.capture_time_offset.as_deref(), Some("+00:00"));
}

#[test]
fn brightness_values_look_like_a_night_sky() {
    let (img, _) = load_jpeg(&testdata("16mm_carina.jpg")).unwrap();
    assert!(
        img.pixels.iter().all(|&v| (0.0..=1.0).contains(&v)),
        "values outside 0..1"
    );
    // Most of the frame is dark sky; bright stars reach near full scale
    let sky = median(&img.pixels);
    let max = img.pixels.iter().cloned().fold(0.0f32, f32::max);
    assert!(sky > 0.003 && sky < 0.2, "sky level looks wrong: {sky}");
    assert!(max > 0.5, "brightest pixel too dim for a star field: {max}");
    assert!(max > 10.0 * sky, "stars barely brighter than sky: max {max}, sky {sky}");
}

#[test]
fn decode_jpeg_matches_load_jpeg() {
    let path = testdata("50mm_carina.jpg");
    let bytes = std::fs::read(&path).unwrap();
    let (a, _) = decode_jpeg(&bytes).unwrap();
    let (b, _) = load_jpeg(&path).unwrap();
    assert_eq!(a.pixels, b.pixels);
}
