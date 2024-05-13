

pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub vehicle_width: u32,
    gap: u32,
    
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Settings {
    pub fn new(width: u32, height: u32, vehicle_width: u32, gap: u32) -> Settings {
        let half_width = width / 2;
        let half_height = height / 2;
        let vehicle_width = 2*vehicle;

        Self {
            width,
            height,
            vector_width,
            gap,
            
            a: half_height - gap - vehicle_width,
            b: half_width - gap - vehicle_width,
            c: half_height + gap + vehicle_width,
            d: half_width + gap + vehicle_width,
        }
    }
}