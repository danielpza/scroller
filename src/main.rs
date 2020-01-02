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

struct Button {
    // text: String,
    rect: sdl2::rect::Rect,
}

impl Button {
    pub fn new(rect: sdl2::rect::Rect) -> Button {
        Button { rect }
    }
}

trait ExtCanvas {
    fn clear_color(&mut self, color: Color);
    fn fill_rect_color(&mut self, color: Color, rect: sdl2::rect::Rect);
    fn draw_button(&mut self, btn: &Button);
}

impl<T: sdl2::render::RenderTarget> ExtCanvas for Canvas<T> {
    fn clear_color(&mut self, color: Color) {
        self.set_draw_color(color);
        self.clear();
    }
    fn fill_rect_color(&mut self, color: Color, rect: sdl2::rect::Rect) {
        self.set_draw_color(color);
        self.fill_rect(rect).unwrap()
    }
    fn draw_button(&mut self, btn: &Button) {
        self.fill_rect_color(Color::RGB(255, 255, 255), btn.rect);
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

enum End {
    Play,
    Finish,
    Quit,
}

fn game_loop(
    canvas: &mut sdl2::render::WindowCanvas,
    event_pump: &mut sdl2::EventPump,
) -> Result<End, String> {
    let sizey = 10;
    let sizex = 14;
    let scale = 60;
    let mut game = core::Game::new(sizex, sizey);
    let player_color = Color::RGB(255, 255, 255);
    let bgcolor = Color::RGB(100, 100, 100);
    let block_color = Color::RGB(10, 10, 10);
    canvas.clear_color(bgcolor);
    canvas.present();
    'running: loop {
        // input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running Ok(End::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running Ok(End::Finish),
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
        for i in game.offset as usize..game.offset as usize + sizex as usize + 1 {
            let h = game.map.get(i as i32);
            rect.set_x(((i as f32 - game.offset) * scale as f32) as i32);
            for j in h..sizey {
                rect.set_y(j as i32 * scale as i32);
                canvas.fill_rect(rect)?;
            }
        }
        let mut rect = game.player.shape;
        rect.position.x -= game.offset;
        canvas.fill_rect_color(player_color, (rect * scale as f32).into());
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}

fn main_menu_loop(
    canvas: &mut sdl2::render::WindowCanvas,
    event_pump: &mut sdl2::EventPump,
) -> Result<End, String> {
    let btnw = 150;
    let btnh = 50;
    let mut btn_play = Button::new(sdl2::rect::Rect::new(0, 0, btnw, btnh));
    let mut btn_quit = Button::new(sdl2::rect::Rect::new(0, 0, btnw, btnh));
    let (w, h) = canvas.output_size()?;
    btn_play.rect.center_on((w as i32 / 2, 100));
    btn_quit.rect.center_on((w as i32 / 2, h as i32 - 100));
    loop {
        canvas.clear_color(Color::RGB(100, 100, 100));
        canvas.draw_button(&btn_play);
        canvas.draw_button(&btn_quit);
        canvas.present();
        let event = event_pump.wait_event();
        match event {
            Event::Quit { .. } => break Ok(End::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => break Ok(End::Quit),
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => break Ok(End::Play),
            Event::MouseButtonUp { x, y, .. } => {
                if btn_play.rect.contains_point(sdl2::rect::Point::new(x, y)) {
                    break Ok(End::Play);
                }
                if btn_quit.rect.contains_point(sdl2::rect::Point::new(x, y)) {
                    break Ok(End::Quit);
                }
            }
            _ => {}
        }
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
    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    event_pump.disable_event(sdl2::event::EventType::MouseMotion);
    loop {
        match main_menu_loop(&mut canvas, &mut event_pump)? {
            End::Play => match game_loop(&mut canvas, &mut event_pump)? {
                End::Quit => break Ok(()),
                _ => (),
            },
            End::Quit => break Ok(()),
            _ => (),
        }
    }
}
