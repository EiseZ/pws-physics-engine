use std::ops::{Add, AddAssign};

#[derive(Copy, Clone)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T, z: T) -> Vector<T> {
        Vector {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl Add for Vector<f32> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector<f32> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}