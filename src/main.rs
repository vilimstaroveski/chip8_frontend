use std::env;
use std::ptr::read;
use chip8_core::*;
use sdl2::event::Event;
use std::fs::File;
use std::io::{BufReader, Read};

const SCALE: u32 = 10;
const WINDOW_HEIGHT: u32 = SCALE * SCREEN_HEIGHT as u32;
const WINDOW_WIDTH: u32 = SCALE * SCREEN_WIDTH as u32;


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let game_file_path = &args[1];

    let sdl_context = sdl2::init().expect("Error in sdl initialization");
    let video_subsystem = sdl_context.video().expect("Error in video subsystem initialization");
    let window = video_subsystem.window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().opengl().build().expect("Error in window initialization");
    let mut canvas = window.into_canvas().present_vsync().build().expect("Error in canvas initialization");

    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().expect("Error in event pump initialization");

    let mut chip8 = Emu::new();

    let mut game_file = File::open(game_file_path).expect("Unable to open file");
    let mut buffer = Vec::new();
    game_file.read_to_end(&mut buffer).expect("Error in reading the game file");
    chip8.load(&buffer);



    'gameloop: loop {
        for evt in event_pump.poll_iter() {
            match evt {
                Event::Quit{..} => {
                    break 'gameloop;
                },
                _ => ()
            }
        }

        chip8.tick();
    }
}
