use crate::config::Global;
use crate::player::Player;

//sdl2
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;

pub struct Renderer<'a> {
    pub config: &'a Global,
    pub canvas: Canvas<Window>
}

impl<'a> Renderer<'a> {
    pub fn new(config: &'a Global, canvas: Canvas<Window>) -> Self {
        Self { config, canvas }
    }

    pub fn render(&mut self, player: &Player) {
        //background
        self.canvas.set_draw_color(self.config.backrgound_color);
        self.canvas.clear();

        //obsticles

        //player
        self.canvas.set_draw_color(Color::RED);
        let pos = player.position.round();
        self.canvas.filled_circle(pos.x as i16, pos.y as i16, player.radius, Color::RED).unwrap();

        //present the render
        self.canvas.present();
    }
}