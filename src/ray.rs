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

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> bool {
        let oc = *self.origin() - *center;
        let a = self.direction().dot(self.direction());
        let b = 2.0 * oc.dot(self.direction());
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b*b - 4.0*a*c;
        discriminant >= 0.0
    }
}
