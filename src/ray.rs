use crate::matrix::{self, Unit, Vec3};

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

    pub fn at(&self, t: Unit) -> Vec3 {
        self.orig + t * self.dir
    }
}

pub fn ray_color(r: &Ray) -> Vec3 {
    let unit_direction = matrix::unit_vector(r.direction());

    let t = unit_direction.y() + 1;

    (1 - t) * Vec3::new(5, 7, 1) + t * Vec3::new(5, 7, 2)
}
