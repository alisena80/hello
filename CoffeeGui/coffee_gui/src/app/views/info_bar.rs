use lovett::window_viewer::*;
use lovett::store::Store;
use lovett::gui_tk::*;
use super::super::state::State;
use super::super::state::filters::*;

use std::sync::mpsc::*;


pub fn create(store: &mut Store) -> View {
    // create info_bar for the window_viewer
    // - requires a mutation sender to send mutator signals to the state
    // - requires a ViewStateUpdater fn to process state changes
    let bar_update_fn: ViewStateUpdater = |objects, state, canvas| {
        let decoded_state: State = super::super::state_decoder(state);
        if &decoded_state.time.current_time[..] != objects[1].get_text() {
            objects[1].set_text(decoded_state.time.current_time.clone(), canvas);
        }
    };

    let (bar_view_state_sender, bar_view_state_receiver) = channel();
    store.reg_state_sender(bar_view_state_sender, BAR_VIEW_FILTER); 

    let mut info_bar = View::new(bar_update_fn, bar_view_state_receiver);

    // register gui elements for the info bar
    let bar_clock: Box<TextBlock> = Box::new(TextBlock::new("00:00:00 XX".to_string(), 4, 0, 140, 28, Event::new("Time Click", None)));
    let top_bar_block: Box<Block> = Box::new(Block::new(0,0, 240, 30, Event::new("", None)));
    // add the button state tracker
    // add it to the view

    info_bar.add_static_object(top_bar_block);
    info_bar.add_static_object(bar_clock);

    info_bar
}
