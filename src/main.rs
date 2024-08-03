use std::time::{Duration, Instant};
use anyhow::Result;

//sdl2
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

//mods
mod view;
mod config;
mod player;
mod obsticles;

fn main() -> Result<()> {
    //configs
    let config = config::Global {
        resolution: config::Resolution { width: 1024, height: 576 },
        backrgound_color: Color::RGB(94, 94, 94),
        title: String::from("Tiger Ball"),
        gravity: 0.5,
        fps: 60,
        frame_delay: Duration::from_millis(1000 / 60)
    };

    //sdl2 initialization
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(&config.title, config.resolution.width, config.resolution.height)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    //renderer
    let mut renderer = view::Renderer::new(&config, canvas);

    //player
    let mut player = player::Player {
        position: player::Vector2 {
            x: 200.0,
            y: 200.0
        },
        velocity: player::Vector2 {
            x: 10.0,
            y: -5.0
        },
        radius: 20
    };

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut start_mooving = false;
    let mut mouse_start_pos: player::Vector2<i32> = player::Vector2 { x: 0, y: 0 };

    'main: loop {
        let frame_start = Instant::now();

        //events part
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main
                },
                Event::MouseButtonDown { mouse_btn, clicks, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        mouse_start_pos = player::Vector2 { x, y };
                    }
                }
                Event::MouseButtonUp { mouse_btn, clicks, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        player.velocity = player::Vector2 {
                            x: (x - mouse_start_pos.x) as f32 * 0.2,
                            y: (y - mouse_start_pos.y) as f32 * 0.2
                        };
                        start_mooving = true;
                    }
                }
                _ => {}
            }
        }

        // Rest of the game loop goes here...
        if start_mooving {
            player.check_collision(&config);
            player.make_movement(&config.gravity);
        }

        //rendering part
        renderer.render(&player);

        // Control frame time
        let frame_time = frame_start.elapsed();
        if frame_time < config.frame_delay {
            std::thread::sleep(config.frame_delay - frame_time);
        }
    }

    Ok(())
}
