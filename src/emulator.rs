use std::fs::File;
use std::io::Read;

extern crate sdl2;

use super::display;

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
}

pub struct Emulator<'a> {
    pub mem: Memory,
    pub display: display::Display<'a>,
}

impl Emulator<'_> {
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Emulator {
        let mem = Memory::init();

        let mut display = display::Display::new(canvas);
        display.init_cells(
            16,
            16,
            sdl2::pixels::Color::WHITE,
            sdl2::pixels::Color::BLACK,
        );

        Emulator { mem, display }
    }

    pub fn load_rom(&mut self, filename: &String) {
        let mut f = File::open(&filename).expect("no file found");
        f.read(&mut self.mem.ram[0x200..])
            .expect("error reading file");
    }

    pub fn emulate_cycle(&mut self) {
        let mut opcode: u16 = self.mem.ram[self.mem.pc] as u16;

        opcode = opcode << 8 | self.mem.ram[self.mem.pc + 1] as u16;
        // println!(
        //     "PC: {:02X?}, Current opcode: {:02X?}, i : {:02X?}",
        //     self.mem.pc, opcode, self.mem.i
        // );

        self.mem.pc += 2;

        if opcode == 0x00E0 {
            self.display.clear_screen();
            return;
        }

        // Return from subroutine
        if opcode == 0x00EE {
            self.mem.pc = match self.mem.stack.pop() {
                Some(x) => (x as usize),
                None => (panic!("Error: Stack empty!")),
            };
            return;
        }

        match opcode & 0xF000 {
            0x1000 => (self.mem.pc = (opcode & 0x0FFF) as usize),
            0x2000 => {
                self.mem.stack.push(self.mem.pc as u16);
                self.mem.pc = (opcode & 0x0FFF) as usize;
            }
            0x6000 => {
                let index = (opcode & 0x0F00) >> 8;
                //println!("v[{}] = {}", index, (opcode & 0x00FF));
                self.mem.v[index as usize] = (opcode & 0x00FF) as u8;
            }
            0x7000 => {
                let index = (opcode & 0x0F00) >> 8;
                //println!("v[{}] += {}", index, (opcode & 0x00FF));
                self.mem.v[index as usize] =
                    u8::wrapping_add(self.mem.v[index as usize], (opcode & 0x00FF) as u8);
            }
            0x8000 => {
                let x = (opcode & 0x0F00) >> 8;
                let y = (opcode & 0x00F0) >> 4;

                match opcode & 0x000F {
                    0x0 => {
                        self.mem.v[usize::from(x)] = self.mem.v[usize::from(y)];
                    }
                    0x1 => {
                        self.mem.v[usize::from(x)] |= self.mem.v[usize::from(y)];
                    }
                    0x2 => {
                        self.mem.v[usize::from(x)] &= self.mem.v[usize::from(y)];
                    }
                    0x3 => {
                        self.mem.v[usize::from(x)] ^= self.mem.v[usize::from(y)];
                    }
                    0x4 => {
                        if (self.mem.v[usize::from(x)] + self.mem.v[usize::from(y)]) > 254 {
                            self.mem.v[0xF] = 1;
                        } else {
                            self.mem.v[0xF] = 0;
                        }

                        self.mem.v[usize::from(x)] = u8::wrapping_add(
                            self.mem.v[usize::from(x)],
                            self.mem.v[usize::from(y)],
                        );
                    }
                    0x5 => {
                        if self.mem.v[usize::from(x)] > self.mem.v[usize::from(y)] {
                            self.mem.v[0xF] = 1;
                        } else {
                            self.mem.v[0xF] = 0;
                        }
                        self.mem.v[usize::from(x)] = u8::wrapping_sub(
                            self.mem.v[usize::from(x)],
                            self.mem.v[usize::from(y)],
                        );
                    }
                    0x6 => {
                        panic!("Warning, Shift not implemented");
                    }
                    0x7 => {
                        if self.mem.v[usize::from(y)] > self.mem.v[usize::from(x)] {
                            self.mem.v[0xF] = 1;
                        } else {
                            self.mem.v[0xF] = 0;
                        }
                        self.mem.v[usize::from(x)] = u8::wrapping_sub(
                            self.mem.v[usize::from(y)],
                            self.mem.v[usize::from(x)],
                        );
                    }
                    _ => (panic!("Instruction: {} is invalid.", opcode)),
                }
            }
            0xA000 => {
                self.mem.i = (opcode - 0xA000) as usize;
            }
            0xD000 => {
                // Extract coordinates and n from opcode
                let x = self.mem.v[((opcode & 0x0F00) >> 8) as usize] % 64;
                let y = self.mem.v[((opcode & 0x00F0) >> 4) as usize] % 32;
                let n = opcode & 0x000F;
                //println!("Printing n {} at {{ {}, {} }}", n, x, y,);

                // Reset V[F]
                self.mem.v[0xF as usize] = 0;

                for i in 0..n {
                    // extract byte of pixels from RAM for this row
                    let pixel = self.mem.ram[self.mem.i + i as usize];

                    for j in 0..8 {
                        // If pixel in memory is on
                        if pixel & (0b1000_0000 >> j) != 0 {
                            // If pixel is active, set V[F] register to show that it was flipped back off (collision detected)
                            if self.display.cells
                                [usize::from((u16::from(y) + i) * 64 + u16::from(x + j))]
                            .0
                            {
                                self.mem.v[0xF] = 1;
                            }
                            // FLip pixel's state
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

    pub fn draw(&mut self) {
        for cell in self.display.cells {
            self.display.draw_rect_to_canvas(&cell);
        }

        self.display.draw_canvas();
    }
}
