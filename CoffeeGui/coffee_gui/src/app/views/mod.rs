pub mod hid_button_pad;

mod info_bar;
mod settings;
use lovett::hid::{ run_button_pad };


use lovett::window_viewer::*;
use lovett::gui_tk::*;
use lovett::store::Store;
use std::sync::mpsc::*;


pub fn setup(event_sender: Sender<Event>, store: &mut Store) -> WindowViewer {

    // setup input for views
    let (user_input_rx, pad) = hid_button_pad::setup();

    // run input
    run_button_pad(pad);

    // decode state - since setting up views will generate state
    // strucutral changes - Adding a view


    // create the info bar
    let info_bar = info_bar::create(store);

    // create the window_viewer
    let mut window_viewer = WindowViewer::new("/dev/fb1", user_input_rx, event_sender, info_bar);
    

    // create the settings voew
    let settings_view = settings::create(store);

    // register the view and set it active
    window_viewer.add_view(settings_view);
    window_viewer.set_active_view(0);

    // return the root view so we can run it
    window_viewer
}
