use lovett::joy_pad::{ Pad, ButtonInitializer, run_pad };
use lovett::controller::{RootController, run_controller};
use lovett::views::*;
use lovett::gui_tk::*;
use lovett::state::{RootState,  time_keeper, run_state};
use std::sync::mpsc;

use std::collections::HashMap;

mod state;
use state::*;

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

        
        let mut views: HashMap<String, Vec<GuiState>> = HashMap::new();
        views.insert("bar".to_string(), vec![]);
        views.insert("boiler".to_string(), vec![]);
        views.insert("steamer".to_string(), vec![]);
        views.insert("settings".to_string(), vec![]);
        


        let mut state = State {
                    boiler: BoilerState {
                        element_on: false,
                        temperature: 0
                    },
                    tank: TankState {
                        level: 0
                    },
                    time: TimeState {
//                        turned_on: SystemTime::now(),
                        current_time: "00:00:00 XX".to_string()
                    },
                    settings: SettingsState {
                        running: false,
                        p: 0,
                        i: 0,
                        d: 0
                    },
                    views,
                };


        // setup the root state object
        let mut root_state = RootState::new(bincode::serialize(&state).unwrap()); 

        // get a state mutation sender for time keeping thread
        // we setup a time keeper thread on a 1 second resolution to trigger initial state senders
        let time_mutator = root_state.get_mutation_sender();
        time_keeper(time_mutator);

        // register a subscriber for state ojbects
        let (root_view_sender, root_view_receiver) = mpsc::channel();
        root_state.reg_state_sender(root_view_sender);
        let settings_view_mutation_sender = root_state.get_mutation_sender();

        let info_bar_view_mutation_sender = root_state.get_mutation_sender();

        let bar_update_fn: ViewStateUpdater = |objects, state, canvas| {
            let decoded_state: State = state_decoder(state);
            objects[0].set_text(decoded_state.time.current_time.clone(), canvas);

        };

        let mut info_bar = View::new(info_bar_view_mutation_sender, "bar".to_string(), bar_update_fn);

        let bar_button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 0, 140, 28, GuiAction::new("Time Click", None)));
        state.views.get_mut("bar").unwrap().push(bar_button.gui_state.clone());
        info_bar.add_object(bar_button, 0, 0);


        fn state_decoder(state: &[u8]) -> State{
            bincode::deserialize(state).unwrap()
        }

        // Initialize the RootController
        let root_controller = RootController::new();
        let action_sender = root_controller.get_action_sender();
        let mut root_view = RootView::new("/dev/fb1", root_view_receiver, input_rx, action_sender, info_bar);
        let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
            let decoded_state: State = state_decoder(state);
            // decode state from Vec<u8>
            // update all of this views things based on the value of state
            objects[0].set_text(decoded_state.time.current_time.clone(), canvas);
            for i in 0..objects.len() {
              objects[i].set_gui_state(decoded_state.views.get("settings").unwrap()[i].clone(), canvas);
            } 

        };

        let mut settings_view = View::new(settings_view_mutation_sender, "settings".to_string(), settings_update_fn);
        let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 28, 200, 32, GuiAction::new("Time Click", None))); 
        let button2: Box<Button> = Box::new(Button::new("X".to_string(), 200, 90, 10, 32, GuiAction::new("Time Click", None))); 
        let button3: Box<Button> = Box::new(Button::new("Y".to_string(), 220, 90, 10, 32, GuiAction::new("Time Click", None))); 
        
        //add buttons to state
        state.views.get_mut("settings").unwrap().push(button.get_gui_state());
        state.views.get_mut("settings").unwrap().push(button2.get_gui_state());
        state.views.get_mut("settings").unwrap().push(button3.get_gui_state());

       // add buttons to view
        settings_view.add_object(button, 0, 0);
        settings_view.add_object(button2, 0, 0);
        settings_view.add_object(button3, 0, 2);

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
