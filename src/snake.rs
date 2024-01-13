use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use piston::{Button, Key};
//use rand::prelude::*;

use std::collections::LinkedList;

// Window dimensions
pub const CELL_SIZE: u32 = 20;
pub const GRID_X_COUNT: u32 = 18;
pub const GRID_Y_COUNT: u32 = 14;

const BACKGROUND_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0]; // grey

// Snake
const SNAKE_COLOR: [f32; 4] = [0.8, 0.4, 0.1, 1.0]; // orange

// Food
const FOOD_COLOR: [f32; 4] = [0.1, 0.6, 0.3, 1.0]; // green

pub struct Application {
    gl: GlGraphics,
    // Snake is list of x and y positions
    list: LinkedList<(u32, u32)>,
    food_x_posit: u32,
    food_y_posit: u32,
    timer: f64,
    snake_direction: Key,
    key_pressed: Key,
}

impl Application {
    pub fn new() -> Self {
        let opengl = OpenGL::V3_2;
        let mut app = Application {
            gl: GlGraphics::new(opengl),
            list: LinkedList::new(),
            food_x_posit: 4,
            food_y_posit: 5,
            timer: 0.0,
            snake_direction: Key::Right,
            key_pressed: Key::Right,
        };
        // Snake elements
        app.list.push_back((10, 7));
        app.list.push_back((9, 7));
        app.list.push_back((8, 7));
        app.list.push_back((7, 7));
        app
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BACKGROUND_COLOR, gl);

            // Loop snake elements
            for element in &self.list {
                // Draw snake element
                let snake = [
                    (element.0 * CELL_SIZE) as f64,
                    (element.1 * CELL_SIZE) as f64,
                    (CELL_SIZE - 1) as f64,
                    (CELL_SIZE - 1) as f64,
                ];
                rectangle(SNAKE_COLOR, snake, c.transform, gl);
            }

            // Draw food
            let food = [
                (self.food_x_posit * CELL_SIZE) as f64,
                (self.food_y_posit * CELL_SIZE) as f64,
                (CELL_SIZE - 1) as f64,
                (CELL_SIZE - 1) as f64,
            ];
            rectangle(FOOD_COLOR, food, c.transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timer = self.timer + args.dt;
        if self.timer >= 0.15 {
            //println!("timer hit {}", self.timer);
            self.timer = 0.0;
        }
    }

    pub fn press(&mut self, button: &Button) {
        if let &Button::Keyboard(key) = button {
            match key {
                Key::Up => {
                    // Allow valid direction
                    if self.snake_direction != Key::Down {
                        self.key_pressed = key;
                    }
                }
                Key::Down => {
                    // Allow valid direction
                    if self.snake_direction != Key::Up {
                        self.key_pressed = key;
                    }
                }
                Key::Left => {
                    // Allow valid direction
                    if self.snake_direction != Key::Right {
                        self.key_pressed = key;
                    }
                }
                Key::Right => {
                    // Allow valid direction
                    if self.snake_direction != Key::Left {
                        self.key_pressed = key;
                    }
                }
                _ => { // Do nothing
                }
            }
        }
    }
}