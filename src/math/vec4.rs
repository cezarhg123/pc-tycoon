use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

pub const fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 {
        x, y, z, w
    }
}

impl<T: Copy> Vec4<T> {
    pub fn as_raw(&self) -> [T; 4] {
        [
            self.x,
            self.y,
            self.z,
            self.w
        ]
    }
}

impl<T: Add<Output = T>> Add<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn add(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w
        }
    }
}

impl<T: Sub<Output = T>> Sub<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn sub(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w
        }
    }
}

impl<T: Mul<Output = T>> Mul<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn mul(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec4<T> {
    type Output = Self;
    
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl<T: Div<Output = T>> Div<Vec4<T>> for Vec4<T> {
    type Output = Self;

    fn div(self, rhs: Vec4<T>) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec4<T> {
    type Output = Self;
    
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}
