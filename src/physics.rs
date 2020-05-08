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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_range: Range<f64>, hit_record: &mut HitRecord) -> bool;
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
            if object.hit(r, t_range.start..closet_so_far, &mut temp_rec) {
                hits_anything = true;
                closet_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hits_anything
    }
}

#[derive(Clone)]
pub struct Camera {
    origin: Vector3,
    low_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3
}

impl Camera {
    pub fn new() -> Self {
        Self {
            origin: Vector3(0.0, 0.0, 0.0),
            low_left_corner: Vector3(-2.0, -1.0, -1.0),
            horizontal: Vector3(4.0, 0.0, 0.0),
            vertical: Vector3(0.0, 2.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.low_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}