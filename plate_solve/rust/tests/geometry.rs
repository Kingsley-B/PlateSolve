//! Tests for engine::geometry: SkyCoord and wrap_angle_deg.
//!
//! Run with:  cargo test --test geometry

use rust_lib_plate_solve::engine::geometry::{wrap_angle_deg, SkyCoord, Vec3};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Floating-point results are never exact; compare within a tolerance.
const EPS: f64 = 1e-9;

fn assert_close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tol,
        "{what}: expected {expected}, got {actual} (diff {:e})",
        (actual - expected).abs()
    );
}

fn assert_vec_close(actual: Vec3, expected: Vec3, tol: f64, what: &str) {
    assert_close(actual.0, expected.0, tol, &format!("{what} x"));
    assert_close(actual.1, expected.1, tol, &format!("{what} y"));
    assert_close(actual.2, expected.2, tol, &format!("{what} z"));
}

fn dot(a: Vec3, b: Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn length(v: Vec3) -> f64 {
    dot(v, v).sqrt()
}

/// A spread of positions across the whole sky, including awkward ones:
/// RA near 0/360, both hemispheres, close to (but not on) the poles.
fn sample_coords() -> Vec<SkyCoord> {
    let mut out = Vec::new();
    for &ra in &[0.0, 0.001, 45.0, 90.0, 123.4, 180.0, 270.0, 359.999] {
        for &dec in &[-89.9, -72.08, -45.0, -5.39, 0.0, 30.0, 60.0, 89.9] {
            out.push(SkyCoord::new(ra, dec));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// to_vec
// ---------------------------------------------------------------------------

#[test]
fn to_vec_known_directions() {
    assert_vec_close(
        SkyCoord::new(0.0, 0.0).to_vec(),
        (1.0, 0.0, 0.0),
        EPS,
        "RA 0, Dec 0",
    );
    assert_vec_close(
        SkyCoord::new(90.0, 0.0).to_vec(),
        (0.0, 1.0, 0.0),
        EPS,
        "RA 90, Dec 0",
    );
    assert_vec_close(
        SkyCoord::new(180.0, 0.0).to_vec(),
        (-1.0, 0.0, 0.0),
        EPS,
        "RA 180, Dec 0",
    );
    assert_vec_close(
        SkyCoord::new(270.0, 0.0).to_vec(),
        (0.0, -1.0, 0.0),
        EPS,
        "RA 270, Dec 0",
    );
    // Poles: RA doesn't matter
    for &ra in &[0.0, 77.0, 300.0] {
        assert_vec_close(
            SkyCoord::new(ra, 90.0).to_vec(),
            (0.0, 0.0, 1.0),
            EPS,
            "north pole",
        );
        assert_vec_close(
            SkyCoord::new(ra, -90.0).to_vec(),
            (0.0, 0.0, -1.0),
            EPS,
            "south pole",
        );
    }
}

#[test]
fn to_vec_is_unit_length() {
    for c in sample_coords() {
        assert_close(length(c.to_vec()), 1.0, 1e-12, &format!("|{c:?}|"));
    }
}

#[test]
fn to_vec_ra_360_equals_ra_0() {
    assert_vec_close(
        SkyCoord::new(360.0, -30.0).to_vec(),
        SkyCoord::new(0.0, -30.0).to_vec(),
        EPS,
        "RA 360 vs 0",
    );
}

// ---------------------------------------------------------------------------
// from_vec
// ---------------------------------------------------------------------------

#[test]
fn from_vec_known_directions() {
    let c = SkyCoord::from_vec((0.0, 1.0, 0.0));
    assert_close(c.ra_deg, 90.0, EPS, "RA");
    assert_close(c.dec_deg, 0.0, EPS, "Dec");

    // Negative y must give RA in 180..360, not a negative RA
    let c = SkyCoord::from_vec((0.0, -1.0, 0.0));
    assert_close(c.ra_deg, 270.0, EPS, "RA (negative y)");

    // Third quadrant: x<0, y<0 -> RA 225 (plain atan would give 45)
    let h = 0.5_f64.sqrt();
    let c = SkyCoord::from_vec((-h, -h, 0.0));
    assert_close(c.ra_deg, 225.0, EPS, "RA (third quadrant)");

    let c = SkyCoord::from_vec((0.0, 0.0, -1.0));
    assert_close(c.dec_deg, -90.0, EPS, "Dec (south pole)");
}

#[test]
fn from_vec_ra_is_always_0_to_360() {
    for c in sample_coords() {
        let back = SkyCoord::from_vec(c.to_vec());
        assert!(
            (0.0..360.0).contains(&back.ra_deg),
            "RA out of range: {} from {c:?}",
            back.ra_deg
        );
    }
}

#[test]
fn from_vec_survives_slightly_too_long_vectors() {
    // Rounding can make z a hair above 1; asin would return NaN without the clamp.
    let c = SkyCoord::from_vec((0.0, 0.0, 1.0 + 1e-15));
    assert!(!c.dec_deg.is_nan(), "Dec is NaN");
    assert_close(c.dec_deg, 90.0, 1e-6, "Dec");
}

#[test]
fn radec_vector_round_trip() {
    for c in sample_coords() {
        let back = SkyCoord::from_vec(c.to_vec());
        assert_close(back.dec_deg, c.dec_deg, EPS, &format!("Dec of {c:?}"));
        // Compare RA as a wrapped difference: 359.999 and -0.001 are the same
        assert_close(
            wrap_angle_deg((back.ra_deg, c.ra_deg)),
            0.0,
            1e-7,
            &format!("RA of {c:?}"),
        );
    }
}

// ---------------------------------------------------------------------------
// separation_deg
// ---------------------------------------------------------------------------

#[test]
fn separation_known_values() {
    let origin = SkyCoord::new(0.0, 0.0);
    assert_close(
        origin.separation_deg(&SkyCoord::new(90.0, 0.0)),
        90.0,
        EPS,
        "quarter turn",
    );
    assert_close(
        origin.separation_deg(&SkyCoord::new(180.0, 0.0)),
        180.0,
        EPS,
        "opposite",
    );
    assert_close(
        origin.separation_deg(&SkyCoord::new(0.0, 90.0)),
        90.0,
        EPS,
        "to pole",
    );
    assert_close(
        SkyCoord::new(0.0, 90.0).separation_deg(&SkyCoord::new(0.0, -90.0)),
        180.0,
        EPS,
        "pole to pole",
    );
}

#[test]
fn separation_of_identical_points_is_zero() {
    for c in sample_coords() {
        let s = c.separation_deg(&c);
        assert!(!s.is_nan(), "NaN for {c:?}");
        assert_close(s, 0.0, 1e-5, &format!("self-separation of {c:?}"));
    }
}

#[test]
fn separation_across_ra_wraparound() {
    // Plain RA subtraction would say 358 degrees
    let a = SkyCoord::new(359.0, 0.0);
    let b = SkyCoord::new(1.0, 0.0);
    assert_close(a.separation_deg(&b), 2.0, 1e-9, "359 vs 1");
}

#[test]
fn separation_near_the_pole() {
    // Plain RA subtraction would say 180 degrees
    let a = SkyCoord::new(0.0, -89.0);
    let b = SkyCoord::new(180.0, -89.0);
    assert_close(
        a.separation_deg(&b),
        2.0,
        1e-9,
        "either side of the south pole",
    );
}

#[test]
fn separation_shrinks_with_cos_dec() {
    // 1 degree of RA at Dec -60 is about half a degree on the sky
    let a = SkyCoord::new(100.0, -60.0);
    let b = SkyCoord::new(101.0, -60.0);
    assert_close(a.separation_deg(&b), 0.5, 1e-4, "1 deg RA at Dec -60");
}

#[test]
fn separation_is_symmetric() {
    let coords = sample_coords();
    for a in &coords {
        for b in coords.iter().step_by(7) {
            assert_close(a.separation_deg(b), b.separation_deg(a), 1e-12, "symmetry");
        }
    }
}

// ---------------------------------------------------------------------------
// glob_ne (local north and east)
// ---------------------------------------------------------------------------

#[test]
fn north_east_are_unit_and_perpendicular() {
    for c in sample_coords() {
        let (n, e) = c.glob_ne();
        let v = c.to_vec();
        assert_close(length(n), 1.0, 1e-12, &format!("|north| at {c:?}"));
        assert_close(length(e), 1.0, 1e-12, &format!("|east| at {c:?}"));
        assert_close(dot(n, e), 0.0, 1e-12, &format!("north·east at {c:?}"));
        // Both lie flat against the sky sphere at this point
        assert_close(dot(n, v), 0.0, 1e-12, &format!("north·position at {c:?}"));
        assert_close(dot(e, v), 0.0, 1e-12, &format!("east·position at {c:?}"));
    }
}

#[test]
fn north_points_towards_increasing_dec() {
    // Step a tiny amount north; the step direction should match `north`.
    let step = 1e-4;
    for c in sample_coords()
        .into_iter()
        .filter(|c| c.dec_deg.abs() < 89.0)
    {
        let (n, _) = c.glob_ne();
        let a = c.to_vec();
        let b = SkyCoord::new(c.ra_deg, c.dec_deg + step).to_vec();
        let d = (b.0 - a.0, b.1 - a.1, b.2 - a.2);
        assert!(dot(d, n) > 0.0, "north points the wrong way at {c:?}");
    }
}

#[test]
fn east_points_towards_increasing_ra() {
    let step = 1e-4;
    for c in sample_coords()
        .into_iter()
        .filter(|c| c.dec_deg.abs() < 89.0)
    {
        let (_, e) = c.glob_ne();
        let a = c.to_vec();
        let b = SkyCoord::new(c.ra_deg + step, c.dec_deg).to_vec();
        let d = (b.0 - a.0, b.1 - a.1, b.2 - a.2);
        assert!(dot(d, e) > 0.0, "east points the wrong way at {c:?}");
    }
}

#[test]
fn north_east_known_values() {
    // At RA 0, Dec 0: east is +y, north is +z
    let (n, e) = SkyCoord::new(0.0, 0.0).glob_ne();
    assert_vec_close(n, (0.0, 0.0, 1.0), EPS, "north at origin");
    assert_vec_close(e, (0.0, 1.0, 0.0), EPS, "east at origin");
}

// ---------------------------------------------------------------------------
// wrap_angle_deg
// ---------------------------------------------------------------------------

#[test]
fn wrap_simple_differences() {
    assert_close(wrap_angle_deg((30.0, 10.0)), 20.0, EPS, "30 - 10");
    assert_close(wrap_angle_deg((10.0, 30.0)), -20.0, EPS, "10 - 30");
    assert_close(wrap_angle_deg((5.0, 5.0)), 0.0, EPS, "equal");
}

#[test]
fn wrap_takes_the_short_way_round() {
    assert_close(wrap_angle_deg((10.0, 350.0)), 20.0, EPS, "10 - 350");
    assert_close(wrap_angle_deg((350.0, 10.0)), -20.0, EPS, "350 - 10");
    assert_close(wrap_angle_deg((0.0, 359.0)), 1.0, EPS, "0 - 359");
}

#[test]
fn wrap_accepts_unnormalised_inputs() {
    assert_close(wrap_angle_deg((-170.0, 170.0)), 20.0, EPS, "-170 - 170");
    assert_close(wrap_angle_deg((720.0, 10.0)), -10.0, EPS, "720 - 10");
    assert_close(wrap_angle_deg((-10.0, 10.0)), -20.0, EPS, "-10 - 10");
    assert_close(wrap_angle_deg((370.0, -350.0)), 0.0, EPS, "370 - (-350)");
}

#[test]
fn wrap_result_always_within_half_turn() {
    let angles = [
        -725.0, -360.0, -181.0, -90.0, -0.5, 0.0, 0.5, 90.0, 179.9, 180.1, 359.0, 540.0,
    ];
    for &a in &angles {
        for &b in &angles {
            let w = wrap_angle_deg((a, b));
            assert!(
                (-180.0..=180.0).contains(&w),
                "wrap({a}, {b}) = {w} out of range"
            );
        }
    }
}

#[test]
fn wrap_is_antisymmetric() {
    // Swapping the inputs flips the sign (except at exactly ±180, either is fine)
    let angles = [-170.0, -45.0, 0.0, 10.0, 123.4, 200.0, 350.0];
    for &a in &angles {
        for &b in &angles {
            let fwd = wrap_angle_deg((a, b));
            let back = wrap_angle_deg((b, a));
            if fwd.abs() < 179.999 {
                assert_close(fwd, -back, EPS, &format!("wrap({a},{b}) vs wrap({b},{a})"));
            }
        }
    }
}

#[test]
fn wrap_matches_true_difference() {
    // Adding the wrapped difference to b must land on a (mod 360)
    let angles: [f64; 7] = [-170.0, -45.0, 0.0, 10.0, 123.4, 200.0, 350.0];
    for &a in &angles {
        for &b in &angles {
            let w = wrap_angle_deg((a, b));
            let landed = (b + w).rem_euclid(360.0);
            let target = f64::rem_euclid(a, 360.0);
            assert_close(
                wrap_angle_deg((landed, target)),
                0.0,
                1e-9,
                &format!("b + wrap(a,b) = a for ({a},{b})"),
            );
        }
    }
}
