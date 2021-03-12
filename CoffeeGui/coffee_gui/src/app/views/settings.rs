use lovett::window_viewer::*;
use lovett::store::Store;
use lovett::gui_tk::*;
use super::super::state::State;
use super::super::state::filters::*;
//use log::*;
use std::sync::mpsc::*;


pub fn create(store: &mut Store) -> View {


    // create the settings view
    // - since its the first view its also the home view
    let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
        let decoded_state: State = super::super::state_decoder(state);
        // update all of this views things based on the value of state
        if &decoded_state.settings.target_temp.to_string()[..] != objects[2].get_text() {
            objects[2].set_text(decoded_state.settings.target_temp.to_string(), canvas);
        } else {
        }

        if &decoded_state.settings.p.to_string()[..] != objects[6].get_text() {
            objects[6].set_text(decoded_state.settings.p.to_string(), canvas);
        } else {
        }

        if &decoded_state.settings.i.to_string()[..] != objects[10].get_text() {
            objects[10].set_text(decoded_state.settings.i.to_string(), canvas);
        } else {
        }

        if &decoded_state.settings.d.to_string()[..] != objects[14].get_text() {
            objects[14].set_text(decoded_state.settings.d.to_string(), canvas);
        } else {
        }

    };


    let (state_tx, state_rx) = channel();
    store.reg_state_sender(state_tx, SETTINGS_VIEW_FILTER);

    let mut settings_view = View::new( settings_update_fn, state_rx);

    // add buttons
    let up_temp: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 40, 18, 20, Event::new("[temp.click]", Some(vec!["up".to_string()])))); 
    let dn_temp: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 61, 18, 20, Event::new("[temp.click]", Some(vec!["dn".to_string()])))); 
    let temp_disp: Box<TextBlock> = Box::new(TextBlock::new("200".to_string(), 25, 45, 80, 32, Event::new("[temp]", None)));
    let temp_label: Box<TextBlock> = Box::new(TextBlock::new("°F : Target".to_string(), 90, 45, 100, 32, Event::new("[temp_label]", None)));

    // add p 
    let up_p: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 70, 18, 20, Event::new("[p.click]", Some(vec!["up".to_string()])))); 
    let dn_p: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 91, 18, 20, Event::new("[p.click]", Some(vec!["dn".to_string()])))); 
    let p_disp: Box<TextBlock> = Box::new(TextBlock::new("15.0".to_string(), 25, 75, 80, 32, Event::new("[p]", None)));
    let p_label: Box<TextBlock> = Box::new(TextBlock::new(" : P".to_string(), 90, 75, 100, 32, Event::new("[p_label]", None)));

    // add i
    let up_i: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 100, 18, 20, Event::new("[i.click]", Some(vec!["up".to_string()])))); 
    let dn_i: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 121, 18, 20, Event::new("[i.click]", Some(vec!["dn".to_string()])))); 
    let i_disp: Box<TextBlock> = Box::new(TextBlock::new("3.0".to_string(), 25, 105, 80, 32, Event::new("[i]", None)));
    let i_label: Box<TextBlock> = Box::new(TextBlock::new(" : I".to_string(), 90, 105, 100, 32, Event::new("[i_label]", None)));



    // add d
    let up_d: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 130, 18, 20, Event::new("[d.click]", Some(vec!["up".to_string()])))); 
    let dn_d: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 151, 18, 20, Event::new("[d.click]", Some(vec!["dn".to_string()])))); 
    let d_disp: Box<TextBlock> = Box::new(TextBlock::new("3.0".to_string(), 25, 135, 80, 32, Event::new("[d]", None)));
    let d_label: Box<TextBlock> = Box::new(TextBlock::new(" : D".to_string(), 90, 135, 100, 32, Event::new("[d_label]", None)));



    // add buttons to view
    settings_view.add_object(up_temp, 0, 0);
    settings_view.add_object(dn_temp, 1, 0);
    settings_view.add_static_object(temp_disp);
    settings_view.add_static_object(temp_label);


    // add buttons to view
    settings_view.add_object(up_p, 0, 0);
    settings_view.add_object(dn_p, 1, 0);
    settings_view.add_static_object(p_disp);
    settings_view.add_static_object(p_label);


    // add buttons to view
    settings_view.add_object(up_i, 0, 0);
    settings_view.add_object(dn_i, 1, 0);
    settings_view.add_static_object(i_disp);
    settings_view.add_static_object(i_label);

    // add buttons to view
    settings_view.add_object(up_d, 0, 0);
    settings_view.add_object(dn_d, 1, 0);
    settings_view.add_static_object(d_disp);
    settings_view.add_static_object(d_label);


    settings_view
}
