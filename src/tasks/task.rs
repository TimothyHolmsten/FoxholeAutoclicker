use std::sync::{Arc, Mutex};

use crate::clicker::Clicker;

pub trait Task: Send + Sync {
    fn execute(&self, clicker: Arc<Mutex<Clicker>>, macro_positions: Arc<Mutex<Vec<(i32, i32)>>>);
}