use crate::{
    matrix::{Unit, Vec3},
    ray::{self, Ray},
    ASPECT_RATIO, HEIGHT, WIDTH,
};

pub fn render(buffer: &mut [u32]) {
    let viewport_height = 2;
    let viewport_width = (ASPECT_RATIO as u32) * viewport_height;
    let focal_length = 1;

    let origin = Vec3::new(0, 0, 0);
    let horizontal = Vec3::new(viewport_width, 0, 0);
    let vertical = Vec3::new(0, viewport_height, 0);
    let lower_left_corner = origin - horizontal / 2 - vertical / 2 - Vec3::new(0, 0, focal_length);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let u = ((j as f64 / WIDTH as f64) * 255.0) as Unit;
            let v = ((i as f64 / HEIGHT as f64) * 255.0) as Unit;

            let b = 0.2 * 255.0;

            let index = i * WIDTH + j;

            // let r = Ray::new(
            //     origin,
            //     lower_left_corner + u * horizontal + v * vertical - origin,
            // );
            // let pixel_color = ray::ray_color(&r);

            // buffer[index] = from_u8_rgb(pixel_color.x(), pixel_color.y(), pixel_color.z());
        }
    }
}

fn from_u8_rgb(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) | (g << 8) | b
}
