//! Sky and image geometry. Conventions match tools/make_starfield.py exactly,
//! so results can be compared with the truth files without fudge factors.

use super::camera::CameraModel;

/// A position on the sky (J2000, degrees). Used for stars, targets, and
/// the centre of a pointing.
/// /// A 3D vector (usually a unit direction on the sky).
pub type Vec3 = (f64, f64, f64);
pub type Vec2 = (f64,f64);
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
    pub fn glob_NE(&self) -> (Vec3, Vec3) {
        ((-self.dec_deg.to_radians().sin() * self.ra_deg.to_radians().cos(), -self.dec_deg.to_radians().sin() * self.ra_deg.to_radians().sin(), self.dec_deg.to_radians().cos()), (-self.ra_deg.to_radians().sin(),self.ra_deg.to_radians().cos(),0))
    }
}

/// Where a camera points: the sky position at the image centre, plus roll.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pointing {
    pub centre: SkyCoord,
    pub roll_deg: f64,
}

impl Pointing {
    pub fn new(ra_deg: f64, dec_deg: f64, roll_deg: f64) -> Self {
        Self { centre: SkyCoord::new(ra_deg, dec_deg), roll_deg }
    }

    pub fn from_sky_coord(sky_coord:SkyCoord, roll_deg: f64) -> Self {
        Self { centre: sky_coord, roll_deg }
    }

    fn find_roll(sky_coord:SkyCoord,image_right:Vec3) -> f64 {
        let (n,e) = sky_coord.glob_NE();
        let (n0,n1,n2) = n;
        let (e0,e1,e2) = e;
        let (i0,i1,i2) = image_right;
        let rn = i0*n0;
        rn.atan2(-i0*e0)
    }

    pub fn from_camera_basis(camera_basis: (Vec3, Vec3, Vec3)) -> Self {
        let (b,r,u) = camera_basis;
        let sky_coord = SkyCoord::from_vec(b);
        let x = find_roll(sky_coord,r);
        Self {centre:sky_coord,roll_deg:x}
    }

    /// Unit vector along the camera's line of sight.
    pub fn boresight(&self) -> Vec3 {
        self.centre.to_vec()
    }

    fn image_right_comp(&self,n,e) -> f64 :{
        -self.roll_deg.to_radians().cos() * e + self.roll_deg.to_radians().sin() * n
    }

    pub fn image_right(&self) ->Vec3 {
        let (n,e) = self.sky_coord.glob_NE();
        let (n0,n1,n2) = n;
        let (e0,e1,e2) = e;
        (self.image_right_comp(n0,e0),self.image_right_comp(n1,e1),self.image_right_comp(n2,e2))
    }

    fn image_up_comp(&self,n,e) -> f64 :{
        self.roll_deg.to_radians().cos() * n + self.roll_deg.to_radians().sin() * e
    }

    pub fn image_up(&self) -> Vec3 {
        let (n,e) = self.sky_coord.glob_NE();
        let (n0,n1,n2) = n;
        let (e0,e1,e2) = e;
        (self.image_up_comp(n0,e0),self.image_up_comp(n1,e1),self.image_up_comp(n2,e2))
    }

    pub fn camera_basis(&self) -> (Vec3, Vec3, Vec3) {
        (self.boresight(),self.image_right(),self.image_up())
    }
}

/// Difference between two angles wrapped into -180..180.
pub fn wrap_angle_deg(ab:Vec2) -> f64 {
    let (mut a,mut b) = ab;
    let mut dir = 1.0;
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

/// Sky position -> pixel (with distortion). For overlays and verification.
pub fn radec_to_pixel(p: &Pointing, cam: &CameraModel, k1: f64, ra_deg: f64, dec_deg: f64) -> Option<(f64, f64)> {
    let _ = (p, cam, k1, ra_deg, dec_deg);
    todo!()
}

/// Pixel -> sky position (removing distortion).
pub fn pixel_to_radec(p: &Pointing, cam: &CameraModel, k1: f64, x: f64, y: f64) -> (f64, f64) {
    let _ = (p, cam, k1, x, y);
    todo!()
}

/// Remove radial distortion from a pixel position (iterative inverse of
/// x_d = x_u * (1 + k1 * r^2), r = radius / half-diagonal).
pub fn undistort(cam: &CameraModel, k1: f64, x: f64, y: f64) -> (f64, f64) {
    let _ = (cam, k1, x, y);
    todo!()
}
