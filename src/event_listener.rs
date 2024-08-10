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
                match receiver.lock().unwrap().recv() {
                    Ok(key) => {
                        let new_command = match key {
                            Keycode::F6 => Command::StartClicking,
                            Keycode::F7 => Command::StartHolding,
                            Keycode::F9 => Command::SaveMousePosition,
                            Keycode::F10 => Command::StartMacro,
                            Keycode::Escape => Command::None,
                            _ => continue,
                        };
                        // Retry with backoff
                        let mut attempt = 0;
                        let max_attempts = 5;
                        let mut backoff_duration = Duration::from_millis(50);  // Initial backoff duration

                        while attempt < max_attempts {
                            if let Ok(mut clicker) = clicker.try_lock() {
                                clicker.handle_command(new_command);
                                break;
                            } else {
                                // Wait before retrying
                                thread::sleep(backoff_duration);
                                attempt += 1;
                                // Exponential backoff
                                backoff_duration *= 2;
                            }
                        }

                        if attempt == max_attempts {
                            eprintln!("Failed to acquire clicker lock after {} attempts.", max_attempts);
                        }
                    }
                    Err(mpsc::RecvError) => continue,
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