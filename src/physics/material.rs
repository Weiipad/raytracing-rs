use crate::rmath::{Vector3, random_double};
use super::*;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3 /* attenuation */, Ray /* scatter */)>;
}

pub struct Lambertian {
    albedo: Vector3
}

impl Lambertian {
    pub fn new(color: Vector3) -> Self {
        Self {
            albedo: color
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Vector3, Ray)> {
        let scatter_direction = rec.normal + Vector3::from_random_unit();
        Some((self.albedo, Ray::new(rec.p, scatter_direction)))
    }
}

pub struct Metal {
    albedo: Vector3,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Vector3, fuzz: f64) -> Self {
        Self {
            albedo: color, 
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3, Ray)> {
        let reflected = r_in.get_direction().unit().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vector3::from_random_unit());
        if scattered.get_direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f64
}

fn schlick(cos: f64, ref_idx: f64) -> f64{
    let mut r = (1.0 - ref_idx) / (1.0 + ref_idx);
    r = r * r;
    r + (1.0 - r) * f64::powi(1.0 - cos, 5)
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3, Ray)> {
        let etai_over_etat = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_dir = r_in.get_direction().unit();
        let cos_theta = f64::min(-unit_dir.dot(rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        if etai_over_etat * sin_theta > 1.0 /*|| random_double() < schlick(cos_theta, etai_over_etat)*/ {
            let reflected = unit_dir.reflect(rec.normal);
            Some((Vector3::new(1.0), Ray::new(rec.p, reflected)))
        } else {
            let refracted = unit_dir.refract(rec.normal, etai_over_etat);
            Some((Vector3::new(1.0), Ray::new(rec.p, refracted)))
        }
    }
}