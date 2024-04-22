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

    fn update(&mut self, args: &UpdateArgs) {
        if self.last_update.elapsed() >= Duration::from_millis(500) {
            for row in 0..self.rows {
                for col in 0..self.cols {
                    self.grid[row as usize][col as usize] = !self.grid[row as usize][col as usize];
                }
            }
            self.last_update = Instant::now(); // Réinitialiser le timer
        }
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
    grid[20][20] = true;

    // for row in 0..rows {
    //     for col in 0..cols {
    //         print!("{} ", if grid[row][col] { 'o' } else { 'x'});
    //     }
    //     println!();
    // }
}
