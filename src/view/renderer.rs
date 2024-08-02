use crate::config::Global;

//sdl2
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer<'a> {
    pub config: &'a Global,
    pub canvas: Canvas<Window>
}

impl<'a> Renderer<'a> {
    pub fn new(config: &'a Global, canvas: Canvas<Window>) -> Self {
        Self { config, canvas }
    }

    pub fn render(&mut self) {
        //background
        self.canvas.set_draw_color(self.config.backrgound_color);
        self.canvas.clear();


        //present the render
        self.canvas.present();
    }
}