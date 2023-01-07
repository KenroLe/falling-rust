extern crate minifb;

use minifb::{Key, Window, WindowOptions};

mod engine;
pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 160;
pub const WINDOW_WIDTH:usize = WIDTH*4;
pub const WINDOW_HEIGHT:usize = HEIGHT*4;
pub const WRATIO :usize= WINDOW_WIDTH/WIDTH;
pub const HRATIO :usize= WINDOW_HEIGHT/HEIGHT;
use log::{info};
fn main() {
    env_logger::init();
    info!("starting up");
    let mut world: Vec<Vec<engine::Point>> = vec![ vec![engine::EMPTY_POINT; HEIGHT]; WIDTH]; // world[width][height]
    let mut render_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate with from_micros(16600)
    window.limit_update_rate(Some(std::time::Duration::from_micros(25000)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        engine::physics::tick(&mut world);
        engine::game::tick(&mut world, &window);
        
        for x in 0..world.len() {
            for y in 0..world[x].len() {
                render_buffer[x+y*WIDTH] =  world[x][y].color;
            }
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&render_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

