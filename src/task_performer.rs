use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

use crate::clicker::Clicker;
use crate::event_listener::Command;
use crate::tasks::task::Task;

pub struct TaskPerformer {
    clicker: Arc<Mutex<Clicker>>,
    receiver: Arc<Mutex<mpsc::Receiver<Command>>>,
    macro_positions: Arc<Mutex<Vec<(i32, i32)>>>,
    current_task: Arc<Mutex<Option<Arc<dyn Task>>>>,
}

impl TaskPerformer {
    pub fn new(
        clicker: Arc<Mutex<Clicker>>,
        receiver: mpsc::Receiver<Command>,
    ) -> Self {
        TaskPerformer {
            clicker,
            receiver: Arc::new(Mutex::new(receiver)),
            macro_positions: Arc::new(Mutex::new(Vec::new())),
            current_task: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start(&self) -> thread::JoinHandle<()> {
        let clicker = Arc::clone(&self.clicker);
        let receiver = Arc::clone(&self.receiver);
        let macro_positions = Arc::clone(&self.macro_positions);
        let current_task = Arc::clone(&self.current_task);

        thread::spawn(move || {
            loop {
                // Receive a command from the channel
                let command = {
                    let receiver = receiver.lock().unwrap();
                    receiver.recv_timeout(Duration::from_millis(50)) // Adjusted timeout
                };

                match command {
                    Ok(command) => {
                        let mut task_lock = current_task.lock().unwrap();
                        let mut macro_lock = macro_positions.lock().unwrap(); // Lock macro_positions here

                        match command {
                            Command::StartClicking => {
                                *task_lock = Some(Arc::new(crate::tasks::click_task::ClickTask));
                            }
                            Command::StartHolding => {
                                *task_lock = Some(Arc::new(crate::tasks::hold_task::HoldTask));
                            }
                            Command::StopHolding => {
                                *task_lock = Some(Arc::new(crate::tasks::release_task::ReleaseTask));
                            }
                            Command::SaveMousePosition => {
                                *task_lock = Some(Arc::new(crate::tasks::save_position_task::SavePositionTask));
                            }
                            Command::StartMacro => {
                                *task_lock = Some(Arc::new(crate::tasks::macro_task::MacroTask));
                            }
                            Command::StopClicking => {
                                *task_lock = None;
                            }
                            Command::None => *task_lock = None,
                            Command::ClearMacro => {
                                macro_lock.clear(); // Clear macro_positions here
                                *task_lock = None;
                            }
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Minor delay to reduce CPU usage
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        println!("Task performer stopped because of disconnected");
                        break;
                    }
                }

                // Execute the current task if there is one
                let task_opt = {
                    let task_lock = current_task.lock().unwrap();
                    task_lock.clone()
                };

                if let Some(task) = task_opt {
                    task.execute(clicker.clone(), macro_positions.clone());
                }
            }
        })
    }
}