
use sdl2::{pixels::Color, render::Canvas, video::Window};
use crate::map::Path;

use crate::lane::Cross;

#[derive(Debug, Clone, Copy)]
pub struct TrafficLight {
    pub light: Cross,
    pub color: Color,
}

impl TrafficLight {
    pub fn new(light: Cross) -> TrafficLight {
        Self{light, color: Color::RED}
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, width: i32, height: i32, vehicle: i32) {
        let half_width = width / 2;
        let half_height = height / 2;
        let vehicle_width = 2*vehicle;
        let gap = 5;

        let (x1, x2, x3, x4) = (half_width - vehicle_width - 10 * gap, half_width - vehicle_width - gap, half_width + vehicle_width + gap, half_width + vehicle_width + 10 * gap);
        let (y1, y2, y3, y4) = (half_height  - vehicle_width - 10 * gap, half_height  - vehicle_width - gap, half_height  + vehicle_width + gap, half_height  + vehicle_width + 10 * gap);

        canvas.set_draw_color(self.color);
        let points = match self.light {
            Cross::First => [
                Path::new((x1, y2), (x1, y1)),
                Path::new((x2, y1), (x1, y1)),
            ],
            Cross::Second => [
                Path::new((x1, y3), (x1, y4)),
                Path::new((x1, y4), (x2, y4)),
            ],
            Cross::Third => [
                Path::new((x4, y2), (x4, y1)),
                Path::new((x3, y1), (x4, y1)),
            ],
            Cross::Fourth => [
                Path::new((x4, y3), (x4, y4)),
                Path::new((x4, y4), (x3, y4)),
            ],
        };

        for point in points {
            canvas.draw_line(point.start, point.end).unwrap();
        };
    }

    pub fn change_traffic_light(&mut self) {
        match self.color {
            Color::RED => self.color = Color::GREEN,
            Color::GREEN => self.color = Color::RED,
            _ => ()
        };
        // self.draw(canvas, width, height, vehicle);
    }
}