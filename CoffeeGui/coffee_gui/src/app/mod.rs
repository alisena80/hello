use super::joy_pad::{ Pad, ButtonInitializer, run_pad };
use super::controller::{RootController, run_controller};
use super::views::*;
use super::gui_tk::*;
use super::state::{RootState,  time_keeper, run_state};
use std::sync::mpsc;


pub fn run_app(app: App) {
    run_pad(app.input_pad);
    run_view(app.root_view);
    run_state(app.root_state);
    run_controller(app.root_controller); //joined thread
}


pub struct App {
    pub root_controller: RootController,
    pub root_state: RootState,
    pub root_view: RootView,
    pub input_pad: Pad
}


impl App {

    pub fn new() -> App{
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


        //create channesl for sending raw input buttons to the root_view
        let (input_tx, input_rx) = mpsc::channel();

        // setup the input_pad
        let pad =  match Pad::new(&button_initializers, input_tx) {
            Ok(pad) => pad,
            Err(x) => panic!("Error Starting Input Pad: {}", x)
        };


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


        App {
            root_view,
            root_controller,
            root_state,
            input_pad: pad

        } 

    }


}
