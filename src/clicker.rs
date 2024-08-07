use enigo::{Direction::{Press, Release, Click}, Enigo, Mouse, Settings, Button::Left};

pub struct Clicker {
    pub enigo: Enigo,
    pub holding: bool
}

impl Clicker {
    pub fn new() -> Self {
        Clicker {
            enigo: {let enigo = Enigo::new(&Settings::default()).unwrap(); enigo},
            holding: false
        }
    }

    pub fn click(&mut self) {
        self.enigo.button(Left, Click).expect("Could not click");
        self.holding = false;
    }

    pub fn hold_down(&mut self) {
        if !self.holding {
            self.enigo.button(Left, Press).expect("Could not hold down");
            self.holding = true;
        }
    }

    pub fn release(&mut self) {
        if self.holding {
            self.enigo.button(Left, Release).expect("Could not release");
            self.holding = false;
        }
    }
}