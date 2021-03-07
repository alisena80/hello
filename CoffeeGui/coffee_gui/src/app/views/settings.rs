use lovett::window_viewer::*;
use lovett::store::Store;
use lovett::gui_tk::*;
use super::super::state::State;
use super::super::state::filters::*;

use std::sync::mpsc::*;


pub fn create(store: &mut Store) -> View {


    // create the settings view
    // - since its the first view its also the home view
    let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
        let decoded_state: State = super::super::state_decoder(state);
        // update all of this views things based on the value of state
        if &decoded_state.settings.target_temp.to_string()[..] != objects[2].get_text() {
            objects[2].set_text(decoded_state.settings.target_temp.to_string(), canvas);
        }

    };


    let (state_tx, state_rx) = channel();
    store.reg_state_sender(state_tx, SETTINGS_VIEW_FILTER);

    let mut settings_view = View::new( settings_update_fn, state_rx);

    // add buttons
    let up_temp: Box<Button>  = Box::new(Button::new("^".to_string(), 0, 40, 24, 24, Event::new("[temp.click]", Some(vec!["up".to_string()])))); 
    let dn_temp: Box<Button>  = Box::new(Button::new("v".to_string(), 0, 65, 24, 24, Event::new("[temp.click]", Some(vec!["dn".to_string()])))); 
    let temp_disp: Box<TextBlock> = Box::new(TextBlock::new("xxx".to_string(), 25, 40, 80, 28, Event::new("[temp]", None)));
 
    // add buttons to view
    settings_view.add_object(up_temp, 0, 0);
    settings_view.add_object(dn_temp, 1, 0);
    settings_view.add_static_object(temp_disp);


    settings_view
}
