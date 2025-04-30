use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub type Unit = u32;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [Unit; 3],
}

impl Vec3 {
    pub fn new(x: Unit, y: Unit, z: Unit) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> Unit {
        self.e[0]
    }

    pub fn y(&self) -> Unit {
        self.e[1]
    }

    pub fn z(&self) -> Unit {
        self.e[2]
    }

    pub fn length(&self) -> Unit {
        (self.length_squared()).isqrt()
    }

    pub fn length_squared(&self) -> Unit {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

// Output formatting
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// Vec3 += Vec3
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

// Vec3 *= f64
impl MulAssign<Unit> for Vec3 {
    fn mul_assign(&mut self, t: Unit) {
        *self = *self * t;
    }
}

// Vec3 /= f64
impl DivAssign<Unit> for Vec3 {
    fn div_assign(&mut self, t: Unit) {
        *self = *self / t;
    }
}

// Vec3 + Vec3
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

// Vec3 - Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

// Vec3 * Vec3
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

// f64 * Vec3
impl Mul<Vec3> for Unit {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

// Vec3 * f64
impl Mul<Unit> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: Unit) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

// Vec3 / f64
impl Div<Unit> for Vec3 {
    type Output = Vec3;

    fn div(self, t: Unit) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> Unit {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
