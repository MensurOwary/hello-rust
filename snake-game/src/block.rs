use crate::constants;
use crate::util::Direction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub x: f64,
    pub y: f64,
}

impl Block {
    /// updates the block based on the direction
    pub fn update(self, direction: Direction) -> Block {
        // move the values in the appropriate direction
        let x = self.x.move_x(direction);
        let y = self.y.move_y(direction);

        // clamp the values
        let x = x.clamp_value();
        let y = y.clamp_value();
        Block {
            x,
            y,
        }
    }
}

trait UpdatePoint {
    fn move_x(self, direction: Direction) -> f64;
    fn move_y(self, direction: Direction) -> f64;
    fn clamp_value(self) -> f64;
}

impl UpdatePoint for f64 {
    fn move_x(self, direction: Direction) -> f64 {
        match direction {
            Direction::Right => self + constants::INCR,
            Direction::Left => self - constants::INCR,
            _ => self,
        }
    }

    fn move_y(self, direction: Direction) -> f64 {
        match direction {
            Direction::Up => self - constants::INCR,
            Direction::Down => self + constants::INCR,
            _ => self,
        }
    }

    fn clamp_value(self) -> f64 {
        if self < 0.0 {
            0.0
        } else if self > constants::SCREEN_SIZE - constants::WIDTH {
            constants::SCREEN_SIZE
        } else {
            self
        }
    }
}