use lovett::window_viewer::*;
use lovett::store::Store;
use lovett::gui_tk::*;
use super::super::state::State;
use super::super::state::filters::*;
use super::super::CONFIG;

//use log::*;
use std::sync::mpsc::*;

// we use 10x ints to have better precision than fpu
fn format_dec(num: i32) -> String {
    let num_vec = &num.to_string()[..];
    let mut output: String = String::new();
    output.push_str(&num_vec[0..(num_vec.len() - 1)]);
    output.push('.');
    output.push_str(&num_vec[(num_vec.len() - 1)..(num_vec.len())]);
    output
}

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

        let p_disp = format_dec(decoded_state.settings.p);
        if &p_disp[..] != objects[6].get_text() {
            objects[6].set_text(p_disp, canvas);
        } else {
        }

        let i_disp = format_dec(decoded_state.settings.i);
        if &i_disp[..] != objects[10].get_text() {
            objects[10].set_text(i_disp, canvas);
        } else {
        }

        let d_disp = format_dec(decoded_state.settings.d);
        if &d_disp[..] != objects[14].get_text() {
            objects[14].set_text(d_disp, canvas);
        } else {
        }


    };


    let (state_tx, state_rx) = channel();
    store.reg_state_sender(state_tx, SETTINGS_VIEW_FILTER);

    let mut settings_view = View::new( settings_update_fn, state_rx);

    // add buttons
    let up_temp: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 40, 18, 20, Event::new("[temp.click]", Some(vec!["up".to_string()])), &CONFIG)); 
    let dn_temp: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 61, 18, 20, Event::new("[temp.click]", Some(vec!["dn".to_string()])), &CONFIG)); 
    let temp_disp: Box<TextBlock> = Box::new(TextBlock::new("200".to_string(), 25, 45, 80, 32, Event::new("[temp]", None), &CONFIG));
    let temp_label: Box<TextBlock> = Box::new(TextBlock::new("°F : Target".to_string(), 90, 45, 100, 32, Event::new("[temp_label]", None), &CONFIG));

    // add p 
    let up_p: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 90, 18, 20, Event::new("[p.click]", Some(vec!["up".to_string()])), &CONFIG)); 
    let dn_p: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 111, 18, 20, Event::new("[p.click]", Some(vec!["dn".to_string()])), &CONFIG)); 
    let p_disp: Box<TextBlock> = Box::new(TextBlock::new("15.0".to_string(), 25, 95, 80, 32, Event::new("[p]", None), &CONFIG));
    let p_label: Box<TextBlock> = Box::new(TextBlock::new(" : P".to_string(), 90, 95, 100, 32, Event::new("[p_label]", None), &CONFIG));

    // add i
    let up_i: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 140, 18, 20, Event::new("[i.click]", Some(vec!["up".to_string()])), &CONFIG)); 
    let dn_i: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 161, 18, 20, Event::new("[i.click]", Some(vec!["dn".to_string()])), &CONFIG)); 
    let i_disp: Box<TextBlock> = Box::new(TextBlock::new("3.0".to_string(), 25, 145, 80, 32, Event::new("[i]", None), &CONFIG));
    let i_label: Box<TextBlock> = Box::new(TextBlock::new(" : I".to_string(), 90, 145, 100, 32, Event::new("[i_label]", None), &CONFIG));



    // add d
    let up_d: Box<Button>  = Box::new(Button::new("▲".to_string(), 0, 190, 18, 20, Event::new("[d.click]", Some(vec!["up".to_string()])), &CONFIG)); 
    let dn_d: Box<Button>  = Box::new(Button::new("▼".to_string(), 0, 211, 18, 20, Event::new("[d.click]", Some(vec!["dn".to_string()])), &CONFIG)); 
    let d_disp: Box<TextBlock> = Box::new(TextBlock::new("3.0".to_string(), 25, 195, 80, 32, Event::new("[d]", None), &CONFIG));
    let d_label: Box<TextBlock> = Box::new(TextBlock::new(" : D".to_string(), 90, 195, 100, 32, Event::new("[d_label]", None), &CONFIG));



    // add buttons to view
    settings_view.add_object(up_temp, 0, 0);
    settings_view.add_object(dn_temp, 1, 0);
    settings_view.add_static_object(temp_disp);
    settings_view.add_static_object(temp_label);


    // add buttons to view
    settings_view.add_object(up_p, 2, 0);
    settings_view.add_object(dn_p, 3, 0);
    settings_view.add_static_object(p_disp);
    settings_view.add_static_object(p_label);


    // add buttons to view
    settings_view.add_object(up_i, 4, 0);
    settings_view.add_object(dn_i, 5, 0);
    settings_view.add_static_object(i_disp);
    settings_view.add_static_object(i_label);

    // add buttons to view
    settings_view.add_object(up_d, 6, 0);
    settings_view.add_object(dn_d, 7, 0);
    settings_view.add_static_object(d_disp);
    settings_view.add_static_object(d_label);


    settings_view
}
