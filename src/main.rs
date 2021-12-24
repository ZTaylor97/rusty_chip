use std::fs::File;
use std::io::Read;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod display;

pub struct Memory {
    ram: [u8; 4096],
    pc: usize, // program counter
    i: usize,  // index register
    stack: Vec<u16>,
    sp: usize,       // stack pointer
    delay_timer: u8, // delay timer
    sound_timer: u8, // sound timer
    v: [u8; 16],
}

pub struct Keypad {}

impl Memory {
    pub fn init() -> Memory {
        Memory {
            ram: [0; 4096],
            pc: 0x200,
            i: 0,
            stack: vec![],
            sp: 0,
            delay_timer: 255,
            sound_timer: 255,
            v: [0; 16],
        }
    }
    fn load_rom(&mut self, filename: &String) {
        let mut f = File::open(&filename).expect("no file found");
        f.read(&mut self.ram[0x200..]).expect("error reading file");
    }
}

pub struct Emulator {
    mem: Memory,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    display: display::Display,
}

impl Emulator {
    pub fn new(canvas: sdl2::render::Canvas<sdl2::video::Window>) -> Emulator {
        let mem = Memory::init();

        let mut display = display::Display::new();
        display.init_cells(
            16,
            16,
            sdl2::pixels::Color::WHITE,
            sdl2::pixels::Color::BLACK,
        );

        Emulator {
            mem,
            canvas,
            display,
        }
    }

    fn emulate_cycle(&mut self) {
        let mut opcode: u16 = self.mem.ram[self.mem.pc] as u16;

        opcode = opcode << 8 | self.mem.ram[self.mem.pc + 1] as u16;
        println!(
            "PC: {:02X?}, Current opcode: {:02X?}, i : {:02X?}",
            self.mem.pc, opcode, self.mem.i
        );
        self.mem.pc += 2;

        if opcode == 0x00E0 {
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();
            return;
        }

        match opcode & 0xF000 {
            0x1000 => (self.mem.pc = (opcode & 0x0FFF) as usize),
            0x6000 => {
                let index = (opcode & 0x0F00) >> 8;
                println!("v[{}] = {}", index, (opcode & 0x00FF));
                self.mem.v[index as usize] = (opcode & 0x00FF) as u8;
            }
            0x7000 => {
                let index = (opcode & 0x0F00) >> 8;
                println!("v[{}] += {}", index, (opcode & 0x00FF));
                self.mem.v[index as usize] += (opcode & 0x00FF) as u8;
            }
            0xA000 => {
                self.mem.i = (opcode - 0xA000) as usize;
            }
            0xD000 => {
                let x = self.mem.v[((opcode & 0x0F00) >> 8) as usize] % 64;
                let y = self.mem.v[((opcode & 0x00F0) >> 4) as usize] % 32;
                let n = opcode & 0x000F;
                //println!("Printing n {} at {{ {}, {} }}", n, x, y,);

                self.mem.v[0xF as usize] = 0;

                for i in 0..n {
                    let pixel = self.mem.ram[self.mem.i + i as usize];
                    //println!("Drawing at pixel no {}", pixel);

                    for j in 0..8 {
                        if pixel & (0x80 >> j) != 0 {
                            self.display.cells
                                [usize::from((u16::from(y) + i) * 64 + u16::from(x + j))]
                            .0 ^= true;
                        }
                    }
                }
            }
            _ => {
                println!("Instruction not implemented!");
            }
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rusty Chip", 1024, 512)
        .position_centered()
        .build()
        .unwrap();
    let canvas = window.into_canvas().build().unwrap();

    let mut emulator = Emulator::new(canvas);

    emulator.mem.load_rom(&String::from("Roms/IBMLOGO.ch8"));

    emulator.canvas.set_draw_color(Color::RGB(0, 0, 0));
    emulator.canvas.clear();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
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
        // The rest of the game loop goes here...
        for cell in emulator.display.cells {
            if cell.0 {
                emulator.canvas.set_draw_color(Color::RGB(255, 255, 255));
            } else {
                emulator.canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            emulator
                .canvas
                .fill_rect(cell.1)
                .expect("Failed to draw rects");
        }

        emulator.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    //Test file reading code
    // for i in 0x200..0x200 + 234 {
    //     println!("{}", mem.ram[mem.pc]);
    //     mem.pc += 1;
    // }
}
