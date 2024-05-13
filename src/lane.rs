use std::rc::Rc;
use std::time::Duration;

use sdl2::rect::Rect;
use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};
use crate::cars::{Vehicle, Route};
use crate::settings::Settings;
use crate::traffic::TrafficLight;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
pub enum Cross {
    First,
    Second,
    Third,
    Fourth,
}


#[derive(Debug, Clone)]
pub struct Lane {
    pub vehicles: Vec<Vehicle>,
    pub traffic_light: TrafficLight,
    pub cross: Cross,
    pub is_vehicles_stopped: bool,
    pub last_light_change: Instant,
    pub change_interval: Duration,
    settings: Rc<Settings>
}

impl Lane {
    pub fn new(cross: Cross,  settings: Rc<Settings>) -> Lane {
        Lane {
            vehicles: Vec::new(),
            traffic_light: TrafficLight::new(cross),
            cross,
            is_vehicles_stopped: false,
            last_light_change: Instant::now(), // Initialize with the current time
            change_interval: Duration::from_secs(15), // Change light every 15 seconds
            settings
        }
    }

    pub fn draw(mut self, canvas: &mut Canvas<Window>) {
        self.traffic_light.draw(canvas, self.settings.width, self.settings.height, self.settings.vehicle_width);
    }

    pub fn stopped_coordinate(&mut self) {
        let (x1, x2) = (
            (self.settings.width / 2) - 2 * self.settings.vehicle_width / 2,
            (self.settings.width / 2) + self.settings.vehicle_width,
        );
        let (y1, y2) = (
            self.settings.height / 2 + self.settings.vehicle_width / 2,
            self.settings.height / 2 - 2 * self.settings.vehicle_width + self.settings.vehicle_width / 2,
        );

        match self.cross {
            Cross::First => {
                for (i, vehicle) in self.vehicles.iter_mut().enumerate() {
                    if vehicle.position.y == y1 {
                        self.is_vehicles_stopped = true;
                    }
                }
            },
            Cross::Second => {
                for (i, vehicle) in self.vehicles.iter_mut().enumerate() {
                    if vehicle.position.x == x1 {
                        self.is_vehicles_stopped = true;
                    }
                }
            },
            Cross::Third => {
                for (i, vehicle) in self.vehicles.iter_mut().enumerate() {
                    if vehicle.position.x == x2 {
                        self.is_vehicles_stopped = true;
                    }
                }
            },
            Cross::Fourth => {
                for (i, vehicle) in self.vehicles.iter_mut().enumerate() {
                    if vehicle.position.y == y2 {
                        self.is_vehicles_stopped = true;
                    }
                }
            },
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        // Update vehicles
        for (i, vehicle) in self.vehicles.iter_mut().enumerate() {
            // self.stopped_coordinate(width, height, vehicle_width);
            // Get the position of the previous vehicle
            // let mut prev_vehicle_position = if i > 0 {
            //     &self.vehicles[i - 1].position
            // } else {
            //     &vehicle.position
            // };

            // Ensure safety distance between vehicles
            // let min_distance = self.vehicle_spacing + vehicle_width; // Safety distance + vehicle width
            // let current_distance = 50;
            // if current_distance < min_distance {
            //     vehicle.stop(); // Stop if too close to the previous vehicle
            // } else {
            //     vehicle.resume(); // Resume movement if safe distance is maintained
            // }

            // // Stop at red traffic light

            // if self.traffic_light.color == Color::RED && self.is_vehicles_stopped {
            //     vehicle.stop();
            // } else {
            //     vehicle.resume();
            // }

        
            if self.last_light_change.elapsed() >= self.change_interval {
                // self.traffic_light.change_traffic_light(canvas);
                self.last_light_change = Instant::now(); // Reset the last light change time
            }

            // Move the vehicle forward
            vehicle.move_forward();
            canvas.set_draw_color(vehicle.color);
            let rect = Rect::new(vehicle.position.x, vehicle.position.y, self.settings.width as u32, self.settings.width as u32);
            canvas.fill_rect(rect).unwrap();

            // Remove vehicles that have reached the end of the lane
            // if vehicle.has_reached_end(canvas_width, canvas_height, vehicle_width) {
            //     self.vehicles.remove(i);
            // }
        }

        // Update traffic light
        // This logic can be implemented based on a timer or a specific algorithm to change the traffic light color.
        // For simplicity, let's just alternate between red and green every few seconds.
        // We can add a timer or counter to keep track of time and change the traffic light color accordingly.
        // For now, let's just toggle the traffic light color every 100 frames.
        // You may adjust this logic based on your requirements.
        // if frame_count % 100 == 0 {
        //     self.traffic_light.change_traffic_light();
        // }

        // Note: You need to implement a way to track frame count and call this update method accordingly.
    }

    pub fn add_vehicle(&mut self, route: Route) {
        let mut vehicle = Vehicle::new(route, 1, self.settings.clone());
        vehicle.spawn(route);
        self.vehicles.push(vehicle);
    }
}
