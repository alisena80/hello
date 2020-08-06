use std::error::Error;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::InputPin;

struct PadValues {
  b: Level,
  a: Level,
  l: Level,
  r: Level,
  up: Level,
  dn: Level,
  hat: Level
}

impl PadValues {
  fn new(b: &InputPin, a: &InputPin, l: &InputPin, r: &InputPin, up: &InputPin, dn: &InputPin, hat: &InputPin) -> PadValues{
    PadValues {
        b: b.read(),
        a: a.read(),
        l: l.read(),
        r: r.read(),
        up: up.read(),
        dn: dn.read(),
        hat: hat.read()
      
    }
  }

}
pub enum Action {
    Pressed,
    Released,
}
pub struct Presses {
    pub a: Option<Action>,
    pub b: Option<Action>,
    pub l: Option<Action>,
    pub r: Option<Action>,
    pub up: Option<Action>,
    pub dn: Option<Action>,
    pub hat: Option<Action>,
}

pub struct Pad {
    b: InputPin,
    a: InputPin,
    l: InputPin,
    r: InputPin,
    up: InputPin,
    dn: InputPin,
    hat: InputPin,

    state: PadValues,
    possible_state: PadValues,

}

impl Pad {
  pub fn new(pin_b: u8, pin_a: u8, pin_l: u8, pin_r: u8, pin_up: u8, pin_dn: u8, pin_hat: u8) -> Result<Pad, Box<dyn Error>> {
      let gpio = Gpio::new()?;
      let b = gpio.get(pin_b)?.into_input();
      let a = gpio.get(pin_a)?.into_input();
      let l = gpio.get(pin_l)?.into_input();
      let r = gpio.get(pin_r)?.into_input();
      let up = gpio.get(pin_up)?.into_input();
      let dn = gpio.get(pin_dn)?.into_input();
      let hat = gpio.get(pin_hat)?.into_input();

      let state = PadValues::new(&b, &a, &l, &r, &up, &dn,  &hat);
     
      let possible_state = PadValues::new(&b, &a, &l, &r, &up, &dn, &hat);
      let pad: Pad = Pad {
        b,
        a,
        l,
        r,
        up,
        dn,
        hat,
        state,
        possible_state
      };
     Ok(pad)
  }

  pub fn detect_changes(&mut self) -> Presses {
        let b = Pad::detect_button_change( &self.b, &mut self.state.b, &mut self.possible_state.b);
        let a = Pad::detect_button_change( &self.a, &mut self.state.a, &mut self.possible_state.a);
        let l = Pad::detect_button_change( &self.l, &mut self.state.l, &mut self.possible_state.l);
        let r = Pad::detect_button_change( &self.r, &mut self.state.r, &mut self.possible_state.r);
        let up = Pad::detect_button_change( &self.up, &mut self.state.up, &mut self.possible_state.up);
        let dn = Pad::detect_button_change( &self.dn, &mut self.state.dn, &mut self.possible_state.dn);
        let hat = Pad::detect_button_change( &self.hat, &mut self.state.hat, &mut self.possible_state.hat);
        let button_presses = Presses {b, a, l, r, up, dn, hat};
        self.detect_possible_changes();
        button_presses
  }

  fn detect_possible_changes(&mut self) {
    self.possible_state.b = self.b.read();
    self.possible_state.a = self.a.read();
    self.possible_state.l = self.l.read();
    self.possible_state.r = self.r.read();
    self.possible_state.up = self.up.read();
    self.possible_state.dn = self.dn.read();
    self.possible_state.hat = self.hat.read();

  }

  fn detect_button_change<'l>(button: &InputPin, state: &'l mut Level, possible_state: &'l mut Level) -> Option<Action> {
      if possible_state != state {
          if button.read() == *possible_state {
              *state = *possible_state;
              if *state == Level::Low {
                Some(Action::Pressed)
              } else {
                Some(Action::Released)
              }
          } else {
              *possible_state = *state;
              None
          }
      } else { 
          None
      }
    }
}


