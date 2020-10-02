pub mod pad_input;

use lovett::joy_pad::ButtonAction;
use lovett::views::{ RootView, ViewStateUpdater, View };
use super::state::State;
use lovett::gui_tk::*;
use lovett::state::RootState;
use std::sync::mpsc::*;

pub fn setup(root_view_state_receiver: Receiver<Vec<u8>>, joy_pad_input_rx: Receiver<Vec<ButtonAction>>, action_sender: Sender<GuiAction>, root_state: &mut RootState) -> RootView{

    //decode state
    let mut state = super::state_decoder(&root_state.state[..]);

    // create info_bar for the root_view
    // - requires a mutation sender to send mutator signals to the state
    // - requires a ViewStateUpdater fn to process state changes
    let info_bar_view_mutation_sender = root_state.get_mutation_sender();
    let bar_update_fn: ViewStateUpdater = |objects, state, canvas| {
        let decoded_state: State = super::state_decoder(state);
        if &decoded_state.time.current_time[..] != objects[1].get_text() {
            objects[1].set_text(decoded_state.time.current_time.clone(), canvas);
        }
    };
    let mut info_bar = View::new(info_bar_view_mutation_sender, "bar".to_string(), bar_update_fn);

    // register gui elements for the info bar
    let bar_clock: Box<TextBlock> = Box::new(TextBlock::new("00:00:00 XX".to_string(), 0, 0, 140, 28, GuiAction::new("Time Click", None)));
    let top_bar_block: Box<Block> = Box::new(Block::new(0,0, 240, 30, GuiAction::new("", None)));
    // add the button state tracker
    state.views.get_mut("bar").unwrap().push(bar_clock.get_gui_state());
    // add it to the view

    info_bar.add_static_object(top_bar_block);
    info_bar.add_static_object(bar_clock);

    // create the root_view
    let mut root_view = RootView::new("/dev/fb1", root_view_state_receiver, joy_pad_input_rx, action_sender, info_bar);

    // create the settings view
    // - since its the first view its also the home view
    let settings_view_mutation_sender = root_state.get_mutation_sender();
    let settings_update_fn: ViewStateUpdater = | objects, state, canvas | {
        let decoded_state: State = super::state_decoder(state);
        // decode state from Vec<u8>
        // update all of this views things based on the value of state
        if &decoded_state.time.current_time[..] != objects[0].get_text() {
            objects[0].set_text(decoded_state.time.current_time.clone(), canvas);
        }
        for i in 0..objects.len() {
            let current_state = objects[i].get_gui_state();
            let new_state = decoded_state.views.get("settings").unwrap()[i].clone();
            
            if let GuiState::Base = current_state {
                match new_state {
                    GuiState::Base => (),
                    _ => {
                        objects[i].set_gui_state(decoded_state.views.get("settings").unwrap()[i].clone(), canvas);
                    }
                }
            }
            if let GuiState::Clicked = current_state {
                match new_state {
                    GuiState::Clicked => (),
                    _ => {
                        objects[i].set_gui_state(decoded_state.views.get("settings").unwrap()[i].clone(), canvas);
                    }
                }
            }
            if let GuiState::Selected = current_state {
                match new_state {
                    GuiState::Selected => (),
                    _ => {
                        objects[i].set_gui_state(decoded_state.views.get("settings").unwrap()[i].clone(), canvas);
                    }
                }
            }
        } 
    };

    let mut settings_view = View::new(settings_view_mutation_sender, "settings".to_string(), settings_update_fn);

    // add buttons
    let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 28, 200, 32, GuiAction::new("Time Click", None))); 
    let button2: Box<Button> = Box::new(Button::new("X".to_string(), 0, 90, 20, 32, GuiAction::new("Time Click", None))); 
    let button3: Box<Button> = Box::new(Button::new("Y".to_string(), 100, 150, 20, 32, GuiAction::new("Time Click", None))); 
    let button4: Box<Button> = Box::new(Button::new("Z".to_string(), 0, 150, 20, 32, GuiAction::new("Time Click", None))); 
   
    // add buttons to state
    state.views.get_mut("settings").unwrap().push(button.get_gui_state());
    state.views.get_mut("settings").unwrap().push(button2.get_gui_state());
    state.views.get_mut("settings").unwrap().push(button3.get_gui_state());
    state.views.get_mut("settings").unwrap().push(button4.get_gui_state());

    // add buttons to view
    settings_view.add_object(button, 0, 0);
    settings_view.add_object(button2, 1, 0);
    settings_view.add_object(button3, 2, 2);
    settings_view.add_object(button4, 2, 0);

    root_view.add_view(settings_view);
    root_view.set_active_view(0);

    // register all state changes back to root state
    root_state.state = bincode::serialize(&state).unwrap();

    // return the root view so we can run it
    root_view
}
