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

pub enum KeyEvent {
    Pressed(Keycode),
    Released(Keycode),
}

pub enum MouseEvent {
    Move(MousePosition),
    Pressed(MouseButton),
    Released(MouseButton),
}

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}

fn listener_thread(sender: Sender<Event>, terminate: Receiver<()>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let device_state = DeviceState::new();

        let tx = sender.clone();
        let guard_kd = device_state.on_key_down(move |key| {
            tx.send(Event::Key(KeyEvent::Pressed(*key)))
                .expect("failed to send key pressed event");
        });

        let tx = sender.clone();
        let guard_ku = device_state.on_key_up(move |key| {
            tx.send(Event::Key(KeyEvent::Released(*key)))
                .expect("failed to send key pressed event");
        });

        let tx = sender.clone();
        let guard_mm = device_state.on_mouse_move(move |pos| {
            tx.send(Event::Mouse(MouseEvent::Move(*pos)))
                .expect("failed to send mouse move event");
        });

        let tx = sender.clone();
        let guard_md = device_state.on_mouse_down(move |button| {
            tx.send(Event::Mouse(MouseEvent::Pressed(*button)))
                .expect("failed to send mouse button down event");
        });

        let tx = sender;
        let guard_mu = device_state.on_mouse_up(move |button| {
            tx.send(Event::Mouse(MouseEvent::Released(*button)))
                .expect("failed to send mouse button up event");
        });

        // Block in terminate signal
        let _ = terminate.recv();
        drop(guard_kd);
        drop(guard_ku);
        drop(guard_mm);
        drop(guard_md);
        drop(guard_mu);
    })
}

fn main() {
    let (tx, rx) = crossbeam_channel::unbounded();
    let mut cast = Broadcast::new();
    listener_thread(tx, cast.subscribe());

    while let Ok(event) = rx.recv() {
        match event {
            Event::Key(event) => match event {
                KeyEvent::Pressed(key) => {
                    if key == Keycode::Delete {
                        cast.send(());
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
        }
    }
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
