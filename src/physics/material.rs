use crate::rmath::Vector3;
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