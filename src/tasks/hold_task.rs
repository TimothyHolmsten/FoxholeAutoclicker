use std::sync::{Arc, Mutex};
use crate::clicker::Clicker;
use crate::tasks::task::Task;

pub struct HoldTask;

impl Task for HoldTask {
    fn execute(&self, clicker: Arc<Mutex<Clicker>>, _: Arc<Mutex<Vec<(i32, i32)>>>) {
        let mut clicker = clicker.lock().unwrap();
        clicker.hold_down();
    }
}