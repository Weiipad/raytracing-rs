use crate::{
    physics::{
        Ray,
        Hittable,
        HitRecord
    },
    rmath::Vector3,
};

use std::ops::Range;

pub struct Sphere {
    center: Vector3,
    r: f64
}

impl Sphere {
    pub fn new(center: Vector3, r: f64) -> Self {
        Sphere { center, r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let oc = r.get_origin() - self.center;
        let a = r.get_direction().length_square();
        let h = oc.dot(r.get_direction());
        let c = oc.length_square() - self.r * self.r;
        let delta = h * h - a * c;
        if delta > 0.0 {
            let root = f64::sqrt(delta);
            let mut t = (-h - root) / a;
            if t_range.contains(&t) {
                let p = r.at(t);
                let (front_face, normal) = HitRecord::get_face_normal(r, (p - self.center) / self.r);
                let rec = HitRecord { p, t, front_face, normal };
                return Some(rec)
            }
            t = (-h + root) / a;
            if t_range.contains(&t) {
                let p = r.at(t);
                let (front_face, normal) = HitRecord::get_face_normal(r, (p - self.center) / self.r);
                let rec = HitRecord { p, t, front_face, normal };
                return Some(rec)
            }
        }
        None
    }
}

pub struct Plane {
    normal: Vector3,
    offset: f64
}

impl Plane {
    pub fn new(normal: Vector3, offset: f64) -> Self {
        Plane {
            normal, offset
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let t = (-self.offset - r.get_origin().dot(self.normal)) / r.get_direction().dot(self.normal);
        if t_range.contains(&t) {
            Some(HitRecord { p: r.at(t), t, front_face: true, normal: self.normal })
        } else {
            None
        }
    }
}