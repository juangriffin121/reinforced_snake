use crossterm::event::{self, poll, read, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use std::io;
use std::io::Write;
use std::process;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::Duration;

use super::snake::Direction;

pub trait User {
    fn get_direction(&self) -> Option<Direction>;
}

pub struct SyncUser;
impl User for SyncUser {
    fn get_direction(&self) -> Option<Direction> {
        let interval = Duration::from_millis(200);
        let now = Instant::now();
        while now.elapsed() < interval {
            let timeout = interval - now.elapsed();
            if poll(timeout).ok()? {
                let pressed = read().ok()?;
                write!(io::stdout(), "{}\r\n", format!("{pressed:?}")).expect("nonsense");
                match pressed {
                    Event::Key(key_event) => return handle_key(key_event.code),
                    _ => None::<Direction>,
                }
            } else {
                None
            };
        }
        None
    }
}
// impl User for SyncUser {
//     fn get_direction(&self) -> Option<Direction> {
//         if event::poll(Duration::from_millis(500)).unwrap() {
//             loop {
//                 if let Ok(Event::Key(key_event)) = event::read() {
//                     println!("enters");
//                     return handle_key(key_event.code);
//                 } else {
//                     break;
//                 }
//             }
//         }
//         None
//     }
// }

pub struct AsyncUser {
    direction_receiver: Arc<Mutex<mpsc::Receiver<KeyCode>>>,
}
impl AsyncUser {
    pub async fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        let receiver = Arc::new(Mutex::new(rx));

        // Spawn a task to handle key events.
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut last_key_pressed: Option<KeyCode> = None;
            loop {
                if event::poll(Duration::from_millis(1000)).unwrap() {
                    if let Event::Key(key_event) = event::read().unwrap() {
                        last_key_pressed = Some(key_event.code);
                        tx_clone.send(key_event.code).await.unwrap();
                    }
                }
            }
        });

        AsyncUser {
            direction_receiver: receiver,
        }
    }
}

impl User for AsyncUser {
    fn get_direction(&self) -> Option<Direction> {
        let mut receiver = self.direction_receiver.lock().unwrap();
        receiver
            .try_recv()
            .ok()
            .map(|key_code| handle_key(key_code))
            .or(None)?
    }
}
// impl AsyncUser {
//     pub async fn new() -> Self {
//         let (tx, rx) = mpsc::channel(32);
//         let receiver = Arc::new(Mutex::new(rx));
//
//         // Spawn a task to handle key events.
//         let tx_clone = tx.clone();
//         let wait = Duration::from_millis(1000);
//
//         tokio::spawn(async move {
//             loop {
//                 if event::poll(wait).unwrap() {
//                     if let Event::Key(key_event) = event::read().unwrap() {
//                         tx_clone.send(key_event.code).await.unwrap();
//                     }
//                 }
//             }
//         });
//
//         AsyncUser {
//             direction_receiver: receiver,
//         }
//     }
// }
//
// impl User for AsyncUser {
//     fn get_direction(&self) -> Option<Direction> {
//         let mut receiver = self.direction_receiver.lock().unwrap();
//         if let Ok(key_code) = receiver.try_recv() {
//             handle_key(key_code)
//         } else {
//             None
//         }
//     }
// }

pub fn handle_key(key: KeyCode) -> Option<Direction> {
    write!(io::stdout(), "{}\r\n", format!("{key:?}")).expect("nonsense");
    match key {
        KeyCode::Up | KeyCode::Char('k') => {
            return Some(Direction::Up);
        }
        KeyCode::Down | KeyCode::Char('j') => {
            return Some(Direction::Down);
        }
        KeyCode::Left | KeyCode::Char('h') => {
            return Some(Direction::Left);
        }
        KeyCode::Right | KeyCode::Char('l') => {
            return Some(Direction::Right);
        }
        KeyCode::Char('q') => {
            terminal::disable_raw_mode().unwrap();
            process::exit(0);
            return None;
        }
        _ => None,
    }
}
/*
       KeyCode::Ctrl('d') | KeyCode::Ctrl('z') => {
           std::process::exit(0);
       }
*/
