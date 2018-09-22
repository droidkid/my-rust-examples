extern crate sdl2;

use std::thread;
use std::time::Duration;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust sdl2", 800, 600)
                                .position_centered()
                                .opengl()
                                .build()
                                .unwrap();

    thread::sleep(Duration::new(10, 0));

}
