use image;

mod rmath;
mod physics;
mod model;

use rmath::{
    clamp,
    Vector3,
    random_double,
};

use physics::{
    Ray,
    HittableList,
    HitRecord,
    Hittable,
    Camera
};


use model::{Sphere, Plane};


use std::{
    f64::INFINITY,
    sync::{
        Arc,
        Mutex
    },
};

fn write_color(pixcolor: &Vector3, samples_per_pixel: u32) -> image::Rgb<u8> {
    let mut r = pixcolor.x();
    let mut g = pixcolor.y();
    let mut b = pixcolor.z();

    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(r * scale);
    g = f64::sqrt(g * scale);
    b = f64::sqrt(b * scale);

    Vector3(clamp(r, 0.0, 0.999), clamp(g, 0.0, 0.999), clamp(b, 0.0, 0.999)).into_rgb()
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vector3 {
    if depth <= 0 {
        return Vector3(0.0, 0.0, 0.0)
    }
    
    if let Some(rec) = world.hit(r, 0.001..INFINITY) {
        let target = rec.p + rec.normal + Vector3::from_random_in_unit_sphere();
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1)
    }
    let unit_dir = r.get_direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vector3(1.0, 1.0, 1.0) + t * Vector3(0.5, 0.7, 1.0)
}

fn main() {
    let width = 384;
    let height = 192;

    let hw = width / 4;
    let hh = height / 2;
    
    let samples_per_pixel = 100;
    let max_depth = 10;

    // The traverse direction of Rust's Vec is different from C++ std::vector's
    let mut world = HittableList::new();
    //world.add(Arc::from(Sphere::new(Vector3(0.0, -100.5, -1.0), 100.0)));
    world.add(Arc::from(Plane::new(Vector3(0.0, 1.0, 0.0), 0.5)));
    world.add(Arc::from(Sphere::new(Vector3(0.0, 0.0, -1.0), 0.5)));

    let world_shared = Arc::from(world);

    let cam_shared = Arc::from(Camera::new());

    let imgbuf = Arc::from(Mutex::from(image::RgbImage::new(width, height)));

    let mut threads = Vec::new();
    for i in 0..2 {
        for j in 0..4 {
            let img = imgbuf.clone();
            let cam = cam_shared.clone();
            let world = world_shared.clone();
            let handle = std::thread::spawn(move || {    
                for y in i*hh..(i+1)*hh {
                    for x in j*hw..(j+1)*hw {
                        let mut pixcolor = Vector3::new();
                        for _ in 0..samples_per_pixel {
                            let u = (x as f64 + random_double()) / width as f64;
                            let v = ((height - y) as f64 + random_double())/ height as f64;
                            let ray = cam.get_ray(u, v);
                            pixcolor += ray_color(&ray, &world, max_depth);
                        }
                        let mut img = img.lock().unwrap();
                        img.put_pixel(x, y, write_color(&pixcolor, samples_per_pixel));
                    }
                }
            });
            threads.push(handle);
        }
    }

    for h in threads {
        h.join().unwrap();
    }


    imgbuf.lock().unwrap().save("test.png").unwrap();
}
