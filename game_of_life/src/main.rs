/**
 * Reference: https://github.com/Rust-SDL2/rust-sdl2/blob/master/examples/demo.rs
 */
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;
use game_of_life::{GameOfLife, GameOfLifeRenderer};
use renderable::{Renderable, Position2D};

mod renderable {
    extern crate sdl2;
    use sdl2::render::{Canvas, RenderTarget};

    pub struct Position2D{
        pub x:i32, 
        pub y:i32,
    }

    pub trait Renderable {
        fn update(&mut self);
        fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos:Position2D);
    }

}

mod game_of_life {

    extern crate rand;
    use sdl2::pixels::Color;
    use sdl2::render::{Canvas, RenderTarget};
    use sdl2::rect::Rect;
    use renderable;

    #[derive(Copy, Clone,PartialEq)]
    pub enum CellState {
        Alive,
        Dead,
    }

    pub struct GameOfLife {
        rows: usize,
        cols: usize,
        board: Vec<Vec<CellState>>,
    }

    pub struct GameOfLifeRenderer {
        alive_color: Color,
        dead_color: Color,
        cell_width: u32,
        cell_height: u32,
        game_of_life: GameOfLife
    }

    impl GameOfLife {

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

        fn count_neighbours(&self, x:usize, y:usize) -> i32 {
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


    impl GameOfLifeRenderer {
        pub fn build(game_of_life:GameOfLife) -> GameOfLifeRenderer {
            GameOfLifeRenderer {
                dead_color : Color::RGB(255,255,255),
                alive_color : Color::RGB(0,0,0),
                cell_width : 20,
                cell_height : 20,
                game_of_life : game_of_life,
            }
        }
    }

    impl renderable::Renderable for GameOfLifeRenderer {

        fn update(&mut self) {
            self.game_of_life.update();
        }

        fn draw<T:RenderTarget>(&self, canvas: &mut Canvas<T>, pos: renderable::Position2D) {

            for x in 0..self.game_of_life.rows() {
                for y in 0..self.game_of_life.cols() {

                    let rx = (x as i32) * (self.cell_width as i32) + pos.x;
                    let ry = (y as i32) * (self.cell_height as i32) + pos.y;

                    let box_rect = Rect::new(rx, ry, self.cell_width, self.cell_height);

                    if self.game_of_life.is_cell_alive(x, y) {
                        canvas.set_draw_color(self.alive_color);
                    }
                    else {
                        canvas.set_draw_color(self.dead_color);
                    }
                    canvas.fill_rect(box_rect);
                }
            }
            canvas.present();
        }
    }
}


fn main() {

    let width = 800;
    let height = 600;

    let mut gol1 = GameOfLifeRenderer::build(GameOfLife::build_random(20,20));
    let mut gol2 = GameOfLifeRenderer::build(GameOfLife::build_random(20,20));


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

        if next_update < 0 {
            gol1.update();
            gol2.update();
            next_update = 100;
        }

        gol1.draw(&mut canvas, Position2D{x:0, y:0});


        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // Update 60fps
        next_update = next_update - 1;

    }
}
