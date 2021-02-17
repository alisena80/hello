use lovett::controller::{RootController, run_controller};
use lovett::views::*;
use lovett::state::{RootState,  run_state};

mod state;
use state::*;

mod time;
use time::time_keeper;

mod views;


pub fn run_app(app: App) {
    run_view(app.root_view);
    run_state(app.root_state);
    // join the last thread
    run_controller(app.root_controller).join().expect("Couldn't join on the associated thread");
}


pub struct App {
    pub root_controller: RootController,
    pub root_state: RootState,
    pub root_view: RootView,
}


impl App {

    pub fn new() -> App {

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


        // Initialize the RootController
        let root_controller = RootController::new();

        // create action sender for the root_view
        let action_sender = root_controller.get_action_sender();


        // add our views to the root view
        let root_view = views::setup(action_sender,  &mut root_state);


        App {
            root_view,
            root_controller,
            root_state,

        } 

    }


}
