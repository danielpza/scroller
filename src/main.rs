extern crate sdl2;

mod core;
use crate::core::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::FullscreenType;
use std::time::Duration;

trait ExtCanvas<T> {
    fn clear_color(&mut self, color: Color);
    fn fill_rect_color(&mut self, color: Color, rect: sdl2::rect::Rect);
}

impl<T: sdl2::render::RenderTarget> ExtCanvas<T> for Canvas<T> {
    fn clear_color(&mut self, color: Color) {
        self.set_draw_color(color);
        self.clear();
    }
    fn fill_rect_color(&mut self, color: Color, rect: sdl2::rect::Rect) {
        self.set_draw_color(color);
        self.fill_rect(rect).unwrap();
    }
}
impl Into<sdl2::rect::Rect> for Rect {
    fn into(self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            self.size.x as u32,
            self.size.y as u32,
        )
    }
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

    let mut game = core::Game::new(width as i32, height as i32);
    let player_color = Color::RGB(255, 255, 255);
    let mut canvas = window.into_canvas().build().unwrap();
    let bgcolor = Color::RGB(100, 100, 100);
    canvas.clear_color(bgcolor);
    canvas.fill_rect_color(player_color, game.player.shape.into());
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let mut jump = false;
        // input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => jump = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    keymod: sdl2::keyboard::Mod::LCTRLMOD,
                    ..
                } => {
                    let window = canvas.window_mut();
                    window.set_fullscreen(
                        if window.fullscreen_state() == FullscreenType::True {
                            FullscreenType::Off
                        } else {
                            FullscreenType::True
                        },
                    )?;
                }
                _ => {}
            }
        }
        game.step(Input {
            jump,
            left: event_pump.keyboard_state().is_scancode_pressed(Scancode::A),
            right: event_pump.keyboard_state().is_scancode_pressed(Scancode::D),
        });
        canvas.clear_color(bgcolor);
        canvas.fill_rect_color(player_color, game.player.shape.into());
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
    Ok(())
}
