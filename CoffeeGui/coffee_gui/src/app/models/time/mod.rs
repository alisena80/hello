use chrono::DateTime;
use chrono::Local;
use std::sync::mpsc::{Sender};
use std::thread;
use log::*;
use std::time::{Duration};
use lovett::gui_tk::Event;
use lovett::model_scheduler::Model;

pub struct TimeModel {
    pub event_tx: Sender<Event>
}


impl TimeModel {
    pub fn new(event_tx: Sender<Event>) -> TimeModel {
        TimeModel {
            event_tx
        }
    }

    fn get_current_time(&self ) -> String {
        let local: DateTime<Local> = Local::now();
        local.format("%r").to_string()
    }


}

impl Model for TimeModel {
    fn handler(&mut self) {
         loop {
            self.event_tx.send(
                Event::new(
                    "[hw.update_current_time]",
                    Some(vec![self.get_current_time()]),
                )
            ).unwrap();
            debug!("Clock Tick");
            thread::sleep(Duration::from_millis(1000));        
        }
    }
}


