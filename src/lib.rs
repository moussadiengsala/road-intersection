use lane::Stage;
use rand::Rng;
pub use sdl2;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
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
    if lanes.iter().any(|v| v.stage == Stage::Crossing) {
        return;
    }

    let mut next_cross_lane = None;
    let mut min_distance = f64::MAX;

    for lane in lanes.iter_mut() {
        let a = lane
            .vehicles
            .iter()
            .filter(|v| v.stage == Stage::Waiting)
            .collect::<Vec<&Vehicle>>();
        if !a.is_empty() {
            if let Some(distance) = lane.closest_vehicle_distance() {
                if distance < min_distance {
                    min_distance = distance;
                    next_cross_lane = Some(lane);
                }
            }
        }
    }

    if let Some(lane) = next_cross_lane {
        lane.traffic_light.change_traffic_light();
        lane.stage = Stage::Crossing;
    }
}

// pub fn update_traffic_lights(lanes: &mut [Lane]) {
// let mut cross_lane = None;
// let mut next_cross_lane = None;
// let mut min_distance = f64::MAX;

// for lane in lanes.iter_mut() {
//     if lane.stage == Stage::Crossing {
//         cross_lane = Some(lane);
//         continue;
//     }

//     let a = lane.vehicles.iter().filter(|v| v.stage == Stage::Waiting).collect::<Vec<&Vehicle>>();
//     if !a.is_empty() && lane.stage == Stage::Waiting {
//         if let Some(distance) = lane.closest_vehicle_distance() {
//             if distance < min_distance {
//                 min_distance = distance;
//                 next_cross_lane = Some(lane);
//             }
//         }
//     }
// }

// if let Some(lane) = next_cross_lane {
//     if let Some(lane) = cross_lane {
//         let vehicles = lane.vehicles.iter().filter(|v| v.stage == Stage::Waiting).collect::<Vec<&Vehicle>>();
//         if let Some(vehicle) = vehicles.first() {
//             if vehicle.distance_to(lane.stop_point) > 2.0 * lane.settings.safety_distance {
//                 lane.traffic_light.change_traffic_light();
//                 lane.stage = Stage::Waiting;
//             } else {
//                 return;
//             }
//         } else {
//             lane.traffic_light.change_traffic_light();
//             lane.stage = Stage::Waiting;
//             println!("helooooooooooooooooooooooooooooooooooo")
//         }
//     } else {
//         lane.traffic_light.change_traffic_light();
//         lane.stage = Stage::Crossing;
//     }
// }
// }
