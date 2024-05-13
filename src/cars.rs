use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{event::Event, pixels::Color, rect::Point};

use crate::lane::Lane;

#[derive(Debug, Clone, Copy)]
pub struct Vehicle {
    pub position: Point,
    pub color: Color,
    pub route: Route,
    pub destination: Route,
    pub velocity: i32,
    pub is_changed_direction: bool,
    pub is_stopped: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Route {
    Up,
    Down,
    Left,
    Right,
}


impl Vehicle {
    pub fn new(route: Route, velocity: i32) -> Self {
        let (color, destination) = Self::random(route);
        Self {
            position: Point::new(0, 0),
            color,
            destination,
            route,
            velocity,
            is_changed_direction: false,
            is_stopped: false,
        }
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
    }

    pub fn resume(&mut self) {
        self.is_stopped = false;
    }

    pub fn random(route: Route) -> (Color, Route) {
        let mut rng = rand::thread_rng();
        let color = match rng.gen_range(0, 3) {
            0 => Color::GREEN,
            1 => Color::BLUE,
            _ => Color::YELLOW,
        };

        let destination = match color {
            Color::YELLOW => match route {
                Route::Up => Route::Left,
                Route::Down => Route::Right,
                Route::Left => Route::Down,
                _ => Route::Up,
            },
            Color::BLUE => route,
            Color::GREEN => match route {
                Route::Up => Route::Right,
                Route::Down => Route::Left,
                Route::Right => Route::Right,
                _ => Route::Up,
            },
            _ => Route::Up, // Default route for other colors
        };
        (color, destination)
    }

    pub fn spawn(
        &mut self,
        direction: Route,
        canvas_width: i32,
        canvas_height: i32,
        vehicle_width: i32,
    ) {
        match direction {
            Route::Up => {
                self.position.x = (canvas_width / 2) + vehicle_width / 2;
                self.position.y = canvas_height;
            }
            Route::Down => {
                self.position.x = (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2;
                self.position.y = -vehicle_width;
            }
            Route::Left => {
                self.position.x = canvas_width;
                self.position.y = canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2;
            }
            Route::Right => {
                self.position.x = -vehicle_width;
                self.position.y = canvas_height / 2 + vehicle_width / 2;
            }
        }
    }

    pub fn move_forward(&mut self, canvas_width: i32, canvas_height: i32, vehicle_width: i32) {
        if self.is_stopped {
            return;
        };

        let (x1, x2) = (
            (canvas_width / 2) - 2 * vehicle_width + vehicle_width / 2,
            (canvas_width / 2) + vehicle_width / 2,
        );
        let (y1, y2) = (
            canvas_height / 2 + vehicle_width / 2,
            canvas_height / 2 - 2 * vehicle_width + vehicle_width / 2,
        );
        match self.route {
            Route::Up => {
                if !self.is_changed_direction {
                    self.position.y -= self.velocity
                } else {
                    let d = if self.destination == Route::Left {
                        -1
                    } else {
                        1
                    };
                    self.position.x += d * self.velocity;
                };

                if (self.position.y == y2) && (self.destination == Route::Left)
                    || self.destination == Route::Right && (self.position.y == y1)
                {
                    self.is_changed_direction = true;
                };
            }
            Route::Down => {
                if !self.is_changed_direction {
                    self.position.y += self.velocity
                } else {
                    let d = if self.destination == Route::Left {
                        -1
                    } else {
                        1
                    };
                    self.position.x += d * self.velocity;
                };

                if (self.position.y == y1) && (self.destination == Route::Right)
                    || (self.position.y == y2) && self.destination == Route::Left
                {
                    self.is_changed_direction = true;
                };
            }
            Route::Left => {
                if !self.is_changed_direction {
                    self.position.x -= self.velocity
                } else {
                    let d = if self.destination == Route::Down {
                        1
                    } else {
                        -1
                    };
                    self.position.y += d * self.velocity;
                };

                if self.destination == Route::Down && self.position.x == x1
                    || self.destination == Route::Up && self.position.x == x2
                {
                    self.is_changed_direction = true;
                };
            }
            Route::Right => {
                if !self.is_changed_direction {
                    self.position.x += self.velocity
                } else {
                    self.position.y -= self.velocity;
                };

                if self.destination == Route::Up && self.position.x == x2 {
                    self.is_changed_direction = true;
                };
            }
        }
    }
}

pub fn handle_keyboard_event(
    event: &Event,
    lanes: &mut Vec<Lane>,
    canvas_width: i32,
    canvas_height: i32,
    vehicle_width: i32,
) {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Up, 1);
            vehicle.spawn(Route::Up, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(3) {
                lane.vehicles.push(vehicle);
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Down, 1);
            vehicle.spawn(Route::Down, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(0) {
                lane.vehicles.push(vehicle);
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Left, 1);
            vehicle.spawn(Route::Left, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(2) {
                lane.vehicles.push(vehicle);
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => {
            let mut vehicle = Vehicle::new(Route::Right, 1);
            vehicle.spawn(Route::Right, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = lanes.iter_mut().nth(1) {
                lane.vehicles.push(vehicle);
            }
        }
        Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
        } => {
            let mut rng = rand::thread_rng();
            let random_route = match rng.gen_range(0, 4) {
                0 => Route::Up,
                1 => Route::Down,
                2 => Route::Left,
                _ => Route::Right,
            };
            let mut vehicle = Vehicle::new(random_route, 1);
            vehicle.spawn(random_route, canvas_width, canvas_height, vehicle_width);
            if let Some(lane) = match random_route {
                Route::Up => lanes.iter_mut().nth(3),
                Route::Down => lanes.iter_mut().nth(0),
                Route::Left => lanes.iter_mut().nth(2),
                Route::Right => lanes.iter_mut().nth(1),
            } {
                lane.vehicles.push(vehicle);
            }
        }
        _ => {}
    }
}

// Yellow
/*
    route: UP => destination: TurnLeft
    route: Down => destination: TurnRight
    route: Left => destination: TurnDown
    route: Right => destination: TurnUp
*/

// Blue: still forward
/*
    route: UP => destination: GoUP
    route: Down => destination: GoDown
    route: Left => destination: goLeft
    route: Right => destination: GoRight
*/

// GREEN
/*
    route: Up => destination: TurnRight
    route: Down => destination: GoLeft
    route: Left => destination: goUp
    route: Right => destination: GoRight
*/
