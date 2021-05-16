use lovett::window_viewer::*;
use lovett::store::Store;
use lovett::gui_tk::*;
use super::super::state::State;
use super::super::state::filters::*;
use super::super::CONFIG;

//use log::*;
use std::sync::mpsc::*;

#[allow(dead_code, unused_variables)]
pub fn create(store: &mut Store) -> View {


    // create the settings view
    // - since its the first view its also the home view
    let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
        let decoded_state: State = super::super::state_decoder(state);


    };


    let (state_tx, state_rx) = channel();
    store.reg_state_sender(state_tx, SETTINGS_VIEW_FILTER);

    let mut settings_view = View::new( settings_update_fn, state_rx);

    // add label
    let page_label: Box<TextBlock>      = Box::new(TextBlock::new("Time Settings".to_string(), 32, 30, 186, 24, Event::new("[settings_label]", None), &CONFIG));
    let page_left: Box<Button>          = Box::new(Button::new("◀".to_string(), 0, 33, 16, 22, 2, 3, 15, Event::new("[settings_pager.click]", Some(vec!["left".to_string()])), &CONFIG));
    let page_right: Box<Button>         = Box::new(Button::new("▶".to_string(), 223, 33, 16, 22, 1, 3, 15, Event::new("[settings_pager.click]", Some(vec!["right".to_string()])), &CONFIG));
    // pager
    settings_view.add_static_object(page_label);
    settings_view.add_object(page_left, 0, 0);
    settings_view.add_object(page_right, 0, 1);

    settings_view
}
