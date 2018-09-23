/**
 * Reference: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/demo.rs
 */
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, RenderTarget};
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

mod game_of_life {

    extern crate rand;

    #[derive(Copy, Clone,PartialEq)]
    pub enum CellState {
        Alive,
        Dead,
    }

    pub struct GameOfLife{
        rows: usize,
        cols: usize,
        board: Vec<Vec<CellState>>,
    }

    impl GameOfLife {

        pub fn build_empty(rows:usize, cols:usize) -> GameOfLife {

            let mut board: Vec<Vec<CellState>> = Vec::new();

            for i in 0..rows {
                board.push(Vec::new());

                for _ in 0..cols {
                    board[i].push(CellState::Dead);
                }
            }

            GameOfLife {
                rows,
                cols,
                board,
            }
        }

        pub fn build_random(rows:usize, cols:usize) -> GameOfLife {

            let mut board: Vec<Vec<CellState>> = Vec::new();

            for i in 0..rows {
                board.push(Vec::new());
                for _ in 0..cols {
                    if rand::random() {
                        board[i].push(CellState::Alive);
                    } else {
                        board[i].push(CellState::Dead);
                    }
                }
            }

            GameOfLife {
                rows,
                cols,
                board,
            }
        }

        pub fn count_neighbours(&self, x:usize, y:usize) -> i32 {
            let mut ret = 0;
            if x > 0 && self.board[x-1][y] == CellState::Alive {
                ret = ret + 1;
            }
            if x+1 < self.rows && self.board[x+1][y] == CellState::Alive {
                ret = ret + 1;
            }
            if y > 0 && self.board[x][y-1] == CellState::Alive {
                ret = ret + 1;
            }
            if y+1 < self.cols && self.board[x][y+1] == CellState::Alive {
                ret = ret + 1;
            }
            ret
        }

        pub fn update(&mut self) {
            let mut n_count : Vec<Vec<i32>> = Vec::new();       

            for i in 0..self.rows {
                n_count.push(Vec::new());
                for j in 0..self.cols {
                    n_count[i].push(self.count_neighbours(i,j));
                }
            }

            for x in 0..self.rows {
                for y in 0..self.cols {
                    self.board[x][y] = match self.board[x][y] {
                        CellState::Alive => {
                            let mut new_cell_state = CellState::Dead;
                            if n_count[x][y] == 2 || n_count[x][y] == 3 {
                                new_cell_state = CellState::Alive;
                            }  
                            new_cell_state
                        },
                        CellState::Dead => {
                            let mut new_cell_state = CellState::Dead;
                            if n_count[x][y] == 3 {
                                new_cell_state = CellState::Alive;
                            }  
                            new_cell_state
                        }
                    }
                }
            }
        }


        pub fn is_cell_alive(&self, x:usize, y:usize) -> bool {
            match self.board[x][y] {
                CellState::Alive => true,
                CellState::Dead => false
            }
        }

        pub fn rows(&self) -> usize {
            self.rows
        }
        pub fn cols(&self) -> usize {
            self.cols
        }

    }
}

fn draw<T: RenderTarget>(gameOfLife: &game_of_life::GameOfLife, canvas: &mut Canvas<T>, 
                         sx:i32, sy:i32, cell_width:u32, cell_height:u32) {

    let white = Color::RGB(255,255,255);
    let black = Color::RGB(0,0,0);

    for x in 0..gameOfLife.rows() {
        for y in 0..gameOfLife.cols() {

            let rx = (x as i32) * (cell_width as i32) + sx;
            let ry = (y as i32) * (cell_height as i32) + sy;

            let box_rect = Rect::new(rx, ry, cell_width, cell_height);

            if gameOfLife.is_cell_alive(x, y) {
                    canvas.set_draw_color(black);
            }
            else {
                    canvas.set_draw_color(white);
            }
            canvas.fill_rect(box_rect);

        }
    }

    canvas.present();

}


fn main() {

    let width = 800;
    let height = 600;
    let box_width = 20;
    let box_height = 20;
    let num_box_rows = height / box_height;
    let num_box_cols = width / box_width;

    let mut gol1 = game_of_life::GameOfLife::build_random(20,20);
    let mut gol2 = game_of_life::GameOfLife::build_random(20,20);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust sdl2", width, height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255,0,0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut next_update = 100;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    break 'running
                },
                _=> {}
            }
        }

        if (next_update < 0) {
            gol1.update();
            gol2.update();
            next_update = 100;
        }

        draw(&gol1, &mut canvas, 0, 0, 10, 10);
        draw(&gol2, &mut canvas, 300, 0, 10, 10);

        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // Update 60fps
        next_update = next_update - 1;

    }
}
