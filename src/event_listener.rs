use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use device_query::Keycode;

#[derive(Debug)]
pub enum Command {
    StartClicking,
    StopClicking,
    StartHolding,
    StopHolding,
    SaveMousePosition,
    StartMacro,
    ClearMacro,
    None,
}

pub struct EventListener {
    sender: mpsc::Sender<Command>,
    receiver: Arc<Mutex<mpsc::Receiver<Keycode>>>,
}

impl EventListener {
    pub fn new(sender: mpsc::Sender<Command>, receiver: mpsc::Receiver<Keycode>) -> Self {
        EventListener { 
            sender, 
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let sender = self.sender.clone();
        let receiver = Arc::clone(&self.receiver);
        let mut running = false;

        thread::spawn(move || {
            let receiver = receiver.lock().unwrap();

            loop {
                thread::park();
                match receiver.try_recv() {
                    Ok(key) => {
                        // Define a helper function to send commands and update `running`
                        let mut toggle_running = |start_cmd, stop_cmd| {
                            if running {
                                sender.send(stop_cmd).unwrap();
                            } else {
                                sender.send(start_cmd).unwrap();
                            }
                            running = !running;
                        };

                        match key {
                            Keycode::F6 => toggle_running(Command::StartClicking, Command::StopClicking),
                            Keycode::F7 => toggle_running(Command::StartHolding, Command::StopHolding),
                            Keycode::F9 => {
                                sender.send(Command::SaveMousePosition).unwrap();
                                sender.send(Command::None).unwrap();
                            },
                            Keycode::F10 => toggle_running(Command::StartMacro, Command::ClearMacro),
                            _ => (),
                        }
                    }
                    Err(mpsc::TryRecvError::Empty) => {},
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
        })
    }
}