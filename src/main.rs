use image;

mod rmath;
mod physics;
mod model;

use rmath::{
    clamp,
    Vector3,
};

use physics::{
    Ray,
    HittableList,
    HitRecord,
    Hittable,
    Camera
};

use model::Sphere;

use std::{
    f64::INFINITY,
    rc::Rc
};

fn write_color(pixcolor: Vector3, samples_per_pixel: u32) -> image::Rgb<u8> {
    let mut r = pixcolor.x();
    let mut g = pixcolor.y();
    let mut b = pixcolor.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    Vector3(clamp(r, 0.0, 0.999), clamp(g, 0.0, 0.999), clamp(b, 0.0, 0.999)).into_rgb()
}

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
    let ratio = 2.0;
    let width = 200;
    let height = (width as f64/ ratio) as u32;
    let samples_per_pixel = 100;

    // The traverse direction of Rust's Vec is different from C++ std::vector's
    let mut world = HittableList::new();
    world.add(Rc::from(Sphere::new(Vector3(0.0, -100.5, -1.0), 100.0)));
    world.add(Rc::from(Sphere::new(Vector3(0.0, 0.0, -1.0), 0.5)));

    let cam = Camera::new();
    

    let mut imgbuf = image::ImageBuffer::new(width, height);
    for (x, y, pix) in imgbuf.enumerate_pixels_mut() {
        let mut pixcolor = Vector3::new();
        for _ in 0..samples_per_pixel {
            let u = (x as f64 + rand::random::<f64>()) / width as f64;
            let v = ((height - y) as f64 + rand::random::<f64>())/ height as f64;
            let ray = cam.get_ray(u, v);
            pixcolor += ray_color(&ray, &world)
        }
        
        *pix = write_color(pixcolor, samples_per_pixel)
    }

    imgbuf.save("test.png").unwrap();
}
