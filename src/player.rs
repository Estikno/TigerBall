use std::ops::{Add, Sub, Mul, Div};

use crate::config::Global;

// Definir una trait específica para los tipos permitidos
pub trait Vector2Numeric: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Into<f64> {}
impl Vector2Numeric for i32 {}
impl Vector2Numeric for f32 {}

#[derive(Clone)]
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

    pub fn check_collision(&mut self, config: &Global) {
        let x = (self.position.x + self.velocity.x).round() as i32;
        let y = (self.position.y + self.velocity.y).round() as i32;

        let radius_converted = self.radius as i32;

        if x - radius_converted < 0 || x + radius_converted > config.resolution.width as i32 {
            self.velocity.x *= -1.0 * 0.9;
        }

        if y - radius_converted < 0 || y + radius_converted > config.resolution.height as i32 {
            self.velocity.y *= -1.0 * 0.9;
        }
    }
}