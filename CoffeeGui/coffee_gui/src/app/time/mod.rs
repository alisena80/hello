use chrono::DateTime;
use chrono::Local;
use std::sync::mpsc::{Sender};
use std::thread;
use log::*;
use lovett::state::Mutation;
use std::time::{Duration};
fn get_current_time() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%r").to_string()
}

pub fn time_keeper(mutation_sender: Sender<Mutation>) {
    thread::spawn( move|| {
        loop {
            mutation_sender.send(
                Mutation{
                    name: "[time.current_time]",
                    value: get_current_time(),
                    number: 0
                }
            ).unwrap();
            debug!("Clock Tick");
            thread::sleep(Duration::from_millis(1000));        
        };

    });
}
