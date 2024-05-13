use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

pub struct Path {
    pub start: Point,
    pub end: Point,
}

impl Path {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Path {
        Path { start: Point::new(start.0, start.1), end: Point::new(end.0, end.1) }
    }
}

pub fn draw_map(canvas: &mut Canvas<Window>, width: i32, height: i32, vehicle: i32) {
    let half_width = width / 2;
    let half_height = height / 2;
    let vehicle_width = 2*vehicle;
    let gap = 1;

    let a = half_height - gap - vehicle_width;
    let b = half_width - gap - vehicle_width;
    let c = half_height + gap + vehicle_width;
    let d = half_width + gap + vehicle_width;

    let roads = [
        Path::new((0, a), (b, a)),
        Path::new((b, 0), (b, a)),

        Path::new((0, c), (b, c)),
        Path::new((b, height), (b, c)),

        Path::new((width, a), (d, a)),
        Path::new((d, 0), (d, a)),
        
        Path::new((width, c), (d, c)),
        Path::new((d, height), (d, c)),

        Path::new((0, half_height), (b, half_height)),
        Path::new((half_width, height), (half_width, c)),
        Path::new((width, half_height), (d, half_height)),
        Path::new((half_width, 0), (half_width, half_height - vehicle_width)),
    ];

    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Set line color
    for road in &roads {
        canvas.draw_line(road.start, road.end).unwrap(); // Draw lines
    }
}

