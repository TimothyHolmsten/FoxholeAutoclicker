use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread;

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
                println!("Notified");
                // Reset the notification state
                *notified = false;

                // Handle key events
                match receiver.lock().unwrap().try_recv() {
                    Ok(key) => {
                        let new_command = match key {
                            Keycode::F6 => Command::StartClicking,
                            Keycode::F7 => Command::StartHolding,
                            Keycode::F9 => Command::SaveMousePosition,
                            Keycode::F10 => Command::StartMacro,
                            Keycode::Escape => Command::None,
                            _ => continue,
                        };
                        println!("Locking clicker");
                        // Use try_lock to avoid blocking
                        if let Ok(mut clicker) = clicker.try_lock() {
                            clicker.handle_command(new_command);
                        } else {
                            // Handle the case where clicker lock could not be acquired
                            eprintln!("Failed to acquire clicker lock.");
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => continue,
                    Err(mpsc::TryRecvError::Disconnected) => break,
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