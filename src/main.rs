use std::fs;
use std::io::{self, Write};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    shape_width: usize,
    shape_height: usize,
}

// ---- Vec3 struct ----
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn multiply(&self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    fn normalize(&self) -> Vec3 {
        let len = self.length();
        
        if len == 0.0{
            return Vec3::new(0.0,0.0,0.0)
        }

        Vec3::new(self.x / len, self.y / len, self.z / len)

    }
}

// ---- Ray struct ----
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    fn at(&self, t: f64) -> Vec3 {
        self.origin.add(&self.direction.multiply(t))
    }
}

// ---- Sphere struct ----
struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin.subtract(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}

fn ray_color(ray: &Ray, sphere: &Sphere) -> Vec3 {
    if let Some(t) = sphere.hit(ray) {
        let hit_point = ray.at(t);
        let normal = hit_point.subtract(&sphere.center).normalize();
        return Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0).multiply(0.5);
    }

    // Background gradient
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0).multiply(1.0 - t).add(&Vec3::new(0.5, 0.7, 1.0).multiply(t))
}

// ---- Write PPM ----
fn write_ppm(filename: &str, width: usize, height: usize, image: &[Vec3]) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", width, height)?;
    writeln!(file, "255")?;

    for pixel in image {
        let r = (pixel.x * 255.99) as u8;
        let g = (pixel.y * 255.99) as u8;
        let b = (pixel.z * 255.99) as u8;
        writeln!(file, "{} {} {}", r, g, b)?;
    }

    Ok(())
}

// ---- Main program ----
fn main() {

    let config_data = fs::read_to_string("data/config.json").expect("Failed to read config");
    let config: Config = serde_json::from_str(&config_data).expect("Failed to parse config");

    let width = config.shape_width;
    let height = config.shape_height;

    println!("{} {}",width,height);

    // Image dimensions
    //let width = 500;
    //let height = 250;

    // Aspect ratio
    let aspect_ratio = width as f64 / height as f64;

    // Camera setup
    let camera_origin = Vec3::new(0.0, 0.0, 0.0);
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    // Define a sphere
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

    // Create image buffer
    let mut image = Vec::new();

    // Iterate over pixels
    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
            let vertical = Vec3::new(0.0, viewport_height, 0.0);
            let lower_left_corner = camera_origin
                .subtract(&horizontal.multiply(0.5))
                .subtract(&vertical.multiply(0.5))
                .subtract(&Vec3::new(0.0, 0.0, focal_length));

            let ray_direction = lower_left_corner
                .add(&horizontal.multiply(u))
                .add(&vertical.multiply(v))
                .subtract(&camera_origin);

            let ray = Ray::new(camera_origin, ray_direction);

            /*if sphere.hit(&ray) {
                image.push(Vec3::new(1.0, 0.0, 0.0)); //Red
            } else {
                //let color = Vec3::new(0.5, 0.7, 1.0); //Sky blue. Commented out as testing reference.
                let unit_direction = ray.direction.normalize();
                //let t = 0.5 * (unit_direction.y + 1.0);
                let t = 0.25 * (unit_direction.x + unit_direction.y + 2.0); // range [0,1]
                //let color = Vec3::new(1.0, 1.0, 1.0).multiply(1.0 - t).add(&Vec3::new(0.5, 0.7, 1.0).multiply(t)); //Sky blue
                let color = Vec3::new(1.0, 1.0, 1.0).multiply(1.0 - t).add(&Vec3::new(0.8, 0.2, 0.8).multiply(t)); //Dark green
                image.push(color);
            }*/

            let color = ray_color(&ray, &sphere);
            image.push(color);

        }
    }

    // Write output file
    write_ppm("output.ppm", width, height, &image).expect("Failed to write image");
}
