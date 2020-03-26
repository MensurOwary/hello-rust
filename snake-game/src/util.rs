use rand::Rng;
use crate::constants;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    Continue, End, Restart
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Still,
}

impl Direction {
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            _ => Direction::Still,
        }
    }
}

pub fn get_random_within_bound() -> f64 {
    rand::thread_rng().gen_range(0, constants::MAX_RAND_UB) as f64
}