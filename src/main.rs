mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;

use std::sync::Arc;
use vec3::{Color, Vec3, Point3};
use ray::Ray;
use hittable::{Hittable, HitRecord};
use sphere::Sphere;
use hittable_list::HittableList;
use std::io::Write;
use interval::Interval;
use rand::prelude::*;


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Rendering
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
                pixel_color += ray_color(&r, &world);
            }  
            write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel);
        }
    }

    eprintln!("Done.");
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut rec = HitRecord {
        p: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };
    if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn write_color(out: &mut impl Write, pixel_color: Vec3, samples_per_pixel: usize) {
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    writeln!(out, "{} {} {}", 
             (256.0 * clamp(r, 0.0, 0.999)) as i32, 
             (256.0 * clamp(g, 0.0, 0.999)) as i32, 
             (256.0 * clamp(b, 0.0, 0.999)) as i32).unwrap();
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
}

fn random_double() -> f64 {
    rand::random()
}

fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}
