mod matrix;
mod ray;
mod render;

use minifb::{Key, Window, WindowOptions};

use std::thread::spawn;

use crate::matrix::{Unit, Vec3};
use crate::ray::Ray;
use crate::render::render;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 640;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    render(&mut buffer);

    init_window(buffer);
}

fn init_window(buffer: Vec<u32>) {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
