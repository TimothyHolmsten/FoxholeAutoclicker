use std::{thread, time::Duration};

use enigo::{Direction::{Press, Release, Click}, Enigo, Mouse, Settings, Button::Left};

pub enum Command {
    StartClicking,
    StartHolding,
    SaveMousePosition,
    StartMacro,
    None,
}

pub struct Clicker {
    enigo: Enigo,
    macro_positions: Vec<(i32, i32)>,
    state: ClickerState,
    holding: bool
}
#[derive(Clone, Debug)]
enum ClickerState {
    Idle,
    Clicking,
    Holding,
    Releasing(Box<ClickerState>),
    SavingPosition,
    ExecutingMacro,
}

impl Clicker {
    pub fn new() -> Self {
        Clicker {
            enigo: {let enigo = Enigo::new(&Settings::default()).unwrap(); enigo},
            macro_positions: Vec::new(),
            state: ClickerState::Idle,
            holding: false
        }
    }

    pub fn handle_command(&mut self, command: Command) {
        match command {
            Command::StartClicking => {
                match self.state {
                    ClickerState::Idle => self.set_state(ClickerState::Clicking),
                    ClickerState::Clicking => self.set_state(ClickerState::Idle),
                    ClickerState::Holding => {
                        self.set_state(ClickerState::Releasing(Box::new(ClickerState::Clicking)))
                    },
                    ClickerState::SavingPosition => {
                        self.macro_positions.clear();
                        self.set_state(ClickerState::Clicking)
                    },
                    ClickerState::ExecutingMacro => {
                        self.macro_positions.clear();
                        self.set_state(ClickerState::Clicking);
                    },
                    ClickerState::Releasing(_) => todo!(),
                }
            },
            Command::StartHolding => {
                match self.state {
                    ClickerState::Idle | ClickerState::Clicking => {
                        self.set_state(ClickerState::Releasing(Box::new(ClickerState::Holding)));
                    }
                    ClickerState::Holding => {
                        self.set_state(ClickerState::Releasing(Box::new(ClickerState::Idle)));
                    },
                    ClickerState::SavingPosition | ClickerState::ExecutingMacro => {
                        self.set_state(ClickerState::Holding);
                    },
                    ClickerState::Releasing(_) => todo!(),
                }
            },
            Command::SaveMousePosition => {
                match self.state {
                    ClickerState::Idle => {
                        self.save_mouse_position();
                        self.set_state(ClickerState::Idle);
                    }
                    ClickerState::Clicking | ClickerState::Holding | ClickerState::ExecutingMacro => {
                        self.macro_positions.clear();
                        self.save_mouse_position();
                        self.set_state(ClickerState::Idle);
                    }
                    ClickerState::SavingPosition => {
                        self.save_mouse_position();
                        self.set_state(ClickerState::Idle);
                    }
                    ClickerState::Releasing(_) => todo!(),
                }
            },
            Command::StartMacro => {
                match self.state {
                    ClickerState::Idle | ClickerState::Clicking | ClickerState::SavingPosition => {
                        self.set_state(ClickerState::ExecutingMacro);
                    }
                    ClickerState::ExecutingMacro => {
                        self.macro_positions.clear();
                        self.set_state(ClickerState::Idle)
                    },
                    ClickerState::Holding => {
                        self.set_state(ClickerState::Releasing(Box::new(ClickerState::ExecutingMacro)));
                    },
                    ClickerState::Releasing(_) => todo!(),
                }
            },
            Command::None => {
                self.set_state(ClickerState::Idle);
            },
        }
    }

    fn set_state(&mut self, state: ClickerState) {
        self.state = state;
    }

    pub fn execute(&mut self) {
        match &self.state {
            ClickerState::Clicking => self.click(),
            ClickerState::Holding => self.hold_down(),
            ClickerState::Releasing(state) => {
                match **state {
                    ClickerState::Releasing(_) => {
                        self.release();
                        self.set_state(ClickerState::Idle);
                    },
                    ClickerState::Idle => {
                        self.release();
                        self.set_state(ClickerState::Idle);
                    },
                    ClickerState::Clicking => {
                        self.release();
                        self.set_state(ClickerState::Clicking)
                    },
                    ClickerState::Holding => {
                        self.release();
                        self.set_state(ClickerState::Holding);
                    },
                    ClickerState::SavingPosition => {
                        self.release();
                        self.set_state(ClickerState::SavingPosition);
                    },
                    ClickerState::ExecutingMacro => {
                        self.release();
                        self.set_state(ClickerState::ExecutingMacro);
                    },
                    
                }
            },
            ClickerState::SavingPosition => self.save_mouse_position(),
            ClickerState::ExecutingMacro => self.execute_macro(),
            ClickerState::Idle => {},
        }
    }

    fn click(&mut self) {
        self.enigo.button(Left, Click).expect("Could not click");
    }

    fn hold_down(&mut self) {
        if !self.holding {
            self.enigo.button(Left, Press).expect("Could not hold down");
            self.holding = true;
        }
    }

    fn release(&mut self) {
        thread::sleep(Duration::from_millis(50));
        self.holding = false;
        self.enigo.button(Left, Release).expect("Could not release");
    }

    fn save_mouse_position(&mut self) {
        let (x, y) = self.enigo.location().unwrap();
        self.macro_positions.push((x, y));
    }

    fn execute_macro(&mut self) {
        for &(x, y) in self.macro_positions.iter() {
            let _ = self.enigo.move_mouse(x, y, enigo::Coordinate::Abs);
            self.enigo.button(Left, Click).expect("Could not click");
        }
    }
}