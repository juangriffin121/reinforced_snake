use std::io::{self, Write};

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

use super::{snake::Snake, world::World};

pub fn graph_ascii(world: &World, snake: &Snake) {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    let mut txt = String::new();
    let (width, height) = world.grid_shape;

    // Top border
    txt += &"#".repeat((width + 2) as usize);
    txt += "\r\n";

    for y in 0..height {
        txt += "#"; // Left border
        for x in 0..width {
            let square = (x, y);
            if snake.squares.contains(&square) {
                // Differentiate head from body if needed
                if snake.squares[0] == square {
                    txt += "H"; // Snake head
                } else {
                    txt += "O"; // Snake body
                }
            } else if square == world.food {
                txt += "*"; // Food
            } else {
                txt += "."; // Empty space
            }
        }
        txt += "#"; // Right border
        txt += "\r\n";
    }

    // Bottom border
    txt += &"#".repeat((width + 2) as usize);
    txt += "\r\n";
    write!(io::stdout(), "{txt}").expect("asd");
}
/*
pub fn graph_ascii(world: &World, snake: &Snake) {
    let mut txt = String::new();
    let mut row = -1;
    txt += &"#".repeat(world.grid_shape.0 as usize);
    txt += "\n";
    for square in &world.grid {
        txt += "#";
        if square.1 > row {
            txt += "\n";
            row = square.1;
        }
        if snake.squares.contains(square) {
            // todo: make head differnet
            txt += "O"
        } else if *square == world.food {
            txt += "*";
        } else {
            txt += ".";
        }
        txt += "#";
    }
    txt += "\n";
    txt += &"#".repeat(world.grid_shape.0 as usize);
    println!("{txt}");
} */
