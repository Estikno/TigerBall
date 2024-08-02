//sdl2
use sdl2::pixels::Color;

pub struct Resolution {
    pub width: u32,
    pub height: u32
}

pub struct Global {
    pub resolution: Resolution,
    pub backrgound_color: Color,
    pub title: String,
    pub gravity: f32
}