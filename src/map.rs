use std::rc::Rc;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::settings::Settings;

pub struct Path {
    pub start: Point,
    pub end: Point,
}

impl Path {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Path {
        Path { start: Point::new(start.0, start.1), end: Point::new(end.0, end.1) }
    }
}

pub fn draw_map(canvas: &mut Canvas<Window>, settings: Rc<Settings>) {

    let roads = [
        Path::new((0, settings.horizontal_road_1), (settings.vertical_road_1, settings.horizontal_road_1)),
        Path::new((settings.vertical_road_1, 0), (settings.vertical_road_1, settings.horizontal_road_1)),

        Path::new((0, settings.horizontal_road_2), (settings.vertical_road_1, settings.horizontal_road_2)),
        Path::new((settings.vertical_road_1, settings.height), (settings.vertical_road_1, settings.horizontal_road_2)),

        Path::new((settings.width, settings.horizontal_road_1), (settings.vertical_road_2, settings.horizontal_road_1)),
        Path::new((settings.vertical_road_2, 0), (settings.vertical_road_2, settings.horizontal_road_1)),
        
        Path::new((settings.width, settings.horizontal_road_2), (settings.vertical_road_2, settings.horizontal_road_2)),
        Path::new((settings.vertical_road_2, settings.height), (settings.vertical_road_2, settings.horizontal_road_2)),

        Path::new((0, settings.height / 2), (settings.vertical_road_1, settings.height / 2)),
        Path::new((settings.width / 2, settings.height), (settings.width / 2, settings.horizontal_road_2)),
        Path::new((settings.width, settings.height / 2), (settings.vertical_road_2, settings.height / 2)),
        Path::new((settings.width / 2, 0), (settings.width / 2, settings.height / 2 - settings.offset_road)),
    ];

    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Set line color
    for road in &roads {
        canvas.draw_line(road.start, road.end).unwrap(); // Draw lines
    }
}

