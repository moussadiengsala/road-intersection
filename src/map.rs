use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

pub struct Path {
    pub start: Point,
    pub end: Point,
}

impl Path {
    pub fn new(start: Point, end: Point) -> Path {
        Path { start, end }
    }
}

pub fn draw_map(canvas: &mut Canvas<Window>, width: i32, height: i32, vehicle: i32) {
    let half_width = width / 2;
    let half_height = height / 2;
    let vehicle_width = 2*vehicle;
    let gap = 1;

    let roads = [
        Path::new(Point::new(0, half_height - gap - vehicle_width), Point::new(half_width - gap - vehicle_width, half_height - gap - vehicle_width)),
        Path::new(Point::new(half_width - gap - vehicle_width, 0), Point::new(half_width - gap - vehicle_width, half_height - gap - vehicle_width)),

        Path::new(Point::new(0, half_height + gap + vehicle_width), Point::new(half_width - gap - vehicle_width, half_height + gap + vehicle_width)),
        Path::new(Point::new(half_width - gap - vehicle_width, height), Point::new(half_width - gap - vehicle_width, half_height + gap + vehicle_width)),

        Path::new(Point::new(width, half_height - gap - vehicle_width), Point::new(half_width + gap + vehicle_width, half_height - gap - vehicle_width)),
        Path::new(Point::new(half_width + gap + vehicle_width, 0), Point::new(half_width + gap + vehicle_width, half_height - gap - vehicle_width)),
        
        Path::new(Point::new(width, half_height + gap + vehicle_width), Point::new(half_width + gap + vehicle_width, half_height + gap + vehicle_width)),
        Path::new(Point::new(half_width + gap + vehicle_width, height), Point::new(half_width + gap + vehicle_width, half_height + gap + vehicle_width)),

        Path::new(Point::new(0, half_height), Point::new(half_width - gap - vehicle_width, half_height)),
        Path::new(Point::new(half_width, height), Point::new(half_width, half_height + gap + vehicle_width)),
        Path::new(Point::new(width, half_height), Point::new(half_width + gap + vehicle_width, half_height)),
        Path::new(Point::new(half_width, 0), Point::new(half_width, half_height - vehicle_width)),
    ];

    canvas.set_draw_color(Color::RGB(255, 255, 255)); // Set line color
    for road in &roads {
        canvas.draw_line(road.start, road.end).unwrap(); // Draw lines
    }
}

