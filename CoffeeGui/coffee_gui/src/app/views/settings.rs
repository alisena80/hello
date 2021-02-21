use lovett::views::*;
use lovett::state::RootState;
use lovett::gui_tk::*;
use super::super::state::State;
use super::super::state::filters::*;

use std::sync::mpsc::*;


pub fn create(root_state: &mut RootState) -> View {


    // create the settings view
    // - since its the first view its also the home view
    let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
        let decoded_state: State = super::super::state_decoder(state);
        // update all of this views things based on the value of state
        if &decoded_state.time.current_time[..] != objects[0].get_text() {
            objects[0].set_text(decoded_state.time.current_time.clone(), canvas);
        }

    };


    let (state_tx, state_rx) = channel();
    root_state.reg_state_sender(state_tx, SETTINGS_VIEW_FILTER);

    let mut settings_view = View::new( settings_update_fn, state_rx);

    // add buttons
    let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 30, 200, 32, Event::new("Time Click", None))); 
    let button2: Box<Button> = Box::new(Button::new("X".to_string(), 0, 90, 32, 32, Event::new("Time Click", None))); 
    let button3: Box<Button> = Box::new(Button::new("Y".to_string(), 100, 150, 32, 32, Event::new("Time Click", None))); 
    let button4: Box<Button> = Box::new(Button::new("Z".to_string(), 0, 150, 32, 32, Event::new("Time Click", None))); 
   
    // add buttons to view
    settings_view.add_object(button, 0, 0);
    settings_view.add_object(button2, 1, 0);
    settings_view.add_object(button3, 2, 2);
    settings_view.add_object(button4, 2, 0);



    settings_view
}
