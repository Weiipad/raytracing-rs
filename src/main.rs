use image;

mod rmath;
mod physics;
mod model;

use rmath::{
    Vector3,
};

use physics::{
    Ray,
    HittableList,
    HitRecord,
    Hittable,
};

use model::Sphere;

use std::{
    f64::INFINITY,
    rc::Rc
};

fn ray_color(r: &Ray, world: &HittableList) -> Vector3 {
    let mut rec: HitRecord = Default::default();
    if world.hit(r, 0.0..INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Vector3(1.0, 1.0, 1.0))
    }
    let unit_dir = r.get_direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vector3(1.0, 1.0, 1.0) + t * Vector3(0.5, 0.7, 1.0)
}


fn main() {
    let ratio = 16.0 / 9.0;
    let width = 384;
    let height = (width as f64/ ratio) as u32;

    let origin = Vector3(0.0, 0.0, 0.0);
    let horizontal = Vector3(4.0, 0.0, 0.0);
    let vertical = Vector3(0.0, 2.25, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vector3(0.0, 0.0, 1.0);

    let mut world = HittableList::new();
    world.add(Rc::from(Sphere::new(Vector3(0.0, -100.5, -1.0), 100.0)));
    world.add(Rc::from(Sphere::new(Vector3(0.0, 0.0, -1.0), 0.5)));
    

    let mut imgbuf = image::ImageBuffer::new(width, height);
    for (x, y, pix) in imgbuf.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = (height - y) as f64 / height as f64;
        let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        *pix = ray_color(&ray, &world).into_rgb();
    }

    imgbuf.save("test.png").unwrap();
}
