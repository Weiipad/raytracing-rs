use image;

mod rmath;
mod physics;
mod model;

use rmath::{
    clamp,
    Vector3,
    Camera,
    random_double,
};

use physics::{
    material::{
        Lambertian,
        Metal,
        Dielectric
    },
    Ray,
    HittableList,
    Hittable,
    
};


#[allow(unused_imports)]
use model::{Sphere, Plane};


use std::{
    f64::INFINITY,
    sync::{
        Arc,
        mpsc::channel
    },
};

fn gamma_correction(x: f64) -> f64 {
    x.sqrt()
}

fn write_color(pixcolor: &Vector3, samples_per_pixel: u32) -> image::Rgb<u8> {
    let scale = 1.0 / samples_per_pixel as f64;
    pixcolor.map(move|x| clamp(gamma_correction(x * scale), 0.0, 0.999)).into_rgb()
}

// every elements in color vector are in [0, 1]
fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Vector3 {
    if depth <= 0 {
        return Vector3::zero()
    }

    // Detects if the ray hit the objects 
    if let Some(rec) = world.hit(r, 0.001..INFINITY) {
        // if hits the surface, emit a new ray (to get color)
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            return attenuation.mul_one_by_one(ray_color(&scattered, world, depth - 1))
        }
        return Vector3::zero()
    }

    let unit_dir = r.get_direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Vector3(1.0, 1.0, 1.0) + t * Vector3(0.5, 0.7, 1.0)
}

fn main() {
    let ratio = 2.0 / 1.0;
    // You can change the resolution of picture here (don't edit the ratio).
    let width = 384;
    let height = (width as f64 / ratio) as u32;

    let hw = width / 4;
    let hh = height / 4;
    
    let samples_per_pixel = 100;
    let max_depth = 200;

    // The traverse direction of Rust's Vec is different from C++ std::vector's
    let mut world = HittableList::new();
    world.add(Arc::from( Plane::new(Vector3(0.0, 1.0, 0.0), -Vector3(0.0, 0.5, 0.0), Arc::from(Metal::new(Vector3::new(0.5), 0.2))) ));
    //world.add(Arc::from( Plane::new(Vector3(0.0, 1.0, 0.0), Vector3(0.0, 1.0, 0.0), Arc::from(Dielectric::new(1.7))) ));
    //world.add(Arc::from( Sphere::new(Vector3(0.0, -100.5, -1.0), 100.0, Arc::from(Lambertian::new(Vector3(0.8, 0.8, 0.0)))) ));
    world.add(Arc::from( Sphere::new(Vector3(0.0, 0.0, -1.0), 0.5, Arc::from(Lambertian::new(Vector3(0.7, 0.3, 0.3)))) ));
    world.add(Arc::from( Sphere::new(Vector3(-1.0, 0.0, -1.0), 0.5, Arc::from(Dielectric::new(1.7))) ));
    world.add(Arc::from( Sphere::new(Vector3(1.0, 0.0, -1.0), 0.5, Arc::from(Metal::new(Vector3::new(0.8), 0.0))) ));
    let world_shared = Arc::from(world);

    let cam_shared = Arc::from(Camera::new(Vector3(-2.0, 2.0, 1.0), Vector3(0.0, 0.0, -1.0), Vector3(0.0, 1.0, 0.0), 45.0, ratio));

    let mut img = image::RgbImage::new(width, height);

    let mut threads = vec![];

    let (sender, receiver) = channel();
      
    let start_time = std::time::SystemTime::now();
    for i in 0..4 {
        for j in 0..4 {
            let sender = sender.clone();
            let cam = cam_shared.clone();
            let world = world_shared.clone();

            let handle = std::thread::spawn(move || {    
                for y in i*hh..(i+1)*hh {
                    for x in j*hw..(j+1)*hw {
                        let mut pixcolor = Vector3::zero();
                        for _ in 0..samples_per_pixel {
                            let u = (x as f64 + random_double()) / width as f64;
                            let v = ((height - y) as f64 + random_double())/ height as f64;
                            let ray = cam.get_ray(u, v);
                            pixcolor += ray_color(&ray, &world, max_depth);
                        }
                        sender.send((x, y, write_color(&pixcolor, samples_per_pixel))).unwrap();
                    }
                }
                drop(sender);
            });
            threads.push(handle)
        }
    }

    drop(sender);

    while let Ok((x, y, pixel)) = receiver.recv() {
        img.put_pixel(x, y, pixel);
    }
    
    println!("Time elapsed: {:?}", start_time.elapsed().unwrap_or_default());

    img.save("test.png").unwrap();
}
