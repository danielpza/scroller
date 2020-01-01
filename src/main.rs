extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

trait ExtCanvas<T> {
    fn fill_rect_color(&mut self, color: Color, rect: Rect);
}

impl<T: sdl2::render::RenderTarget> ExtCanvas<T> for Canvas<T> {
    fn fill_rect_color(&mut self, color: Color, rect: Rect) {
        self.set_draw_color(color);
        self.fill_rect(rect).unwrap();
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("scroller", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let player = Rect::new(40, 20, 100, 100);
    canvas.fill_rect_color(Color::RGB(100, 100, 100), player);
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
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
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.fill_rect_color(Color::RGB(100, 100, 100), player);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
    Ok(())
}
