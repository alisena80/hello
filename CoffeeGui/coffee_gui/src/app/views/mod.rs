pub mod pad_input;

mod info_bar;
mod settings;
use lovett::joy_pad::{  run_pad };


use lovett::views::*;
use lovett::gui_tk::*;
use lovett::state::RootState;
use std::sync::mpsc::*;


pub fn setup(action_sender: Sender<Event>, root_state: &mut RootState) -> RootView {

    // setup input for views
    let (user_input_rx, pad) = pad_input::setup();

    // run input
    run_pad(pad);

    // decode state - since setting up views will generate state
    // strucutral changes - Adding a view
    let mut state = super::state_decoder(&root_state.state[..]);


    // create the info bar
    let info_bar = info_bar::create(root_state, &mut state);

    // create the root_view
    let mut root_view = RootView::new("/dev/fb1", user_input_rx, action_sender, info_bar);
    

    // create the settings voew
    let settings_view = settings::create(root_state, &mut state);

    // register the view and set it active
    root_view.add_view(settings_view);
    root_view.set_active_view(0);

    // register all state changes back to root state
    root_state.state = bincode::serialize(&state).unwrap();

    // return the root view so we can run it
    root_view
}
