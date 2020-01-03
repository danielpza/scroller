extern crate sdl2;

mod core;
use crate::core::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::video::FullscreenType;
use std::time::Duration;

trait ExtCanvas {
    fn clear_color(&mut self, color: Color);
    fn fill_rect_color(&mut self, color: Color, rect: sdl2::rect::Rect);
    fn toggle_fullscreen(&mut self) -> Result<(), String>;
}

impl ExtCanvas for sdl2::render::WindowCanvas {
    fn clear_color(&mut self, color: Color) {
        self.set_draw_color(color);
        self.clear();
    }
    fn fill_rect_color(&mut self, color: Color, rect: sdl2::rect::Rect) {
        self.set_draw_color(color);
        self.fill_rect(rect).unwrap()
    }
    fn toggle_fullscreen(&mut self) -> Result<(), String> {
        let window = self.window_mut();
        window.set_fullscreen(if window.fullscreen_state() == FullscreenType::True {
            FullscreenType::Off
        } else {
            FullscreenType::True
        })?;
        Ok(())
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
                } => canvas.toggle_fullscreen()?,
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
    font: &sdl2::ttf::Font,
) -> Result<End, String> {
    let btnw = 150;
    let btnh = 50;
    let texture_creator = canvas.texture_creator();
    let bg_color = Color::RGB(255, 255, 255);
    let font_color = Color::RGB(0, 0, 0);
    let btn_color = Color::RGB(200, 200, 200);
    let with_margin = |rect: sdl2::rect::Rect| {
        let mut cp = rect;
        let center = cp.center();
        cp.resize(cp.width() - 40, cp.height() - 20);
        cp.center_on(center);
        cp
    };
    let render_font = |text: &str| {
        let surface = font.render(text).blended(font_color).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .unwrap();
        texture
    };
    let (w, h) = canvas.output_size()?;
    let mut btn_play_rect = sdl2::rect::Rect::new(0, 0, btnw, btnh);
    btn_play_rect.center_on((w as i32 / 2, 100));
    let btn_play_texture = render_font("PLAY");
    let mut btn_quit_rect = sdl2::rect::Rect::new(0, 0, btnw, btnh);
    btn_quit_rect.center_on((w as i32 / 2, h as i32 - 100));
    let btn_quit_texture = render_font("QUIT");
    let mut btn_fullscreen_rect = sdl2::rect::Rect::new(0, 0, btnw * 2, btnh);
    btn_fullscreen_rect.center_on((w as i32 / 2, 100 + btnh as i32 * 2));
    let btn_fullscreen_texture = render_font("FULLSCREEN");
    loop {
        canvas.clear_color(bg_color);
        canvas.fill_rect_color(btn_color, btn_play_rect);
        canvas.fill_rect_color(btn_color, btn_quit_rect);
        canvas.fill_rect_color(btn_color, btn_fullscreen_rect);
        canvas.copy(&btn_play_texture, None, Some(with_margin(btn_play_rect)))?;
        canvas.copy(&btn_quit_texture, None, Some(with_margin(btn_quit_rect)))?;
        canvas.copy(
            &btn_fullscreen_texture,
            None,
            Some(with_margin(btn_fullscreen_rect)),
        )?;
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
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                keymod: sdl2::keyboard::Mod::LCTRLMOD,
                ..
            } => canvas.toggle_fullscreen()?,
            Event::MouseButtonUp { x, y, .. } => {
                if btn_play_rect.contains_point(sdl2::rect::Point::new(x, y)) {
                    break Ok(End::Play);
                }
                if btn_quit_rect.contains_point(sdl2::rect::Point::new(x, y)) {
                    break Ok(End::Quit);
                }
                if btn_fullscreen_rect.contains_point(sdl2::rect::Point::new(x, y)) {
                    canvas.toggle_fullscreen()?;
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
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = std::path::Path::new("VCR_OSD_MONO_1.001.ttf");
    let font = ttf_context.load_font(font_path, 128)?;
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
        match main_menu_loop(&mut canvas, &mut event_pump, &font)? {
            End::Play => match game_loop(&mut canvas, &mut event_pump)? {
                End::Quit => break Ok(()),
                _ => (),
            },
            End::Quit => break Ok(()),
            _ => (),
        }
    }
}
