use sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::{rc::Rc, time::Duration};

mod settings;
use settings::Settings;

mod map;
use map::draw_map;

mod traffic;

mod cars;
use cars::handle_keyboard_event;

mod lane;
use lane::{Lane, Cross};

pub fn main() {
    let settings = Rc::new(Settings::new(800, 800, 30, 1, 50));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", settings.width as u32, settings.height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let mut lanes = vec![
        Lane::new(Cross::First, settings.clone()),
        Lane::new(Cross::Second, settings.clone()),
        Lane::new(Cross::Third, settings.clone()),
        Lane::new(Cross::Fourth, settings.clone()),
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
                    handle_keyboard_event(&event, &mut lanes, settings.clone());
                }
            }
        }

        canvas.clear();
        // The rest of the game loop goes here...
        
        // map
        draw_map(&mut canvas, settings.clone());

        for lane in &mut lanes {
            // lane.traffic_light.draw(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);
            // lane.update(&mut canvas, WIDTH as i32, HEIGHT as i32, VEHICULE_WIDTH as i32);



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
