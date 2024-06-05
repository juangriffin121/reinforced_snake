use super::snake::Snake;
use super::user::User;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

pub struct World {
    pub grid_shape: (i32, i32),
    pub grid: Vec<(i32, i32)>,
    pub food: (i32, i32),
    pub borders: BorderType,
    pub end: bool,
}

pub enum BorderType {
    Donut,
    Death,
}

impl World {
    pub fn new(grid_shape: (i32, i32), food: (i32, i32), borders: BorderType) -> Self {
        let mut grid = Vec::new();
        for i in 0..grid_shape.0 {
            for j in 0..grid_shape.1 {
                grid.push((i, j))
            }
        }
        Self {
            grid_shape,
            grid,
            food,
            borders,
            end: false,
        }
    }

    pub fn new_food(&mut self, snake: Snake, rng: &mut impl Rng) {
        let free_squares: Vec<&(i32, i32)> = self
            .grid
            .iter()
            .filter(|&square| {
                !snake
                    .squares
                    .iter()
                    .any(|&snake_square| snake_square == *square)
            })
            .collect();
        if let Some(&random_coordinate) = free_squares.choose(rng) {
            self.food = *random_coordinate;
        } else {
            println!("no squares left")
        }
    }

    pub fn run<U: User>(&mut self, snake: &mut Snake, user: &U) {
        while !self.end {
            if let Some(direction) = user.get_direction() {
                snake.direction = direction;
            };
            self.end = snake.step(&*self);
        }
    }
}
