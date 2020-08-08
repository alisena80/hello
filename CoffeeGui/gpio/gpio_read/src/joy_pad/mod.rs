use std::error::Error;

use rppal::gpio::Gpio;
use rppal::gpio::Level;
use rppal::gpio::InputPin;

use std::thread;
use std::time::Duration;
use std::sync::mpsc::Sender;

pub struct ButtonInitializer {
    pub pin: u8,
    pub code: u8
}

struct Button {
   pin: InputPin,
   state: Level,
   possible_state:Level,
   code: u8
}

impl Button {
    pub fn new(pin: InputPin, code: u8 ) -> Button {
        let mut state = pin.read();
        let mut possible_state = pin.read();    
        let button: Button = Button {
            pin: pin,
            state: state,
            possible_state: possible_state,
            code: code
        };
        button
    }

}

pub struct ButtonAction {
    pub action: Option<Action>,
    pub code: u8
}

pub enum Action {
    Pressed,
    Released,
}
pub struct Pad {
    buttons: Vec<Button>,
}

impl Pad {
  pub fn new( pins: Vec<ButtonInitializer>) -> Result<Pad, Box<dyn Error>> {
      let mut buttons : Vec<Button> = Vec::with_capacity(pins.len());
      let gpio = Gpio::new()?;
      for pin in &pins {
        let mut button = Button::new(gpio.get(pin.pin)?.into_input(), pin.code);
        buttons.push(button);
      }
      let pad: Pad = Pad {
        buttons: buttons
      };
     Ok(pad)
  }

  
  pub fn detect_changes(&mut self) -> Vec<ButtonAction> {
      let mut button_actions: Vec<ButtonAction> = Vec::with_capacity(self.buttons.len());

      for mut button in &mut self.buttons {
        let action : Option<Action> =  Pad::detect_button_changes(&mut button);
        button_actions.push(
            ButtonAction{
                action: action,
                code: button.code
            }  
        );
      
      }
      self.detect_possible_changes();
      button_actions
  }

  fn detect_possible_changes(&mut self) {
      for button in &mut self.buttons{
        button.possible_state = button.pin.read()
      }

  }

  fn detect_button_changes(button: &mut Button) -> Option<Action> {
      if button.possible_state != button.state {
          if button.pin.read() == button.possible_state {
              button.state = button.possible_state;
              if button.state == Level::Low {
                Some(Action::Pressed)
              } else {
                Some(Action::Released)
              }
          } else {
              button.possible_state = button.state;
              None
          }
      } else { 
          None
      }
    }
}


