//! Sky geometry.

/// A 3D vector (usually a unit direction on the sky).
pub type Vec3 = (f64, f64, f64);

/// A pair of values, e.g. two angles.
pub type Vec2 = (f64, f64);

/// A position on the sky (J2000, degrees). Used for stars, targets and
/// solved image centres.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkyCoord {
    pub ra_deg: f64,
    pub dec_deg: f64,
}

impl SkyCoord {
    pub fn new(ra_deg: f64, dec_deg: f64) -> Self {
        Self { ra_deg, dec_deg }
    }

    /// -> unit vector. x towards RA 0°/Dec 0°, y towards RA 90°, z towards north pole.
    pub fn to_vec(&self) -> Vec3 {
        let ra = self.ra_deg.to_radians();
        let dec = self.dec_deg.to_radians();
        let cos_dec = dec.cos();
        (cos_dec * ra.cos(), cos_dec * ra.sin(), dec.sin())
    }

    /// Unit vector -> SkyCoord (RA wrapped to 0..360).
    pub fn from_vec(v: Vec3) -> Self {
        let (x,y,z) = v;
        let ra = y.atan2(x).to_degrees().rem_euclid(360.0);
        let dec = z.clamp(-1.0, 1.0).asin().to_degrees();
        Self::new(ra, dec)
    }

    /// Angle between two sky positions, in degrees.
    pub fn separation_deg(&self, other: &SkyCoord) -> f64 {
        let (x0,y0,z0) = self.to_vec();
        let (x1,y1,z1) = other.to_vec();
        let dot = x0*x1 + y0*y1 + z0*z1;
        dot.clamp(-1.0, 1.0).acos().to_degrees()
    }
    pub fn glob_ne(&self) -> (Vec3, Vec3) {
        let decsin = self.dec_deg.to_radians().sin();
        let deccos = self.dec_deg.to_radians().cos();
        let rasin = self.ra_deg.to_radians().sin();
        let racos = self.ra_deg.to_radians().cos();

        ((-decsin * racos, -decsin * rasin, deccos), (-rasin,racos,0.0))
    }
}



/// Difference between two angles wrapped into -180..180.
pub fn wrap_angle_deg(ab:Vec2) -> f64 {
    let (mut a,mut b) = ab;
    let mut dir = 1.0;
    a = a.rem_euclid(360.0);
    b = b.rem_euclid(360.0);
    if b > a {
        dir = -1.0;
        let c = b;
        b = a;
        a = c;
    }
    let n = a-b;
    let u = 360.0 + b - a;
    if n > u {
        return -dir*u;
    }
    return dir*n;
}

