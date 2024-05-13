#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub width: i32,
    pub height: i32,
    pub vehicle_width: i32,
    pub gap: i32,
    pub safety_distance: i32,

    pub horizontal_road_1: i32,
    pub vertical_road_1: i32,
    pub horizontal_road_2: i32,
    pub vertical_road_2: i32,
}

impl Settings {
    pub fn new(width: i32, height: i32, vehicle: i32, gap: i32, safety_distance: i32) -> Settings {
        let half_width = width / 2;
        let half_height = height / 2;
        let vehicle_width = 2 * vehicle;
        let offset_road = gap + vehicle_width;

        Self {
            width,
            height,
            vehicle_width,
            gap,
            safety_distance,

            horizontal_road_1: half_height - offset_road,
            vertical_road_1: half_width - offset_road,
            horizontal_road_2: half_height + offset_road,
            vertical_road_2: half_width + offset_road,
        }
    }
}
