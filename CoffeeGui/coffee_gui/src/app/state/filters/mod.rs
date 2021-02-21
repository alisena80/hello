use lovett::state::{StateSenderFilter};
use super::{ state_decoder };
use super::State;
pub const BAR_VIEW_FILTER: StateSenderFilter = |old_state, new_state| {
    let (old_state_decoded, new_state_decoded) = decode_states(old_state, new_state);
    old_state_decoded.time.current_time != new_state_decoded.time.current_time 
};
pub const SETTINGS_VIEW_FILTER: StateSenderFilter = |old_state, new_state| {
    let (old_state_decoded, new_state_decoded) = decode_states(old_state, new_state);
            old_state_decoded.settings.running != new_state_decoded.settings.running 
            || old_state_decoded.settings.p != new_state_decoded.settings.p
            || old_state_decoded.settings.i != new_state_decoded.settings.i
            || old_state_decoded.settings.d != new_state_decoded.settings.d
            || old_state_decoded.time.current_time != new_state_decoded.time.current_time 

};


fn decode_states(old_state: &[u8], new_state: &[u8]) -> (State, State) {
    (state_decoder(old_state), state_decoder(new_state))
}


