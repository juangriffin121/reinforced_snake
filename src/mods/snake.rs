use super::{
    user::User,
    world::{BorderType, World},
};

pub struct Snake {
    pub direction: Direction,
    pub squares: Vec<(i32, i32)>,
    pub fed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(squares: Vec<(i32, i32)>, direction: Direction) -> Snake {
        Snake {
            squares,
            direction,
            fed: false,
        }
    }

    pub fn update_direction(&mut self, new_direction: Direction) {
        self.direction = match (self.direction, new_direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => self.direction,
            _ => new_direction,
        };
    }

    pub fn step(&mut self, world: &World) -> bool {
        if !self.fed {
            self.squares.pop();
        }
        let new_square = match self.direction {
            Direction::Up => (self.squares[0].0, self.squares[0].1 + 1),
            Direction::Down => (self.squares[0].0, self.squares[0].1 - 1),
            Direction::Left => (self.squares[0].0 - 1, self.squares[0].1),
            Direction::Right => (self.squares[0].0 + 1, self.squares[0].1),
        };
        let mut died = self.squares.iter().any(|&square| square == new_square);
        let (max_x, max_y) = world.grid_shape;
        let new_square = match world.borders {
            BorderType::Donut => (new_square.0 % max_x, new_square.0 % max_y),
            BorderType::Death => {
                died = true;
                new_square
            }
        };
        self.fed = new_square == world.food;
        self.squares.insert(0, new_square);
        died
    }
}
