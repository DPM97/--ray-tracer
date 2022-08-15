use std::{
    ops::{Add, Div, Mul},
};

#[derive(Debug)]
pub struct Vec3 {
    zero: i32,
    one: i32,
    two: i32,
}

pub trait TVec3 {
    fn length(&self) -> f64;
    fn length_sq(&self) -> f64;
}

impl TVec3 for Vec3 {
    fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }
    fn length_sq(&self) -> f64 {
        ((self.zero * self.zero) + (self.one * self.one) + (self.two * self.two)) as f64
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            zero: self.zero + other.zero,
            one: self.one + other.one,
            two: self.two + other.two,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            zero: self.zero * other.zero,
            one: self.one * other.one,
            two: self.two * other.two,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            zero: self.zero * other,
            one: self.one * other,
            two: self.two * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            zero: self * other.zero,
            one: self * other.one,
            two: self * other.two,
        }
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        (1 as f64 / self) * other
    }
}
