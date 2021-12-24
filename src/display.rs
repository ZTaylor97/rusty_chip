extern crate sdl2;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Display {
    pub cells: [(bool, sdl2::rect::Rect); SCREEN_WIDTH * SCREEN_HEIGHT],
    pub cell_colour_on: sdl2::pixels::Color,
    pub cell_colour_off: sdl2::pixels::Color,
}

impl Display {
    pub fn new() -> Display {
        Display {
            cells: [(false, sdl2::rect::Rect::new(0, 0, 0, 0)); SCREEN_WIDTH * SCREEN_HEIGHT],
            cell_colour_on: sdl2::pixels::Color::WHITE,
            cell_colour_off: sdl2::pixels::Color::BLACK,
        }
    }
    pub fn init_cells(
        &mut self,
        cell_width: u32,
        cell_height: u32,
        cell_colour_on: sdl2::pixels::Color,
        cell_colour_off: sdl2::pixels::Color,
    ) {
        for i in 0..SCREEN_WIDTH * SCREEN_HEIGHT {
            self.cells[i] = (
                false,
                sdl2::rect::Rect::new(
                    ((i % SCREEN_WIDTH) * cell_width as usize) as i32,
                    ((i / SCREEN_WIDTH) * cell_width as usize) as i32,
                    cell_width,
                    cell_height,
                ),
            );
        }
        self.cell_colour_on = cell_colour_on;
        self.cell_colour_off = cell_colour_off;
    }
}
