use crossterm::terminal;
use mods::{
    snake::{Direction, Snake},
    user::{AsyncUser, SyncUser},
    world::{BorderType, World},
};

mod mods;
// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _raw_mode = terminal::enable_raw_mode()?;
    let grid_shape = (20, 10);
    let mut world = World::new(grid_shape, (5, 3), BorderType::Donut);
    let squares = vec![(1, 1), (1, 2)];
    let mut snake = Snake::new(squares, Direction::Down);
    let user = SyncUser;
    // println!("{:?}", world.grid);
    world.run(&mut snake, &user);
    terminal::disable_raw_mode()?;
    Ok(())
}
