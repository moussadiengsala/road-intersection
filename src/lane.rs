use std::rc::Rc;
use std::time::Duration;

use crate::cars::{Route, Vehicle};
use crate::settings::Settings;
use crate::traffic::TrafficLight;
use sdl2::{rect::Point, render::Canvas, video::Window};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cross {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Crossing,
    Waiting,
}

#[derive(Debug, Clone)]
pub struct Lane {
    pub vehicles: Vec<Vehicle>,
    pub traffic_light: TrafficLight,
    pub cross: Cross,
    pub stage: Stage,
    pub stop_point: Point,
    pub last_light_change: Instant,
    pub change_interval: Duration,
    pub settings: Rc<Settings>,
}

impl Lane {
    pub fn new(cross: Cross, settings: Rc<Settings>) -> Lane {
        Lane {
            vehicles: Vec::new(),
            traffic_light: TrafficLight::new(cross),
            cross,
            stage: Stage::Waiting,
            stop_point: match cross {
                Cross::First => settings.stop_point_first,
                Cross::Second => settings.stop_point_second,
                Cross::Third => settings.stop_point_third,
                Cross::Fourth => settings.stop_point_fourth,
            },
            last_light_change: Instant::now(),
            change_interval: Duration::from_secs(15),
            settings,
        }
    }

    pub fn draw_light(mut self, canvas: &mut Canvas<Window>) {
        self.traffic_light.draw(
            canvas,
            self.settings.width,
            self.settings.height,
            self.settings.vehicle,
        );
    }

    pub fn closest_vehicle_distance(&self) -> Option<f64> {
        self.vehicles
            .iter()
            .map(|vehicle| vehicle.distance_to(self.stop_point))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn stop_vehicules(&mut self) {
        let stop_point = match self.cross {
            Cross::First => self.settings.stop_point_first,
            Cross::Second => self.settings.stop_point_second,
            Cross::Third => self.settings.stop_point_third,
            Cross::Fourth => self.settings.stop_point_fourth,
        };

        let mut vehicles = self.vehicles.iter_mut().collect::<Vec<&mut Vehicle>>();
        for i in 0..vehicles.len() {
            let can_move = if let Some(next_vehicle) = vehicles.iter().nth((i as i32 - 1) as usize)
            {
                vehicles[i].distance(next_vehicle) > self.settings.safety_distance
            } else {
                true
            };

            if (vehicles[i].position == stop_point && self.stage == Stage::Waiting) || !can_move {
                vehicles[i].is_stopped = true;
            }
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        <Lane as Clone>::clone(&self).draw_light(canvas);
        self.stop_vehicules();

        for i in (0..self.vehicles.len()).rev() {
            self.vehicles[i].update(canvas);

            // Remove vehicles that have reached the end of the lane
            if self.vehicles[i].has_reached_end() {
                self.vehicles.remove(i);
            }
        }
    }

    pub fn add_vehicle(&mut self, route: Route) {
        
        let mut vehicle = Vehicle::new(route, 1, self.settings.clone(), self.stop_point);
        vehicle.spawn(route);

        if let Some(last) = self.vehicles.clone().last() {
            if self.settings.safety_distance < vehicle.distance(last) {
                self.vehicles.push(vehicle);
            }
        } else {
            self.vehicles.push(vehicle);
        }

        // println!("route {:?} len {}", self.cross, self.vehicles.len());
    }
}
