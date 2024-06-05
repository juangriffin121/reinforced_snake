use super::snake::Direction;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub trait User {
    fn get_direction(&self) -> Option<Direction>;
}

pub struct HumanUser;

impl User for HumanUser {
    fn get_direction(&self) -> Option<Direction> {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up | KeyCode::Char('k') => Some(Direction::Up),
                    KeyCode::Down | KeyCode::Char('j') => Some(Direction::Down),
                    KeyCode::Left | KeyCode::Char('h') => Some(Direction::Left),
                    KeyCode::Right | KeyCode::Char('l') => Some(Direction::Right),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
