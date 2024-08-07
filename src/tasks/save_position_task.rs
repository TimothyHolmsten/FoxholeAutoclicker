use std::sync::{Arc, Mutex};
use enigo::Mouse;

use crate::clicker::Clicker;
use crate::tasks::task::Task;

pub struct SavePositionTask;

impl Task for SavePositionTask {
    fn execute(&self, clicker: Arc<Mutex<Clicker>>, macro_positions: Arc<Mutex<Vec<(i32, i32)>>>){
        let (x, y) = clicker.lock().unwrap().enigo.location().unwrap();
        let mut positions = macro_positions.lock().unwrap();
        positions.push((x, y));
        println!("Saved mouse position: ({}, {})", x, y);
    }
}