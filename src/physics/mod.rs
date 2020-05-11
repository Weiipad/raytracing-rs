pub mod material;

use material::Material;

use std::{
    ops::{
        Range,
    },
    sync::Arc,
};

use crate::rmath::Vector3;

pub struct Ray {
    origin: Vector3,
    direction: Vector3
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self {
            origin, direction
        }
    }

    #[allow(dead_code)]
    pub fn from_two_points(p1: Vector3, p2: Vector3) -> Self {
        Self {
            origin: p1,
            direction: p2 - p1
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

#[derive(Clone)]
pub struct HitRecord {
    pub p: Vector3,
    pub t: f64,
    pub front_face: bool,
    pub normal: Vector3,
    pub mat_ptr: Arc<dyn Material>,
}

impl HitRecord {
    #[inline]
    pub fn get_face_normal(r: &Ray, outward_normal: Vector3) -> (bool, Vector3) {
        let front_face = r.get_direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: vec![]
        }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj)
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_range: Range<f64>) -> Option<HitRecord> {
        let mut rec = None;
        let mut closet_so_far = t_range.end;
        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_range.start..closet_so_far) {
                closet_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}