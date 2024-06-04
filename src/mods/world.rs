use super::snake::Snake;

pub struct World {
    grid_shape: (i32, i32),
    food: (i32, i32),
    borders: BorderType,
    snake: Snake,
}

pub enum BorderType {
    Donut,
    Death,
}
