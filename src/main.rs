extern crate sdl2;

use graphics::sprite::Render;
use graphics::sprite::Sprite;
use graphics::triangle::Triangle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;

use sdl2::gfx::primitives::DrawRenderer;

use std::f64::consts::SQRT_2;
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

fn get_square() -> Result<Sprite, String> {
    let mut sprite = Sprite{ origin: graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 }, ..Default::default()};
    let point_a = graphics::point::Point{x:  200.0, y:  200.0, z: 0.0};
    let point_b = graphics::point::Point{x: -200.0, y:  200.0, z: 0.0};
    let point_c = graphics::point::Point{x:  200.0, y: -200.0, z: 0.0};
    let point_d = graphics::point::Point{x: -200.0, y: -200.0, z: 0.0};
    sprite.lines.push(graphics::line::Line{start: point_a, end: point_b, current: point_a});
    sprite.lines.push(graphics::line::Line{start: point_b, end: point_d, current: point_b});
    sprite.lines.push(graphics::line::Line{start: point_d, end: point_c, current: point_a});
    sprite.lines.push(graphics::line::Line{start: point_c, end: point_a, current: point_a});
    return Ok(sprite);
}

fn get_tetrahedron() -> Result<Sprite, String> {
    let points = vec![0., 0., 0.612372, -0.288675, -0.5, -0.204124, -0.288675, 0.5, - 0.204124, 0.57735, 0., -0.204124];
    let faces = vec![3, 1, 2, 3, 3, 2, 1, 0, 3, 3, 0, 1, 3, 0, 3, 2];
    let scale = 200.0;
    let mut sprite = graphics::sprite::from_points_faces(points, faces, scale)?;
    sprite.origin = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 };
    return Ok(sprite);
}

fn get_cube() -> Result<Sprite, String> {
    let size: f64 = 0.5;
    let points = vec![-size, -size, -size, -size, -size, size, -size, size, -size, -size, size,  size, size, -size, -size, size, -size, size, size, size, -size, size, size, size];
    let faces = vec![4, 7, 3, 1, 5, 4, 7, 5, 4, 6, 4, 7, 6, 2, 3, 4, 3, 2, 0, 1, 4, 0, 2, 6, 4, 4, 1, 0, 4, 5];
    let scale = 200.0;
    let mut sprite = graphics::sprite::from_points_faces(points, faces, scale)?;
    sprite.origin = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 };
    return Ok(sprite);
}

fn get_octahedron() -> Result<Sprite, String> {
    let size = SQRT_2;
    let points = vec![-size, 0.0, 0.0, 0.0, size, 0.0, 0.0, 0.0, -size, 0.0, 0.0, size, 0.0, -size, 0.0, size, 0.0, 0.0];
    let faces = vec![3, 3, 4, 5, 3, 3, 5, 1, 3, 3, 1, 0, 3, 3, 0, 4, 3, 4, 0, 2, 3, 4, 2, 5, 3, 2, 0, 1, 3, 5, 2, 1];
    let scale = 200.0;
    let mut sprite = graphics::sprite::from_points_faces(points, faces, scale)?;
    sprite.origin = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 };
    return Ok(sprite);
}

