extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod display;
mod emulator;

// TODO: Refactor SDL code into display module and remove canvas from emulator module (canvas should be in display)
// TODO: Refactor emulator into it's own module

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = match video_subsystem
        .window("Rusty Chip", 1024, 512)
        .position_centered()
        .build()
    {
        Ok(window) => window,
        Err(error) => panic!("Problem initialising the window: {:?}", error),
    };

    let mut canvas = match window.into_canvas().build() {
        Ok(canvas) => canvas,
        Err(error) => panic!("Problem creating canvas: {:?}", error),
    };

    let mut emulator = emulator::Emulator::new(&mut canvas);
    emulator.load_rom(&String::from("Roms/IBMLOGO.ch8"));

    emulator.display.clear_screen();

    // TODO: Refactor into sdl_context etc into a separate module so you can just call something like
    // sdl_context.poll_events or something
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // While running:
        // poll_events
        // emulate_cycle
        //

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        emulator.emulate_cycle();
        emulator.draw();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 700));
    }
}
