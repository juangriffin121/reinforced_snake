pub struct Snake {
    direction: Direction,
    squares: Vec<(i32, i32)>,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Snake {
    pub fn new(squares: Vec<(i32, i32)>, direction: Direction) -> Snake {
        Snake { squares, direction }
    }

    pub fn step(&mut self, food: (i32, i32)) -> bool {
        self.squares.pop();
        let new_square = match self.direction {
            Direction::Up => (self.squares[0].0, self.squares[0].1 + 1),
            Direction::Down => (self.squares[0].0, self.squares[0].1 - 1),
            Direction::Left => (self.squares[0].0 - 1, self.squares[0].1),
            Direction::Right => (self.squares[0].0 + 1, self.squares[0].1),
        };
        let ate = new_square == food;
        self.squares.insert(0, new_square);
        if ate {
            let new_square = match self.direction {
                Direction::Up => (self.squares[0].0, self.squares[0].1 + 1),
                Direction::Down => (self.squares[0].0, self.squares[0].1 - 1),
                Direction::Left => (self.squares[0].0 - 1, self.squares[0].1),
                Direction::Right => (self.squares[0].0 + 1, self.squares[0].1),
            };
            self.squares.insert(0, new_square);
        }
        ate
    }
}