fn get_dodecahedron() -> Result<Sprite, String> {
    let points = vec![-1.37638, 0.0, 0.262866, 1.37638, 0.0, -0.262866, -0.425325, -1.30902, 0.262866, -0.425325, 1.30902, 0.262866, 1.11352, -0.809017, 0.262866, 1.11352, 0.809017, 0.262866, -0.262866, -0.809017, 1.11352, -0.262866, 0.809017, 1.11352, -0.688191, -0.5, -1.11352, -0.688191, 0.5, -1.11352, 0.688191, -0.5, 1.11352, 0.688191, 0.5, 1.11352, 0.850651, 0.0, -1.11352, -1.11352, -0.809017, -0.262866, -1.11352, 0.809017, -0.262866, -0.850651, 0.0, 1.11352, 0.262866, -0.809017, -1.11352, 0.262866, 0.809017, -1.11352, 0.425325, -1.30902, -0.262866, 0.425325, 1.30902, -0.262866];
    let faces = vec![5, 14, 9, 8, 13, 0, 5, 1, 5, 11, 10, 4, 5, 4, 10, 6, 2, 18, 5, 10, 11, 7, 15, 6, 5, 11, 5, 19, 3, 7, 5, 5, 1, 12, 17, 19, 5, 1, 4, 18, 16, 12, 5, 3, 19, 17, 9, 14, 5, 17, 12, 16, 8, 9, 5, 16, 18, 2, 13, 8, 5, 2, 6, 15, 0, 13, 5, 15, 7, 3, 14, 0];
    let scale = 200.0;
    let mut sprite = graphics::sprite::from_points_faces(points, faces, scale)?;
    sprite.origin = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 };
    return Ok(sprite);
}

fn get_icosahedron() -> Result<Sprite, String> {
    let points = vec![0.0, 0.0, -0.951057, 0.0, 0.0, 0.951057, -0.850651, 0.0, -0.425325, 0.850651, 0.0, 0.425325, 0.688191, -0.5, -0.425325, 0.688191, 0.5, -0.425325, -0.688191, -0.5, 0.425325, -0.688191, 0.5, 0.425325, -0.262866, -0.809017, -0.425325, -0.262866, 0.809017, -0.425325, 0.262866, -0.809017, 0.425325, 0.262866, 0.809017, 0.425325];
    let faces = vec![3, 1, 11, 7, 3, 1, 7, 6, 3, 1, 6, 10, 3, 1, 10, 3, 3, 1, 3, 11, 3, 4, 8, 0, 3, 5, 4, 0, 3, 9, 5, 0, 3, 2, 9, 0, 3, 8, 2, 0, 3, 11, 9, 7, 3, 7, 2, 6, 3, 6, 8, 10, 3, 10, 4, 3, 3, 3, 5, 11, 3, 4, 10, 8, 3, 5, 3, 4, 3, 9, 11, 5, 3, 2, 7, 9, 3, 8, 6, 2];
    let scale = 200.0;
    let mut sprite = graphics::sprite::from_points_faces(points, faces, scale)?;
    sprite.origin = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 };
    return Ok(sprite);
}

