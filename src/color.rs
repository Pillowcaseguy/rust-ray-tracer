use crate::matrix::Unit;

use std::io::Write;

pub fn write_color(out: &mut impl Write, pixel_color: [Unit; 3]) {
    // Write the translated [0, 255] value of each color component
    let r = (255.999 * pixel_color[0]) as i32;
    let g = (255.999 * pixel_color[1]) as i32;
    let b = (255.999 * pixel_color[2]) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
