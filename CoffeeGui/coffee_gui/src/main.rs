use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod joy_pad;
use joy_pad::Pad;
use joy_pad::ButtonInitializer;
use joy_pad::Action;


mod fb;
mod canvas;
use canvas::{ Canvas, Layer, Rect, Text};
use fb::Color;


mod views;
mod gui_tk;
mod actions;

use views::*;


mod state;
use state::{RootState, State, Mutator};

use chrono::prelude::*;

use rand::Rng;

extern crate framebuffer;
extern crate image;
extern crate rusttype;

fn main()  -> Result<(), Box<dyn Error>> {
  // setup the root state object
  let mut root_state = RootState::new(); 

  // get a state mutation sender for time keeping thread
  let time_mutator = root_state.getMutationSender();

  // register a subscriber for state ojbects
  let (root_view_sender, root_view_receiver) = mpsc::channel();
  root_state.regStateSender(root_view_sender);

  let mut root_view = RootView::new("/dev/fb1", root_view_receiver);

  let settings_view = SettingsView::new();

  root_view.addView(settings_view);

  root_view.setActiveView(0);

  run_view(root_view);

  let button_initializers = vec![
     ButtonInitializer {pin: 5, code: 0, key: "b"},
     ButtonInitializer {pin: 6, code: 1, key: "a"},
     ButtonInitializer {pin: 27, code: 2, key: "l"},
     ButtonInitializer {pin: 23, code: 3, key: "r"},
     ButtonInitializer {pin: 17, code: 4, key: "up"},
     ButtonInitializer {pin: 22, code: 5, key: "dn"},
     ButtonInitializer {pin: 4, code:  6, key: "hat"},
  
  ];

  let mut pad =  Pad::new(&button_initializers)?;
  //create channesl for threads to send data to central loop
  let (input_tx, input_rx) = mpsc::channel();
  thread::spawn(move || {
            loop {
                let button_actions = pad.detect_changes();
                input_tx.send(button_actions).unwrap();
                thread::sleep(Duration::from_millis(20));
            }

  });


  let controller_thread = thread::spawn(move || {
    loop {
        
        match input_rx.try_recv() {
            Ok(button_actions) => {
                for ba in &button_actions { 
                    match ba.action {
                        Action::Pressed => {},
                        Action::Repeated => {},
                        Action::Released => {}
                    }
                }
                joy_pad::helpers::ba_to_console(button_actions, &button_initializers);
            },
            Err(_) => ()
        }
        thread::sleep(Duration::from_millis(5));

    }

  

  });
  match controller_thread.join() {
        Ok(_) => (),
        Err(_) => ()
    }
  Ok(())

}



