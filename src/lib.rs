use lane::Stage;
use rand::Rng;
pub use sdl2;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
use std::{cmp::Ordering, time::Instant};
pub use std::{rc::Rc, time::Duration};

mod settings;
pub use settings::Settings;

mod map;
pub use map::draw_map;

mod traffic;

mod cars;
pub use cars::{Route, Vehicle};

mod lane;
pub use lane::{Cross, Lane};

pub fn handle_keyboard_event(event: &Event, lanes: &mut Vec<Lane>, settings: Rc<Settings>) {
    let mut binding = Lane::new(Cross::First, settings);
    let (lane, route) = match event {
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => (lanes.iter_mut().nth(3).unwrap(), Route::Up),
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => (lanes.iter_mut().nth(0).unwrap(), Route::Down),
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => (lanes.iter_mut().nth(2).unwrap(), Route::Left),
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => (lanes.iter_mut().nth(1).unwrap(), Route::Right),
        Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
        } => {
            let mut rng = rand::thread_rng();
            match rng.gen_range(0, 4) {
                0 => (lanes.iter_mut().nth(3).unwrap(), Route::Up),
                1 => (lanes.iter_mut().nth(0).unwrap(), Route::Down),
                2 => (lanes.iter_mut().nth(2).unwrap(), Route::Left),
                _ => (lanes.iter_mut().nth(1).unwrap(), Route::Right),
            }
        }
        _ => (&mut binding, Route::None),
    };

    if route != Route::None {
        lane.add_vehicle(route)
    }
}

pub fn update_traffic_lights(lanes: &mut [Lane]) {
    // Check if any lane is currently in the "Crossing" stage
    if lanes.iter().any(|lane| lane.stage == Stage::Crossing) {
        return;
    }

    let current_time = Instant::now();

    // Sort lanes by the time of the last light change
    lanes.sort_by(|a, b| a.last_light_change.cmp(&b.last_light_change));

    // Find the lane with the closest vehicle to the stop point among the top three lanes
    let lane_with_closest_vehicle = lanes.iter_mut()
        .min_by(|a, b| {
            a.closest_vehicle_distance()
                .unwrap_or(f64::MAX)
                .partial_cmp(&b.closest_vehicle_distance().unwrap_or(f64::MAX))
                .unwrap_or(Ordering::Equal)
        });

    // If a suitable lane is found, update its traffic light and reset its timer
    if let Some(lane_with_closest_vehicle) = lane_with_closest_vehicle {
        lane_with_closest_vehicle.traffic_light.change_traffic_light();
        lane_with_closest_vehicle.last_light_change = current_time;
        lane_with_closest_vehicle.stage = Stage::Crossing;
    }
}
