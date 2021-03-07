use lovett::store::*;
use super::*;
use log::*;

pub fn setup(store: &mut Store) {
        // create the mutator handlers
        let time_updater: Reducer = |state, action| {
            let  decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                let new_state = State{
                time: TimeState{ current_time: values[0].clone(), ..decoded_state.time
                    },
                ..decoded_state
                }; 
                bincode::serialize(&new_state).unwrap()
            }  else {
                state.to_vec()
            }
        };

        let thread_updater: Reducer = |state, action| {
            debug!("Call thread updater");
            let decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                let model_state: ModelState = match &values[1][..] {
                    "Running" => ModelState::Running,
                    "Empty" => ModelState::Empty,
                    "Ended" => ModelState::Ended,
                    _ => ModelState::Empty,
                };
                match &values[0][..] {
                    "time" => {
                        let new_state = State {
                            schedule: Schedule { time: model_state, ..decoded_state.schedule},
                            ..decoded_state
                        };

                        bincode::serialize(&new_state).unwrap()
                        },
                    _ => state.to_vec()
                }
            } else {
                state.to_vec()
            }
        };

        store.reducers.insert("[time.current_time]", time_updater);
        store.reducers.insert("[schedule.update_thread]", thread_updater);




}
