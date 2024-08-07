use std::sync::{Arc, Mutex};
use crate::clicker::Clicker;
use crate::tasks::task::Task;

pub struct ClickTask;

impl Task for ClickTask {
    fn execute(&self, clicker: Arc<Mutex<Clicker>>, _: Arc<Mutex<Vec<(i32, i32)>>>) {
        let mut clicker = clicker.lock().unwrap();
        clicker.click(); // Perform click action
    }
}