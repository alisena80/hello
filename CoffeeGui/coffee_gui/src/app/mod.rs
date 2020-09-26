use lovett::joy_pad::{ Pad,  run_pad };
use lovett::controller::{RootController, run_controller};
use lovett::views::*;
use lovett::state::{RootState,  time_keeper, run_state};
use std::sync::mpsc;


mod state;
use state::*;


mod views;

pub fn run_app(app: App) {
    run_pad(app.input_pad);
    run_view(app.root_view);
    run_state(app.root_state);
    // join the last thread
    run_controller(app.root_controller).join().expect("Couldn't join on the associated thread");
}


pub struct App {
    pub root_controller: RootController,
    pub root_state: RootState,
    pub root_view: RootView,
    pub input_pad: Pad
}


impl App {

    pub fn new() -> App {
        let (joy_pad_input_rx, pad) = views::pad_input::setup();

        // Coffee Gui's State
        let state = State::new();

        // setup the root state object
        let mut root_state = RootState::new(bincode::serialize(&state).unwrap()); 

        // initialize the mutators so it can mutate
        state::mutators::setup(&mut root_state);


        // get a state mutation sender for time keeping thread
        // we setup a time keeper thread on a 1 second 
        // resolution to trigger initial state senders
        let time_mutator = root_state.get_mutation_sender();
        time_keeper(time_mutator);

        // register a subscriber for state ojbects
        let (root_view_state_sender, root_view_state_receiver) = mpsc::channel();
        root_state.reg_state_sender(root_view_state_sender);


        // Initialize the RootController
        let root_controller = RootController::new();

        // create action sender for the root_view
        let action_sender = root_controller.get_action_sender();


        // add our views to the root view
        let root_view = views::setup(root_view_state_receiver, joy_pad_input_rx, action_sender,  &mut root_state);


        App {
            root_view,
            root_controller,
            root_state,
            input_pad: pad

        } 

    }


}
