use std::ops::{
    Neg,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Range,
};

use rand::Rng;

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}

pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

pub fn random_interval(interval: Range<f64>) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(interval.start, interval.end)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x > max {
        max
    } else if x < min {
        min
    } else {
        x
    }
}

#[derive(Copy, Clone, Default)]
pub struct Vector3(pub f64, pub f64, pub f64);

impl Vector3 {
    pub fn new() -> Self {
        Vector3(0.0, 0.0, 0.0)
    }

    pub fn from_random() -> Self {
        Vector3(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>())
    }

    pub fn from_interval_random(i: Range<f64>) -> Self {
        Vector3(random_interval(i.clone()), random_interval(i.clone()), random_interval(i.clone()))
    }

    pub fn from_random_in_unit_sphere() -> Self {
        loop {
            let candi = Vector3::from_random();
            if candi.length_square() <= 1.0 {
                return candi
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_square(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_square())
    }

    pub fn dot(self, other: Vector3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn to_string(&self) -> String {
        format!("[{},{},{}]", self.x(), self.y(), self.z())
    }

    pub fn cos_of_angle(self, other: Vector3) -> f64 {
        self.unit().dot(other.unit())
    }

    pub fn angle(self, other: Vector3) -> f64 {
        f64::acos(self.unit().dot(other.unit()))
    }

    pub fn into_rgb(self) -> image::Rgb<u8> {
        image::Rgb([(self.0 * 256.0) as u8, (self.1 * 256.0) as u8, (self.2 * 256.0) as u8])
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, o: Vector3) {
        self.0 += o.0;
        self.1 += o.1;
        self.2 += o.2;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, o: Vector3) {
        self.0 -= o.0;
        self.1 -= o.1;
        self.2 -= o.2;
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, s: f64) {
        self.0 *= s;
        self.1 *= s;
        self.2 *= s;
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, s: f64) {
        self.0 /= s;
        self.1 /= s;
        self.2 /= s;
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, o: Vector3) -> Vector3 {
        Vector3(self.0 + o.0, self.1 + o.1, self.2 + o.2)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, o: Vector3) -> Vector3 {
        Vector3(self.0 - o.0, self.1 - o.1, self.2 - o.2)
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3(
            self.1 * rhs.2 - self.2 * rhs.1, 
            self.2 * rhs.0 - self.0 * rhs.2, 
            self.0 * rhs.1 - self.1 * rhs.0
        )
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, s: f64) -> Vector3 {
        Vector3(self.0 * s, self.1 * s, self.2 * s)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Vector3 {
        v * self
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, s: f64) -> Vector3 {
        Vector3(self.0 / s, self.1 / s, self.2 / s)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3(-self.0, -self.1, -self.2)
    }
}