fn get_spikey() -> Result<Sprite, String> {
    let points = vec![0.0, 0.0, -0.951057, 0.0, 0.0, 0.951057, -0.850651, 0.0, -0.425325, 0.850651, 0.0, 0.425325, -0.688191, -0.5, 0.425325, -0.688191, 0.5, 0.425325, 0.688191, -0.5, -0.425325, 0.688191, 0.5, -0.425325, -0.262866, -0.809017, -0.425325, -0.262866, 0.809017, -0.425325, 0.262866, -0.809017, 0.425325, 0.262866, 0.809017, 0.425325, -1.54435, 0.0, 0.294944, 1.54435, 0.0, -0.294944, -0.954458, 0.0, 1.2494, 0.954458, 0.0, -1.2494, -1.2494, 0.907744, -0.294944, -1.2494, -0.907744, -0.294944, 1.2494, 0.907744, 0.294944, 1.2494, -0.907744, 0.294944, -0.294944, 0.907744, 1.2494, -0.294944, -0.907744, 1.2494, 0.294944, 0.907744, -1.2494, 0.294944, -0.907744, -1.2494, -0.772173, 0.561016, -1.2494, -0.772173, -0.561016, -1.2494, 0.772173, 0.561016, 1.2494, 0.772173, -0.561016, 1.2494, -0.477229, 1.46876, 0.294944, -0.477229, -1.46876, 0.294944, 0.477229, 1.46876, -0.294944, 0.477229, -1.46876, -0.294944];
    let faces = vec![3, 20, 1, 11, 3, 20, 11, 5, 3, 20, 5, 1, 3, 14, 1, 5, 3, 14, 5, 4, 3, 14, 4, 1, 3, 21, 1, 4, 3, 21, 4, 10, 3, 21, 10, 1, 3, 27, 1, 10, 3, 27, 10, 3, 3, 27, 3, 1, 3, 26, 1, 3, 3, 26, 3, 11, 3, 26, 11, 1, 3, 23, 6, 8, 3, 23, 8, 0, 3, 23, 0, 6, 3, 15, 7, 6, 3, 15, 6, 0, 3, 15, 0, 7, 3, 22, 9, 7, 3, 22, 7, 0, 3, 22, 0, 9, 3, 24, 2, 9, 3, 24, 9, 0, 3, 24, 0, 2, 3, 25, 8, 2, 3, 25, 2, 0, 3, 25, 0, 8, 3, 28, 11, 9, 3, 28, 9, 5, 3, 28, 5, 11, 3, 12, 5, 2, 3, 12, 2, 4, 3, 12, 4, 5, 3, 29, 4, 8, 3, 29, 8, 10, 3, 29, 10, 4, 3, 19, 10, 6, 3, 19, 6, 3, 3, 19, 3, 10, 3, 18, 3, 7, 3, 18, 7, 11, 3, 18, 11, 3, 3, 31, 6, 10, 3, 31, 10, 8, 3, 31, 8, 6, 3, 13, 7, 3, 3, 13, 3, 6, 3, 13, 6, 7, 3, 30, 9, 11, 3, 30, 11, 7, 3, 30, 7, 9, 3, 16, 2, 5, 3, 16, 5, 9, 3, 16, 9, 2, 3, 17, 8, 4, 3, 17, 4, 2, 3, 17, 2, 8];
    let scale = 175.0;
    let mut sprite = graphics::sprite::from_points_faces(points, faces, scale)?;
    sprite.origin = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 };
    return Ok(sprite);
}

fn demo_tri(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<String, String> {
    let mut sprite = Sprite{ origin: graphics::point::Point{x: 400.0, y: 300.0, z: 0.0 }, ..Default::default()};
    let point_a = graphics::point::Point{x:  200.0, y:  -200.0, z: 0.0};
    let point_b = graphics::point::Point{x: -200.0, y:  -200.0, z: 0.0};
    let point_c = graphics::point::Point{x:  0.0, y: 200.0, z: 0.0};
    let tri = Triangle{ points: [point_a, point_b, point_c] };
    sprite.tris.push(tri);
    

    'demo_tri: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        let callback = |x, y, color| canvas.pixel(x, y, color);
        sprite.render(callback);
        canvas.present();
        
    }

    //return Ok(String::from("Success!"));
}


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

    let mut canvas: sdl2::render::Canvas<sdl2::video::Window> = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //let mut lastx = 0;
    //let mut lasty = 0;

    demo_tri(&mut canvas);

    let mut events = sdl_context.event_pump()?;

    let mut degrees_x: f64 = 0.0;
    let mut degrees_y: f64 = 0.0;
    let mut degrees_z: f64 = 0.0;
    let mut sprite = get_spikey()?;

    'main: loop {
        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        let center = graphics::point::Point{x: 400.0, y: 300.0, z: 0.0};
        let mut moving_point = graphics::point::Point{x: 0.0, y: 300.0, z: 0.0};
        let rotation_matrix_z = RotationMatrixX{ angle:0.0 };
        moving_point = rotation_matrix_z.rotate(degrees_x, moving_point);
        moving_point = moving_point + center;
        let line = graphics::line::Line{start: center, end: moving_point, current: center};
        sprite.angle_x = degrees_x;
        sprite.angle_y = degrees_y;
        sprite.angle_z = degrees_z;
        let callback = |x, y, color| canvas.pixel(x, y, color);
        sprite.render(callback);
        canvas.present();
        degrees_x += 0.0002;
        degrees_y += 0.0004;
        degrees_z += 0.0001;
        
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