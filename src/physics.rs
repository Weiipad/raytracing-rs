use std::{
    ops::{
        Range,
    },
    rc::Rc,
};

use crate::rmath::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin, direction
        }
    }

    pub fn get_origin(&self) -> Vector3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vector3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + t * self.direction
    }
}

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vector3) {
        self.front_face = r.get_direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_range: Range<f64>, hit_record: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_range: Range<f64>, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = Default::default();
        let mut hits_anything = false;
        let mut closet_so_far = t_range.end;
        for object in &self.objects {
            if object.hit(r, t_range.start..t_range.end, &mut temp_rec) {
                hits_anything = true;
                closet_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hits_anything
    }
}