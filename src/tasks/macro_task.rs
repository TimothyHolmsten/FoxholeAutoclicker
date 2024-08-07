use std::sync::{Arc, Mutex};
use enigo::Mouse;

use crate::clicker::Clicker;
use crate::tasks::task::Task;

pub struct MacroTask;

impl Task for MacroTask {
    fn execute(&self, clicker: Arc<Mutex<Clicker>>, macro_positions: Arc<Mutex<Vec<(i32, i32)>>>){
        let positions = macro_positions.lock().unwrap();
        for &(x, y) in positions.iter() {
            let mut clicker = clicker.lock().unwrap();
            clicker.enigo.move_mouse(x, y, enigo::Coordinate::Abs).expect("Could not move mouse");
            clicker.click();
        }
    }
}