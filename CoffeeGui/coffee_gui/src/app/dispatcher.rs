use lovett::dispatcher::{Dispatch};
use lovett::gui_tk::Event;
use lovett::store::Action;
use log::*;
pub struct DispatchHandler {
    


}

impl Dispatch for DispatchHandler {
    fn handle_event(&self,  event: Event) -> Option<Vec<Action>> {
        match event.name {
            "[hw.update_current_time]" => {
                Some(vec![Action::new("[time.current_time]", event.values)])
            },
            
            "[schedule.update_thread]" => {
                debug!("signal to schdule");
                Some(vec![Action::new("[schedule.update_thread]", event.values)])
            },

            "[boiler.preheat]" => {
                // send mutation to set boiler state to on
                // send mutation to display boiler view
                None
            },

            "[temp.click]" => {
                Some(vec![Action::new("[temp.click]", event.values)])
            },

            "[p.click]" => {
                 Some(vec![Action::new("[p.click]", event.values)])
            }

            "[i.click]" => {
                 Some(vec![Action::new("[i.click]", event.values)])
            }

            "[d.click]" => {
                 Some(vec![Action::new("[d.click]", event.values)])
            }


            "[set_p_down]" => {
                None
            }
            _ => None
        }
    }
}
