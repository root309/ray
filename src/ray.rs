use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = *self.origin() - *center;
        let a = self.direction().length_squared();
        let half_b = oc.dot(self.direction());
        let c = oc.length_squared() - radius * radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
