use std::sync::{Arc, Mutex, mpsc};
use std::thread::sleep;
use std::time::Duration;

use device_query::{DeviceEvents, DeviceState, Keycode};

mod clicker;
mod event_listener;
mod task_performer;

use clicker::Clicker;
use event_listener::EventListener;
use task_performer::TaskPerformer;

fn main() {
    println!("F6 for auto clicker\nF7 for auto holding left mouse\nF9 to start recording mouse macro\nF10 to execute mouse macro");

    // Create a channel for communication
    let (sender, receiver) = mpsc::channel();
    let clicker = Arc::new(Mutex::new(Clicker::new()));

    let task_performer = TaskPerformer::new(Arc::clone(&clicker));
    let event_listener = EventListener::new(Arc::clone(&clicker), receiver);

    let _event_listener = event_listener.run();
    let _task_performer = task_performer.run();

    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(  move |key| {
        // Only handle specific keys and avoid unnecessary operations
        if matches!(key, 
            Keycode::F6 | 
            Keycode::F7 | 
            Keycode::F9 | 
            Keycode::F10 |
            Keycode::Escape
        ) {
            match sender.send(key.clone()) {
                Ok(_) => event_listener.notify(),
                Err(e) => eprintln!("Failed to send key: {:?}", e),
            }
        }
    });
    loop {sleep(Duration::from_millis(50))}
}