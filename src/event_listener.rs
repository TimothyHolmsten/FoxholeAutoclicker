use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use device_query::Keycode;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    StartClicking,
    StopClicking,
    StartHolding,
    StopHolding,
    SaveMousePosition,
    None,
    StartMacro,
    ClearMacro,
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
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
        let running = Arc::new(Mutex::new(None)); // Shared state for tracking commands

        thread::spawn(move || {
            let receiver = receiver.lock().unwrap();
            let running = Arc::clone(&running);

            loop {
                thread::park();
                match receiver.try_recv() {
                    Ok(key) => {
                        let mut running = running.lock().unwrap();

                        // Define a helper function to handle starting and stopping commands
                        let mut handle_command = |start_cmd: Command, stop_cmd: Command| {
                            match *running {
                                Some(current_cmd) if current_cmd == start_cmd => {
                                    // If the same command is running, stop it
                                    sender.send(stop_cmd).unwrap();
                                    *running = None; // Reset running state
                                }
                                Some(current_cmd) => {
                                    // Stop the currently running command
                                    let current_stop_cmd = match current_cmd {
                                        Command::StartClicking => Command::StopClicking,
                                        Command::StartHolding => Command::StopHolding,
                                        Command::StartMacro => Command::ClearMacro,
                                        _ => Command::None,
                                    };
                                    sender.send(current_stop_cmd).unwrap();
                                    // Start the new command
                                    sender.send(start_cmd).unwrap();
                                    *running = Some(start_cmd);
                                }
                                None => {
                                    // No command is running, just start the new command
                                    sender.send(start_cmd).unwrap();
                                    *running = Some(start_cmd);
                                }
                            }
                        };

                        match key {
                            Keycode::F6 => handle_command(Command::StartClicking, Command::StopClicking),
                            Keycode::F7 => handle_command(Command::StartHolding, Command::StopHolding),
                            Keycode::F9 => {
                                sender.send(Command::SaveMousePosition).unwrap();
                                sender.send(Command::None).unwrap();
                            },
                            Keycode::F10 => handle_command(Command::StartMacro, Command::ClearMacro),
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