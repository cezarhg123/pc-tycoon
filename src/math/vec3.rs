use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

pub const fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 {
        x, y, z
    }
}

impl<T: Copy> Vec3<T> {
    pub fn as_raw(&self) -> [T; 3] {
        [
            self.x,
            self.y,
            self.z
        ]
    }
}

impl<T: Add<Output = T>> Add<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl<T: Sub<Output = T>> Sub<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Self;
    
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl<T: Div<Output = T>> Div<Vec3<T>> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: Vec3<T>) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Self;
    
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
