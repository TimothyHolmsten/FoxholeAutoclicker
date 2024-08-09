use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use device_query::Keycode;

use crate::clicker::{Clicker, Command};

pub struct EventListener {
    clicker: Arc<Mutex<Clicker>>,
    receiver: Arc<Mutex<mpsc::Receiver<Keycode>>>,
    condvar: Arc<(Mutex<bool>, Condvar)>,
}

impl EventListener {
    pub fn new(clicker: Arc<Mutex<Clicker>>, receiver: mpsc::Receiver<Keycode>) -> Self {
        EventListener { 
            clicker, 
            receiver: Arc::new(Mutex::new(receiver)),
            condvar: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn run(&self) {
        let clicker = Arc::clone(&self.clicker);
        let receiver = Arc::clone(&self.receiver);
        let condvar = Arc::clone(&self.condvar);

        thread::spawn(move || {
            loop {
                let (lock, cvar) = &*condvar;
                let mut notified = lock.lock().unwrap();

                // Wait until notified
                while !*notified {
                    notified = cvar.wait(notified).unwrap();
                }

                // Reset the notification state
                *notified = false;

                // Handle key events
                match receiver.lock().unwrap().recv_timeout(Duration::from_millis(100)) {
                    Ok(key) => {
                        let new_command = match key {
                            Keycode::F6 => Command::StartClicking,
                            Keycode::F7 => Command::StartHolding,
                            Keycode::F9 => Command::SaveMousePosition,
                            Keycode::F10 => Command::StartMacro,
                            Keycode::Escape => Command::None,
                            _ => continue,
                        };
                        let mut clicker = clicker.lock().unwrap();
                        clicker.handle_command(new_command);
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => continue,
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });
    }

    pub fn notify(&self) {
        let (lock, cvar) = &*self.condvar;
        let mut notified = lock.lock().unwrap();
        *notified = true;
        cvar.notify_one();
    }
}