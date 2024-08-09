use core::time;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::sleep;

use device_query::{DeviceEvents, DeviceState};

mod clicker;
mod event_listener;
mod task_performer;

use clicker::Clicker;
use event_listener::EventListener;
use task_performer::TaskPerformer;

fn main() {
    println!("F6 for auto clicker\nF7 for auto holding left mouse\nF9 to start recording mouse macro\nF10 to execute mouse macro");

    // Create a channel for communication
    let (sender2, receiver2) = mpsc::channel();
    
    // Initialize Clicker
    let clicker = Arc::new(Mutex::new(Clicker::new()));
    
    // Initialize TaskPerformer
    let task_performer = TaskPerformer::new(Arc::clone(&clicker));
    
    // Initialize EventListener
    let event_listener = EventListener::new(Arc::clone(&clicker), receiver2);
    
    // Start EventListener
    let _event_listener = event_listener.run();
    
    // Run TaskPerformer
    let _task_performer = task_performer.run();

    let device_state = DeviceState::new();
    let _guard = device_state.on_key_down(move|key| {
        let _ = sender2.send(key.clone());
        event_listener.notify();
    });
    loop {sleep(time::Duration::from_millis(10))}
    
}