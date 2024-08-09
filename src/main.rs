use core::time;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, sleep};

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
    
    // Initialize Clicker
    let clicker = Arc::new(Mutex::new(Clicker::new()));
    
    // Initialize TaskPerformer
    let task_performer = TaskPerformer::new(Arc::clone(&clicker));
    
    // Initialize EventListener
    let event_listener = EventListener::new(Arc::clone(&clicker), receiver);
    
    // Start EventListener
    let _event_listener = event_listener.run();
    
    // Run TaskPerformer
    let _task_performer = task_performer.run();

    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(move |key| {
        // Only handle specific keys and avoid unnecessary operations
        if matches!(key, 
            Keycode::F6 | 
            Keycode::F7 | 
            Keycode::F9 | 
            Keycode::F10 |
            Keycode::Escape
        ) {
            let _ = sender.send(key.clone());
            event_listener.notify();
        }
    });
    loop {sleep(time::Duration::from_millis(50))}
    
}