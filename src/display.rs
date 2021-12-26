use sdl2::{pixels::Color, rect::Rect};

extern crate sdl2;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

pub struct Display<'a> {
    canvas: &'a mut sdl2::render::Canvas<sdl2::video::Window>,
    pub cells: [(bool, sdl2::rect::Rect); SCREEN_WIDTH * SCREEN_HEIGHT],
    pub cell_colour_on: sdl2::pixels::Color,
    pub cell_colour_off: sdl2::pixels::Color,
}

impl Display<'_> {
    pub fn new(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Display {
        Display {
            cells: [(false, sdl2::rect::Rect::new(0, 0, 0, 0)); SCREEN_WIDTH * SCREEN_HEIGHT],
            cell_colour_on: sdl2::pixels::Color::WHITE,
            cell_colour_off: sdl2::pixels::Color::BLACK,
            canvas,
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

    pub fn clear_screen(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }
    pub fn draw_canvas(&mut self) {
        self.canvas.present();
    }

    pub fn draw_rect_to_canvas(&mut self, cell: &(bool, Rect)) {
        if cell.0 {
            self.canvas.set_draw_color(self.cell_colour_on);
        } else {
            self.canvas.set_draw_color(self.cell_colour_off);
        }
        self.canvas.fill_rect(cell.1).expect("Failed to draw rects");
    }
}
