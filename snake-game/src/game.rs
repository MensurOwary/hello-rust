use crate::{snake, constants};
use crate::util::{GameMode, Direction};
use crate::snake::Snake;
use crate::food::Food;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs, Key};
use graphics::*;

pub struct Game {
    gl: GlGraphics,
    snake: snake::Snake,
    food: Food,
    direction: Direction,
    update_time_passed: f64,
    game_mode: GameMode,
}

impl Game {
    /// initializes a new game
    pub fn new(open_gl: OpenGL) -> Game {
        Game {
            gl: GlGraphics::new(open_gl),
            direction: Direction::Right,
            snake: snake::Snake::new(),
            // initial food position
            food: Food::new(),
            update_time_passed: 0.0,
            game_mode: GameMode::Continue,
        }
    }

    /// renders the game
    pub fn render(&mut self, args: &RenderArgs) {
        // if the game has ended display red
        if self.game_mode == GameMode::End {
            clear(constants::RED, &mut self.gl);
        } else {
            // if game continues, display the game
            clear(constants::GREEN, &mut self.gl);
            self.snake.render(&mut self.gl, &args);
            self.food.render(&mut self.gl, &args);
        }
    }

    /// updates the game
    pub fn update(&mut self, args: &UpdateArgs) {
        match self.game_mode {
            GameMode::Continue => {
                self.play(args);
            }
            GameMode::Restart => {
                self.snake = Snake::new();
                self.food = Food::new();
                self.game_mode = GameMode::Continue;
            }
            GameMode::End => (),
        }
    }

    /// handle key press
    pub fn key_pressed(&mut self, key: &Key) {
        // get the new direction
        let new_direction = match *key {
            Key::Up => Direction::Up,
            Key::Down => Direction::Down,
            Key::Left => Direction::Left,
            Key::Right => Direction::Right,
            _ => self.direction, // keep the previous direction
        };
        // check if the Space is pressed when game ends
        match *key {
            Key::Space if self.game_mode == GameMode::End => {
                self.game_mode = GameMode::Restart;
                self.direction = Direction::Right;
            }
            _ => ()
        }
        // do not allow to move the opposite direction
        if new_direction.opposite() != self.direction {
            self.direction = new_direction;
        }
    }

    /// handle the game play
    fn play(&mut self, args: &UpdateArgs) {
        self.update_time_passed += args.dt;
        if self.update_time_passed > 0.1 {
            let should_eat = self.snake.does_eat_food(&self.food);
            self.game_mode = self.snake.update(self.direction, should_eat);
            if should_eat {
                self.food = Food::new();
            }
            self.update_time_passed = 0.0;
        }
    }
}
