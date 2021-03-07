use lovett::dispatcher::{Dispatch};
use lovett::gui_tk::Event;
use lovett::store::Action;

pub struct DispatchHandler {
    


}

impl Dispatch for DispatchHandler {
    fn handle_event(&self,  event: Event) -> Option<Action> {
        match event.name {
            "[hw.update_current_time]" => {
                Some(Action::new("[time.current_time]", event.values))
            },
            
            "[schedule.update_thread]" => {
                Some(Action::new("[schedule.update_thread]", event.values))
            },

            "[boiler.preheat]" => {
                // send mutation to set boiler state to on
                // send mutation to display boiler view
                None
            },

            "[set_temp_up]" => {
                None
            },

            "[set_temp_down]" => {
                None

            },

            "[set_p_up]" => {
                None
            }

            "[set_p_down]" => {
                None
            }
            _ => None
        }
    }
}
