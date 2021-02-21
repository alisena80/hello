use lovett::state::*;
use super::*;


pub fn setup(root_state: &mut RootState) {
        // create the mutator handlers
        let time_updater: Mutator = |state, mutator| {
            let mut decoded_state = state_decoder(state);
            decoded_state.time.current_time = mutator.value;
            bincode::serialize(&decoded_state).unwrap()
        };


        root_state.mutators.insert("[time.current_time]", time_updater);





}
