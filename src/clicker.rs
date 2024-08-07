use enigo::{Direction::{Press, Release, Click}, Enigo, Mouse, Settings, Button::Left};

pub struct Clicker {
    pub enigo: Enigo,
}

impl Clicker {
    pub fn new() -> Self {
        Clicker {
            enigo: {let enigo = Enigo::new(&Settings::default()).unwrap(); enigo},
        }
    }

    pub fn click(&mut self) {
        self.enigo.button(Left, Click).expect("Could not click");
    }

    pub fn hold_down(&mut self) {
        self.enigo.button(Left, Press).expect("Could not hold down");
    }

    pub fn release(&mut self) {
        self.enigo.button(Left, Release).expect("Could not release");
    }
}