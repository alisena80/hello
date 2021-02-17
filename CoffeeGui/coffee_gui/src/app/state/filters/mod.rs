use lovett::state::{StateSenderFilter, eq_gui_states};
use super::{ state_decoder };
use super::State;
pub const BAR_VIEW_FILTER: StateSenderFilter = |old_state, new_state| {
    let (old_state_decoded, new_state_decoded) = decode_states(old_state, new_state);
    old_state_decoded.time.current_time != new_state_decoded.time.current_time || 
       ! eq_gui_states(old_state_decoded.views.get("bar").unwrap(), new_state_decoded.views.get("bar").unwrap())
};
pub const SETTINGS_VIEW_FILTER: StateSenderFilter = |old_state, new_state| {
    let (old_state_decoded, new_state_decoded) = decode_states(old_state, new_state);
       ! eq_gui_states(old_state_decoded.views.get("settings").unwrap(), new_state_decoded.views.get("settings").unwrap()) 
            || old_state_decoded.settings.running != new_state_decoded.settings.running 
            || old_state_decoded.settings.p != new_state_decoded.settings.p
            || old_state_decoded.settings.i != new_state_decoded.settings.i
            || old_state_decoded.settings.d != new_state_decoded.settings.d
            || old_state_decoded.time.current_time != new_state_decoded.time.current_time 

};


fn decode_states(old_state: &[u8], new_state: &[u8]) -> (State, State) {
    (state_decoder(old_state), state_decoder(new_state))
}


