extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::time::{Duration, Instant};

pub struct App {
    gl: GlGraphics,
    grid: Vec<Vec<bool>>,
    rows: u32,
    cols: u32,
    last_update: Instant,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GRAY: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];


        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            clear(GRAY, gl);

            for row in 0..self.rows {
                for col in 0..self.cols {
                    let square = rectangle::square((row * 10) as f64, (col * 10) as f64, 10.0);
                    rectangle(if self.grid[row as usize][col as usize] { WHITE } else { GRAY }, square, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        if self.last_update.elapsed() >= Duration::from_millis(500) {
            let grid_copy = self.grid.clone();

            for row in 0..self.rows {
                for col in 0..self.cols {

                    self.grid[row as usize][col as usize] = App::pixel(
                        &grid_copy,
                        row as i32,
                        col as i32,
                        self.rows as i32,
                        self.cols as i32,
                    );
                }
            }
            self.last_update = Instant::now();
        }
    }

    fn pixel(grid_copy: &Vec<Vec<bool>>, row: i32, col: i32, rows: i32, cols: i32) -> bool {
        let mut alive_neighbors = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = row + dx;
                let ny = col + dy;

                if nx >= 0 && ny >= 0 && nx < rows && ny < cols {
                    if grid_copy[nx as usize][ny as usize] {
                        alive_neighbors += 1;
                    }
                }
            }
        }

        (alive_neighbors == 3) || (grid_copy[row as usize][col as usize] && (alive_neighbors == 2 || alive_neighbors == 3))
    }

}

fn main() {
    let opengl = OpenGL::V3_2;

    let rows: u32 = 50;
    let cols: u32 = 50;

    let mut window: Window = WindowSettings::new("spinning-square", [rows * 10, cols * 10])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        grid: vec![vec![false; rows as usize]; cols as usize],
        rows: rows,
        cols: cols,
        last_update: Instant::now(),
    };

    grid(&mut app.grid, rows as usize, cols as usize);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

fn grid(grid: &mut Vec<Vec<bool>>, rows: usize, cols: usize) {
    /*
     * Test 1
     **/
    // grid[5][5] = true;
    // grid[4][6] = true;
    // grid[4][4] = true;
    // grid[6][6] = true;
    // grid[7][5] = true;
    // grid[21][21] = true;

    /**
     * Test 2
     */
    // let start_row = 1;
    // let start_col = 1;

    // if start_row + 2 < rows && start_col + 2 < cols {
    //     grid[start_row][start_col + 1] = true;
    //     grid[start_row + 1][start_col + 2] = true;
    //     grid[start_row + 2][start_col] = true;
    //     grid[start_row + 2][start_col + 1] = true;
    //     grid[start_row + 2][start_col + 2] = true;
    // }

    /**
     * Test 3
     */
    // // Choisissez l'emplacement initial approprié pour éviter les problèmes de bord
    // let start_row = 5;
    // let start_col = 1;

    // if start_row + 10 < rows && start_col + 36 < cols {
    //     let indices = [
    //         (5, 1), (5, 2), (6, 1), (6, 2), (3, 13), (3, 14), (4, 12), (4, 16), (5, 11), (5, 17),
    //         (6, 11), (6, 15), (6, 17), (6, 18), (7, 11), (7, 17), (8, 12), (8, 16), (9, 13), (9, 14),
    //         (1, 25), (2, 23), (2, 25), (3, 21), (3, 22), (4, 21), (4, 22), (5, 21), (5, 22), (6, 23),
    //         (6, 25), (7, 25), (3, 35), (3, 36), (4, 35), (4, 36)
    //     ];
    //     for (dx, dy) in indices.iter() {
    //         grid[start_row + dx][start_col + dy] = true;
    //     }
    // }

    /**
     * Test 4
     */
    let start_row = 10;
    let start_col = 10;

    if start_row + 1 < rows && start_col + 3 < cols {
        grid[start_row][start_col + 1] = true;
        grid[start_row][start_col + 2] = true;
        grid[start_row][start_col + 3] = true;
        grid[start_row + 1][start_col] = true;
        grid[start_row + 1][start_col + 1] = true;
        grid[start_row + 1][start_col + 2] = true;
    }
}
