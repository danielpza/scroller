extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

trait ExtCanvas<T> {
    fn clear_color(&mut self, color: Color);
    fn fill_rect_color(&mut self, color: Color, rect: Rect);
}

impl<T: sdl2::render::RenderTarget> ExtCanvas<T> for Canvas<T> {
    fn clear_color(&mut self, color: Color) {
        self.set_draw_color(color);
        self.clear();
    }
    fn fill_rect_color(&mut self, color: Color, rect: Rect) {
        self.set_draw_color(color);
        self.fill_rect(rect).unwrap();
    }
}

struct Player {
    shape: Rect,
    color: Color,
}

fn main() -> Result<(), String> {
    let width = 800;
    let height = 600;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("scroller", width, height)
        .position_centered()
        .build()
        .unwrap();

    let player = Player {
        shape: Rect::new(0, 0, 32, 32),
        color: Color::RGB(0, 255, 255),
    };
    let mut canvas = window.into_canvas().build().unwrap();
    let bgcolor = Color::RGB(100, 100, 100);
    canvas.fill_rect_color(player.color, player.shape);
    canvas.clear_color(bgcolor);
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // logic
        // draw
        canvas.clear();
        canvas.clear_color(bgcolor);
        canvas.fill_rect_color(player.color, player.shape);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
    Ok(())
}
