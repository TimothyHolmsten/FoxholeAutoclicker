use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::clicker::Clicker;

pub struct TaskPerformer {
    clicker: Arc<Mutex<Clicker>>,
}

impl TaskPerformer {
    pub fn new(clicker: Arc<Mutex<Clicker>>) -> Self {
        TaskPerformer { clicker }
    }

    pub fn run(&self) {
        let clicker = Arc::clone(&self.clicker);

        thread::spawn(move || {
            loop {
                {
                    let mut clicker = clicker.lock().unwrap();
                    clicker.execute();
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
    }
}