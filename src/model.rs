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
    fn hit(&self, r: &Ray, t_range: Range<f64>, rec: &mut HitRecord) -> bool {
        let oc = r.get_origin() - self.center;
        let a = r.get_direction().length_square();
        let h = oc.dot(r.get_direction());
        let c = oc.length_square() - self.r * self.r;
        let delta = h * h - a * c;
        if delta > 0.0 {
            let root = f64::sqrt(delta);
            let mut t = (-h - root) / a;
            if t_range.contains(&t) {
                rec.t = t;
                rec.p = r.at(t);
                rec.set_face_normal(&r, (rec.p - self.center) / self.r);
                return true
            }
            t = (-h + root) / a;
            if t_range.contains(&t) {
                rec.t = t;
                rec.p = r.at(t);
                rec.set_face_normal(&r, (rec.p - self.center) / self.r);
                return true
            }
        }
        false
    }
}