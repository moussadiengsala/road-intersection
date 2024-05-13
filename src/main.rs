use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

mod map;
use map::draw_map;

mod traffic;


mod cars;
use cars::handle_keyboard_event;

mod lane;
use lane::{Lane, Cross};

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const VEHICULE_WIDTH: u32 = 30;
    const SAFETY_DISTANCE: i32 = 50;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let mut lanes = vec![
        Lane::new(SAFETY_DISTANCE, Cross::First),
        Lane::new(SAFETY_DISTANCE, Cross::Second),
        Lane::new(SAFETY_DISTANCE, Cross::Third),
        Lane::new(SAFETY_DISTANCE, Cross::Fourth),
    ];
    
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(55, 64, 5));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    handle_keyboard_event(&event, &mut lanes, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
                }
            }
        }

        canvas.clear();
        // The rest of the game loop goes here...
        
        // map
        draw_map(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);

        for lane in &mut lanes {
            lane.traffic_light.draw(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
            lane.update(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
            // for vehicle in &mut lane.vehicles {
            //     vehicle.move_forward(WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
            //     canvas.set_draw_color(vehicle.color);
            //     let rect = Rect::new(vehicle.position.x, vehicle.position.y, VEHICULE_WIDTH, VEHICULE_WIDTH);
            //     canvas.fill_rect(rect).unwrap();
            //     // vehicle.update(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
            // }
        };

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
