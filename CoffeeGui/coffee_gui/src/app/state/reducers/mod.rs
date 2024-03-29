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
        let set_target_temp: Reducer = |state, action| {
            debug!("update target temp");
            let decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                match &values[0][..] {
                    "up" => {
                        let new_state = State {
                            settings: SettingsState { target_temp: decoded_state.settings.target_temp + 1, ..decoded_state.settings},
                            ..decoded_state
                        };
                        bincode::serialize(&new_state).unwrap()
                    },
                    "dn" => {
                        let new_state = State {
                            settings: SettingsState { target_temp: decoded_state.settings.target_temp - 1, ..decoded_state.settings},
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

        let set_p: Reducer = |state, action| {
            let decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                match &values[0][..] {
                    "up" => {
                        let new_state = State {
                            settings: SettingsState { p: decoded_state.settings.p + 1, ..decoded_state.settings},
                            ..decoded_state
                        };
                        bincode::serialize(&new_state).unwrap()
                    },
                    "dn" => {
                        let new_state = State {
                            settings: SettingsState { p: decoded_state.settings.p - 1, ..decoded_state.settings},
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

        let set_i: Reducer = |state, action| {
            let decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                match &values[0][..] {
                    "up" => {
                        let new_state = State {
                            settings: SettingsState { i: decoded_state.settings.i + 1, ..decoded_state.settings},
                            ..decoded_state
                        };
                        bincode::serialize(&new_state).unwrap()
                    },
                    "dn" => {
                        let new_state = State {
                            settings: SettingsState { i: decoded_state.settings.i - 1, ..decoded_state.settings},
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

        let set_d: Reducer = |state, action| {
            let decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                match &values[0][..] {
                    "up" => {
                        let new_state = State {
                            settings: SettingsState { d: decoded_state.settings.d + 1, ..decoded_state.settings},
                            ..decoded_state
                        };
                        bincode::serialize(&new_state).unwrap()
                    },
                    "dn" => {
                        let new_state = State {
                            settings: SettingsState { d: decoded_state.settings.d - 1, ..decoded_state.settings},
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

        // 0 = temp settings
        // 1 = time settings

        let settings_page: Reducer = |state, action| {
            let decoded_state = state_decoder(state);
            if let Some(values) = action.values {
                match &values[0][..] {
                    "right" => {
                        if decoded_state.window.active == 0 {
                            let new_state = State {
                                // increae highest settings view
                                window: WindowState { active: decoded_state.window.active + 1, ..decoded_state.window},
                                ..decoded_state
                            };
                            bincode::serialize(&new_state).unwrap()

                        } else {
                            // on the last page cycle back around
                            let new_state = State {
                                // increae highest settings view
                                window: WindowState { active: 0, ..decoded_state.window},
                                ..decoded_state
                            };
                            bincode::serialize(&new_state).unwrap()
                        }
                    }
                    "left" => {
                         if decoded_state.window.active == 0 {
                            let new_state = State {
                                // increae highest settings view
                                window: WindowState { active: decoded_state.window.active + 1, ..decoded_state.window},
                                ..decoded_state
                            };
                            bincode::serialize(&new_state).unwrap()

                        } else {
                            // on the last page cycle back around
                            let new_state = State {
                                // increae highest settings view
                                window: WindowState { active: 0, ..decoded_state.window},
                                ..decoded_state
                            };
                            bincode::serialize(&new_state).unwrap()


                        }
                       
                    },
                    _ => state.to_vec()
                }

            } else {
                state.to_vec()
            }

        };


        store.reducers.insert("[time.current_time]", time_updater);
        store.reducers.insert("[schedule.update_thread]", thread_updater);
        store.reducers.insert("[temp.click]", set_target_temp);
        store.reducers.insert("[p.click]", set_p);
        store.reducers.insert("[i.click]", set_i);
        store.reducers.insert("[d.click]", set_d);
        store.reducers.insert("[settings_pager.click]", settings_page);

}
