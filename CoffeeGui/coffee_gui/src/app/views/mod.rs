pub mod hid_button_pad;

mod info_bar;
mod brew_settings;
mod time_settings;
use lovett::hid::{ run_button_pad };


use lovett::window_viewer::*;
use lovett::gui_tk::*;
use lovett::store::Store;
use std::sync::mpsc::*;


use super::State; 
use super::state::filters::*;


pub fn setup(event_sender: Sender<Event>, store: &mut Store) -> WindowViewer {
    // setup input for views
    let (user_input_rx, pad) = hid_button_pad::setup();

    // run input
    run_button_pad(pad);

    // decode state - since setting up views will generate state
    // strucutral changes - Adding a view


    // create the info bar
    let info_bar = info_bar::create(store);


    let update_fn: WindowViewerUpdater = |window_viewer, state| -> bool{
        let decoded_state: State = super::state_decoder(state);
        if &decoded_state.window.active != &window_viewer.active {
            window_viewer.set_active_view(decoded_state.window.active);
            return true
        }
        false
    };

    let (viewer_state_sender, viewer_state_receiver) = channel();
    store.reg_state_sender(viewer_state_sender, WINDOW_VIEWER_FILTER); 

    // create the window_viewer
    let mut window_viewer = WindowViewer::new("/dev/fb1", user_input_rx, event_sender,  viewer_state_receiver, update_fn, info_bar);
    

    // create the settings voew
    let brew_settings_view = brew_settings::create(store);

    // create the settings voew
    let time_settings_view = time_settings::create(store);


    // register the view and set it active
    window_viewer.add_view(brew_settings_view);
    window_viewer.add_view(time_settings_view);
    window_viewer.set_active_view(0);

    // return the root view so we can run it
    window_viewer
}
