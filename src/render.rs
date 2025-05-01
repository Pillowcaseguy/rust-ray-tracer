use crate::{
    matrix::{Unit, Vec3},
    ray::{self, Ray},
    ASPECT_RATIO, HEIGHT, WIDTH,
};

pub fn render(buffer: &mut [u32]) {
    // Ray origin
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let u = i as f64 / (WIDTH - 1) as f64;
            let v = j as f64 / (HEIGHT - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = r.ray_color();

            let r = (255.999 * pixel_color.x()) as u32;
            let g = (255.999 * pixel_color.y()) as u32;
            let b = (255.999 * pixel_color.z()) as u32;

            let index = i * WIDTH + j;

            buffer[index] = from_u8_rgb(r, g, b);
        }
    }
}

fn from_u8_rgb(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) | (g << 8) | b
}
