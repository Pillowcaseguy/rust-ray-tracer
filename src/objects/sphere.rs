use crate::{
    matrix::{self, Vec3},
    ray::Ray,
};

pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }

    pub fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool {
        let oc = r.origin() - center;
        let a = matrix::dot(r.direction(), r.direction());
        let b = 2.0 * matrix::dot(oc, r.direction());
        let c = matrix::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        discriminant >= 0.0
    }
}
