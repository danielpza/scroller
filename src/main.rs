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
        self.fill_rect(rect).unwrap()
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
    let sizey = 10;
    let scale = height as f32 / sizey as f32;
    let sizex = ((width as f32) / scale).ceil() as i32;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("scroller", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut game = core::Game::new(sizex, sizey);
    let player_color = Color::RGB(255, 255, 255);
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let bgcolor = Color::RGB(100, 100, 100);
    let block_color = Color::RGB(10, 10, 10);
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
            jump: event_pump.keyboard_state().is_scancode_pressed(Scancode::W),
        });
        canvas.clear_color(bgcolor);
        canvas.set_draw_color(block_color);
        let mut rect = sdl2::rect::Rect::new(0, 0, scale as u32, scale as u32);
        for i in game.offset as usize..game.offset as usize + width as usize {
            let h = game.map.get(i as i32);
            rect.set_x(((i as f32 - game.offset) * scale as f32) as i32);
            for j in h..sizey {
                rect.set_y(j as i32 * scale as i32);
                canvas.fill_rect(rect)?;
            }
        }
        let mut rect = game.player.shape;
        rect.position.x -= game.offset;
        canvas.fill_rect_color(player_color, (rect * scale).into());
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
    Ok(())
}
