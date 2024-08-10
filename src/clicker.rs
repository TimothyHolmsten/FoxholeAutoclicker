use enigo::{Direction::{Press, Release, Click}, Enigo, Mouse, Settings, Button::Left};
#[derive(Debug)]
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
                    _ => {
                        self.release();
                        self.macro_positions.clear();
                        self.set_state(ClickerState::Clicking)
                    }
                }
            },
            Command::StartHolding => {
                match self.state {
                    ClickerState::Idle => self.set_state(ClickerState::Holding),
                    ClickerState::Holding => {
                        self.release();
                        self.set_state(ClickerState::Idle);
                    },
                    _ => self.set_state(ClickerState::Holding)
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
                }
            },
            Command::StartMacro => {
                match self.state {
                    ClickerState::Idle | ClickerState::Clicking => {
                        self.set_state(ClickerState::ExecutingMacro);
                    }
                    ClickerState::ExecutingMacro => {
                        self.macro_positions.clear();
                        self.set_state(ClickerState::Idle)
                    },
                    ClickerState::Holding => {
                        self.release();
                        self.set_state(ClickerState::ExecutingMacro);
                    },
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
            ClickerState::ExecutingMacro => self.execute_macro(),
            ClickerState::Idle => {},
        }
    }

    fn click(&mut self) {
        self.enigo.button(Left, Click).expect("Could not click");
    }

    fn hold_down(&mut self) {
        if !self.holding {
            match self.enigo.button(Left, Press) {
                Ok(_) => self.holding = true,
                Err(err) => eprintln!("Could not press left mouse button: {}", err),
            }
        }
    }

    fn release(&mut self) {
        match self.enigo.button(Left, Release) {
            Ok(_) => self.holding = false,
            Err(err) => eprintln!("Could not release left mouse button: {}", err)
        }
    }

    fn save_mouse_position(&mut self) {
        match self.enigo.location() {
            Ok((x, y)) => self.macro_positions.push((x, y)),
            Err(_) => eprintln!("Could not get mouse location"),
        }
    }

    fn execute_macro(&mut self) {
        for &(x, y) in self.macro_positions.iter() {
            match self.enigo.move_mouse(x, y, enigo::Coordinate::Abs) {
                Ok(_) => match self.enigo.button(Left, Click) {
                    Ok(_) => (),
                    Err(_) => eprintln!("Could not perform click in macro"),
                },
                Err(_) => eprintln!("Could not move mouse in macro"),
            }
        }
    }
}