use core::time;
use std::{sync::mpsc::{channel, TryRecvError}, thread::{self, sleep}};

use enigo::{
    Button, Coordinate,
    Direction::{Press, Release},
    Enigo, Mouse, Settings,
};
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    println!("F6 for auto clicker\nF7 for auto holding left mouse\nF9 to start recording mouse macro\nF10 to execute mouse macro");

    let device_state = DeviceState::new();
    
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::F6 => {
                    sleep(time::Duration::from_millis(100)); auto_clicker(&device_state);},
                Keycode::F7 => {
                    sleep(time::Duration::from_millis(100)); auto_holder(&device_state);},
                Keycode::F9 => {
                    sleep(time::Duration::from_millis(100)); macro_mouse_recorder(&device_state);
                },
                _ => (),
            }
        }
    }
}

fn auto_clicker(device_state: &DeviceState) {
    println!("<Auto clicking>");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        match enigo.button(Button::Left, Press) {
            Err(_e) => println!("Could not press left mouse button"),
            _ => ()
        }
        match enigo.button(Button::Left, Release) {
            Err(_e) => println!("Could not release left mouse button"),
            _ => ()
        }
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::F6 => {
                    sleep(time::Duration::from_millis(500)); return;},
                _ => (),
            }
        }
        sleep(time::Duration::from_micros(64));
    }
}

fn auto_holder(device_state: &DeviceState) {
    println!("<Auto holding>");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match enigo.button(Button::Left, Press) {
        Err(_e) => 
            println!("Could not press left mouse button"),
        _ => ()
    };

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::F7 => {
                        match enigo.button(Button::Left, Release) {
                            Err(_e) => 
                                println!("Could not release left mouse button"),
                            _ => ()
                        };
                        sleep(time::Duration::from_millis(500));
                        return;
                    },
                _ => (),
            }
        }
    }
}

fn macro_mouse_recorder(device_state: &DeviceState) {
    println!("<Recording cursor locations>");

    let enigo = Enigo::new(&Settings::default()).unwrap();
    let mut cursor_locations: Vec<(i32, i32)> = Vec::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::F9 => {
                    cursor_locations.insert(cursor_locations.len(), enigo.location().unwrap());
                    sleep(time::Duration::from_millis(500));
                },
                Keycode::F10 => {
                    sleep(time::Duration::from_millis(500));
                    return macro_mouse_runner(device_state, cursor_locations.clone());
                }
                _ => (),
            }
        }
    }
}

fn macro_mouse_runner(device_state: &DeviceState, cursor_locations: Vec<(i32, i32)>) {
    let (tx, rx) = channel();

    let _macro_thread = thread::spawn(move || {
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        loop {
            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    println!("Terminating macro");
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
            for cursor_location in cursor_locations.iter() {
                let _move = enigo.move_mouse(cursor_location.0, cursor_location.1, Coordinate::Abs);
                let _press = enigo.button(Button::Left, Press);
                let _release = enigo.button(Button::Left, Release);
                sleep(time::Duration::from_millis(50));
            }
        }
    });

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys.iter() {
            match key {
                Keycode::F10 => {
                    let _ = tx.send(()); // Stops the thread
                    sleep(time::Duration::from_millis(500)); 
                    _macro_thread.join().expect("Could not join");
                    return
                },
                _ => ()
            }
        }
    }
}