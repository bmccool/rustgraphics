extern crate sdl2;

use graphics::sprite::Render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;

#[allow(unused_imports)]
use std::{thread, time};

mod graphics;
use graphics::line::Bresenhams;
use graphics::rotation::RotationMatrixX;
use graphics::rotation::RotationMatrixY;
use graphics::rotation::RotationMatrixZ;
use graphics::rotation::Rotation;

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
    let mut sprite = graphics::sprite::Sprite{ origin: graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 }, ..Default::default()};
    //sprite.lines.push(graphics::line::Line{start: graphics::point::Point{x: 0.0, y: 0.0, z: 0.0}, end: graphics::point::Point{x: 100.0, y: 100.0, z: 0.0}, current: graphics::point::Point{x: 0.0, y: 0.0, z: 0.0}});
    sprite.points.push(graphics::point::Point{x:  20.0, y:  20.0, z: 0.0});
    sprite.points.push(graphics::point::Point{x: -20.0, y:  20.0, z: 0.0});
    sprite.points.push(graphics::point::Point{x:  20.0, y: -20.0, z: 0.0});
    sprite.points.push(graphics::point::Point{x: -20.0, y: -20.0, z: 0.0});

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        let center = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0};
        let mut moving_point = graphics::point::Point{x: 0.0, y: 300.0, z: 0.0};
        let rotation_matrix_z = RotationMatrixX{ angle:0.0 };
        moving_point = rotation_matrix_z.rotate(degrees, moving_point);
        moving_point = moving_point + center;
        let line = graphics::line::Line{start: center, end: moving_point, current: center};
        sprite.angle_x = degrees;
        let callback = |x, y, color| canvas.pixel(x, y, color);
        sprite.render(callback);
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