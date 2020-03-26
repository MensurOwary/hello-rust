use crate::block::Block;
use piston::input::RenderArgs;
use crate::{constants, util};
use opengl_graphics::GlGraphics;
use graphics::*;

pub struct Food {
    food: Option<Block>
}

impl Food {
    /// generate a new random food
    pub fn new() -> Food {
        let x = util::get_random_within_bound() * constants::WIDTH;
        let y = util::get_random_within_bound() * constants::WIDTH;
        Food {
            food: Option::Some(Block {
                x,
                y,
            })
        }
    }

    /// render the food object
    pub fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        match self.food {
            Some(food) => {
                gl.draw(args.viewport(), |c, graphics| {
                    let transform = c.transform.trans(food.x, food.y);
                    rectangle(constants::RED, rectangle::square(0.0, 0.0, constants::WIDTH), transform, graphics);
                });
            }
            None => ()
        }
    }

    /// check if it crashes with the given block
    pub fn crashes_with(&self, block: &Block) -> bool {
        match self.food {
            Some(bait) => bait == *block,
            None => false
        }
    }

}