
use road_intersection::*;

pub fn main() {
    let settings = Rc::new(Settings::new(800, 800, 30, 1, 60.0));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", settings.width as u32, settings.height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();

    let mut lanes = vec![
        Lane::new(Cross::First, settings.clone()),
        Lane::new(Cross::Second, settings.clone()),
        Lane::new(Cross::Third, settings.clone()),
        Lane::new(Cross::Fourth, settings.clone()),
    ];
    
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(55, 64, 5));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    handle_keyboard_event(&event, &mut lanes, settings.clone());
                }
            }
        }

        canvas.clear();
        // The rest of the game loop goes here...
        
        // map
        draw_map(&mut canvas, settings.clone());

        for lane in &mut lanes {
            lane.update(&mut canvas);
        };

        update_traffic_lights(&mut lanes);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
