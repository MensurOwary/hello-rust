use opengl_graphics::GlGraphics;
use piston::input::*;
use graphics::*;
use std::collections::LinkedList;

use crate::block::*;
use crate::constants::{WIDTH, BLUE};
use crate::util::{GameMode, Direction};
use crate::food::Food;

pub struct Snake {
    body: LinkedList<Block>,
}

impl Snake {
    /// creates a new snake
    pub fn new() -> Snake {
        let mut ll: LinkedList<Block> = LinkedList::new();
        // initial 3 blocks
        for i in 0..3 {
            ll.push_front(Block {
                x: (i as f64) * WIDTH,
                y: WIDTH,
            });
        }

        Snake {
            body: ll,
        }
    }

    /// renders the snake
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        let ll = &self.body;

        gl.draw(args.viewport(), |c, gl| {
            for block in ll {
                let transform = c.transform.trans(block.x, block.y);
                rectangle(BLUE, rectangle::square(0.0, 0.0, WIDTH), transform, gl);
            }
        });
    }

    /// updates the snake info
    pub fn update(&mut self, direction: Direction, should_eat: bool) -> GameMode {
        // get the new head position
        let new_head = self.get_new_head(direction);

        // check if head crashes into the body
        // if it does crash into itself, end the game
        if self.crashes_into_itself(new_head) {
            return GameMode::End;
        }
        let body = &mut self.body;

        // move the head
        body.push_front(new_head);

        // if there should not be a new tail, then pop the last one
        if !should_eat {
            body.pop_back();
        }
        // save the body
        self.body = body.to_owned();

        // continue
        return GameMode::Continue;
    }

    /// retrieve the new head position based on the Direction
    fn get_new_head(&self, direction: Direction) -> Block {
        self.body
            .front().unwrap()
            .update(direction)
    }

    /// get the number of blocks crashing into the body
    fn crashes_into_itself(&self, new_head: Block) -> bool {
        self.body
            .iter()
            .filter(|&bl| {
                *bl == new_head
            }).count() != 0
    }

    /// check if snake eats the food
    pub fn does_eat_food(&mut self, food: &Food) -> bool {
        food.crashes_with(self.head())
    }

    // get the head block
    fn head(&self) -> &Block {
        self.body.front().unwrap()
    }
}