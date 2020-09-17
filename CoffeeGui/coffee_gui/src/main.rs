// Crates
extern crate framebuffer;
extern crate image;
extern crate rusttype;

// Std lib imports
use std::sync::mpsc;

// Modules
mod joy_pad;
mod fb;
mod canvas;
mod views;
mod gui_tk;
mod controllers;
mod state;

// Basic module bits
use joy_pad::{ Pad, ButtonInitializer, run_pad };
use controllers::{RootController, run_controller};
use views::*;
use gui_tk::*;
use state::{RootState,  time_keeper, run_state};

fn main()  { 
    // setup hw buttons
    let button_initializers = vec![
        ButtonInitializer {pin: 5, code: 0, key: "b"},
        ButtonInitializer {pin: 6, code: 1, key: "a"},
        ButtonInitializer {pin: 27, code: 2, key: "l"},
        ButtonInitializer {pin: 23, code: 3, key: "r"},
        ButtonInitializer {pin: 17, code: 4, key: "up"},
        ButtonInitializer {pin: 22, code: 5, key: "dn"},
        ButtonInitializer {pin: 4, code:  6, key: "hat"},
    ];

    let (input_tx, input_rx) = mpsc::channel();
    let pad =  match Pad::new(&button_initializers, input_tx) {
        Ok(pad) => pad,
        Err(x) => panic!("Error Starting Input Pad: {}", x)
    };

    //create channesl for threads to send data to central loop
    run_pad(pad);

    // setup the root state object
    let mut root_state = RootState::new(); 

    // get a state mutation sender for time keeping thread
    // we setup a time keeper thread on a 1 second resolution to trigger initial state senders
    let time_mutator = root_state.get_mutation_sender();
    time_keeper(time_mutator);

    // register a subscriber for state ojbects
    let (root_view_sender, root_view_receiver) = mpsc::channel();
    root_state.reg_state_sender(root_view_sender);
    let view_mutation_sender = root_state.get_mutation_sender();

    // Initialize the RootController
    let root_controller = RootController::new();
    let action_sender = root_controller.get_action_sender();
    let mut root_view = RootView::new("/dev/fb1", root_view_receiver, &mut root_state, input_rx, action_sender);
    let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
        objects[0].set_text(state.time.current_time.clone(), canvas);
    };

    let mut settings_view = View::new(view_mutation_sender, "settings".to_string(), settings_update_fn);
    let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 28, 200, 32, GuiAction::new("Time Click", None))); 
    let button2: Box<Button> = Box::new(Button::new("X".to_string(), 200, 90, 10, 32, GuiAction::new("Time Click", None))); 
    let button3: Box<Button> = Box::new(Button::new("Y".to_string(), 220, 90, 10, 32, GuiAction::new("Time Click", None))); 

    settings_view.add_object(button, 0, 0, &mut root_state);
    settings_view.add_object(button2, 0, 0, &mut root_state);
    settings_view.add_object(button3, 0, 2, &mut root_state);

    root_view.add_view(settings_view);
    root_view.set_active_view(0);

    run_view(root_view);
    run_state(root_state);
    run_controller(root_controller); //joined thread

}
