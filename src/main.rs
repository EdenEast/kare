#![allow(unused)]
use clap::Parser;
use crossbeam_channel::{Receiver, SendError, Sender};

use crate::cli::Cli;

pub mod cli;
mod gui;

use device_query::{DeviceEvents, DeviceState, Keycode, MouseButton, MousePosition};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Default)]
pub struct Broadcast<T> {
    channels: Vec<crossbeam_channel::Sender<T>>,
}

impl<T: 'static + Clone + Send + Sync> Broadcast<T> {
    pub fn new() -> Self {
        Self { channels: vec![] }
    }

    pub fn subscribe(&mut self) -> Receiver<T> {
        let (tx, rx) = crossbeam_channel::unbounded();
        self.channels.push(tx);
        rx
    }

    pub fn send(&self, message: T) -> Result<(), SendError<T>> {
        for c in self.channels.iter() {
            c.send(message.clone())?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum KeyEvent {
    Pressed(Keycode),
    Released(Keycode),
}

#[derive(Debug, Clone, Copy)]
pub enum MouseEvent {
    Move(MousePosition),
    Pressed(MouseButton),
    Released(MouseButton),
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Play,
    Pause,
    Record,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyBinding {
    pub record: Keycode,
    pub play: Keycode,
    pub pause: Keycode,
}

impl Default for KeyBinding {
    fn default() -> Self {
        Self {
            record: Keycode::F9,
            play: Keycode::F10,
            pause: Keycode::F11,
        }
    }
}

impl KeyBinding {
    pub fn event_from_key(&self, key: &Keycode) -> Option<Event> {
        if key == &self.record {
            Some(Event::Record)
        } else if key == &self.play {
            Some(Event::Play)
        } else if key == &self.pause {
            Some(Event::Pause)
        } else {
            None
        }
    }
}

fn main() {
    let (tx, rx) = crossbeam_channel::unbounded();
    let bindings = KeyBinding::default();

    let device_state = DeviceState::new();

    let sender = tx.clone();
    let guard_kd = device_state.on_key_down(move |key| {
        let event = bindings
            .event_from_key(key)
            .unwrap_or(Event::Key(KeyEvent::Pressed(*key)));

        sender
            .send(event)
            .expect("failed to send key pressed event");
    });

    let sender = tx.clone();
    let guard_ku = device_state.on_key_up(move |key| {
        if bindings.event_from_key(key).is_none() {
            sender
                .send(Event::Key(KeyEvent::Released(*key)))
                .expect("failed to send key pressed event");
        }
    });

    let sender = tx.clone();
    let guard_mm = device_state.on_mouse_move(move |pos| {
        sender
            .send(Event::Mouse(MouseEvent::Move(*pos)))
            .expect("failed to send mouse move event");
    });

    let sender = tx.clone();
    let guard_md = device_state.on_mouse_down(move |button| {
        sender
            .send(Event::Mouse(MouseEvent::Pressed(*button)))
            .expect("failed to send mouse button down event");
    });

    let sender = tx;
    let guard_mu = device_state.on_mouse_up(move |button| {
        sender
            .send(Event::Mouse(MouseEvent::Released(*button)))
            .expect("failed to send mouse button up event");
    });

    while let Ok(event) = rx.recv() {
        match event {
            Event::Key(event) => match event {
                KeyEvent::Pressed(key) => {
                    if key == Keycode::Delete {
                        break;
                    }
                    println!("Key pressed: {:?}", key);
                }
                KeyEvent::Released(key) => {
                    println!("Key Relaesed: {:?}", key);
                }
            },
            Event::Mouse(event) => match event {
                MouseEvent::Move(pos) => {
                    println!("Move: {}, {}", pos.0, pos.1);
                }
                MouseEvent::Pressed(button) => {
                    println!("mouse pressed: {:?}", button);
                }
                MouseEvent::Released(button) => {
                    println!("mouse released: {:?}", button);
                }
            },
            Event::Play => {
                println!("Play event");
            }
            Event::Pause => {
                println!("Pause event");
            }
            Event::Record => {
                println!("Record event");
            }
        }
    }

    drop(guard_kd);
    drop(guard_ku);
    drop(guard_mm);
    drop(guard_md);
    drop(guard_mu);
}

// fn main() {
//     let cli = Cli::parse();
//     match cli.command {
//         Some(command) => match command {
//             cli::Cmd::Play(_) => todo!(),
//             cli::Cmd::Record(_) => todo!(),
//         },
//         None => gui::run().unwrap(),
//     }
// }
