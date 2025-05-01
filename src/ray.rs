use crate::{
    matrix::{self, Vec3},
    objects::sphere,
};

#[derive(Default)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }

    pub fn ray_color(&self) -> Vec3 {
        if sphere::Sphere::hit_sphere(Vec3::new(-1.0, -1.0, -2.0), 0.5, self) {
            return Vec3::new(2.0, 4.0, 0.0);
        }

        let unit_direction = matrix::unit_vector(self.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
