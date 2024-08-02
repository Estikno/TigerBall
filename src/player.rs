use std::ops::{Add, Sub, Mul, Div};

// Definir una trait específica para los tipos permitidos
pub trait Vector2Numeric: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Into<f64> {}
impl Vector2Numeric for i32 {}
impl Vector2Numeric for f32 {}

pub struct Vector2<T: Vector2Numeric> {
    pub x: T, 
    pub y: T,
}

impl Vector2<f32> {
    pub fn round(&self) -> Vector2<i32> {
        Vector2 { 
            x: self.x.round() as i32,
            y: self.y.round() as i32
        }
    }
}

pub struct Player {
    pub position: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub radius: i16
}

impl Player {
    pub fn make_movement(&mut self, gravity: &f32) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        self.velocity.y += *gravity;
    }
}