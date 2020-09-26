use lovett::state::*;
use lovett::gui_tk::*;
use super::*

;
pub fn setup(root_state: &mut RootState) {
        // create the mutator handlers
        let time_updater: StateMutator = |state, mutator| {
            let mut decoded_state = state_decoder(state);
            decoded_state.time.current_time = mutator.value;
            bincode::serialize(&decoded_state).unwrap()
        };

        let selection_mover: StateMutator = |state, mutator| {
            let mut decoded_state = state_decoder(state);
            let current = decoded_state.views.get(mutator.value.as_str()).unwrap().iter().position(|x| match x { 
                GuiState::Selected => true,
                _ => false
            });
            match current {
                Some(position) => decoded_state.views.get_mut(mutator.value.as_str()).unwrap()[position] = GuiState::Base,
                _ => ()
            };
            decoded_state.views.get_mut(mutator.value.as_str()).unwrap()[mutator.number as usize] = GuiState::Selected;
            bincode::serialize(&decoded_state).unwrap()
        };


        let button_clicker: StateMutator = |state, mutator| {
            let mut decoded_state = state_decoder(state);
            decoded_state.views.get_mut(mutator.value.as_str()).unwrap()[mutator.number as usize] = GuiState::Clicked;
            bincode::serialize(&decoded_state).unwrap()
        };


        let button_releaser: StateMutator = |state, mutator| {
            let mut decoded_state = state_decoder(state);
            decoded_state.views.get_mut(mutator.value.as_str()).unwrap()[mutator.number as usize] = GuiState::Selected;
            bincode::serialize(&decoded_state).unwrap()
        };


        root_state.mutators.insert("[time.current_time]", time_updater);
        root_state.mutators.insert("[Move Selection To]", selection_mover);
        root_state.mutators.insert("[Clicked Button]", button_clicker);
        root_state.mutators.insert("[Released Button]", button_releaser);





}
