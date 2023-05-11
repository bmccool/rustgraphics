extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;

use std::{thread, time};

mod graphics;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //let mut lastx = 0;
    //let mut lasty = 0;

    let mut events = sdl_context.event_pump()?;

    let mut degrees: f64 = 0.0;

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        let center = graphics::point::Point{x: 400.0, y: 300.0};
        let px = (degrees.sin() * 300.0) + center.x;
        let py = (degrees.cos() * 300.0) + center.y;
        let moving_point = graphics::point::Point{x: px, y: py};
        let line = graphics::line::Line{start: center, end: moving_point, current: center};
        for p in line.into_iter().take(1000) {
            let _ = canvas.pixel(p.x as i16, p.y as i16, 0xFFFFFFFFu32);
            //println!("x: {}, y: {}", p.x, p.y)
        }
        canvas.present();
        degrees += 0.0001;
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        println!("space down");
                        for i in 0..400 {
                            canvas.pixel(i as i16, i as i16, 0xFFFFFFFu32)?;
                        }
                        canvas.present();
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    //let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    //let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    //lastx = x as i16;
                    //lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    //canvas.present();
                }

                _ => {}
            }
        }

    }

    Ok(())
}