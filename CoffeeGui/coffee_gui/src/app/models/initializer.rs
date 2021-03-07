use lovett::model_scheduler::{ModelScheduler,  Schedule, Model};
use super::time::TimeModel;
use std::sync::mpsc::{Sender, channel};
use lovett::gui_tk::Event;
use lovett::store::{Store, StateSenderFilter};
use super::super::{ state_decoder };
use std::thread::JoinHandle;
use std::thread;
use super::super::state::ModelState;

pub fn setup(event_tx: Sender<Event>, store: &mut Store) -> ModelScheduler{

        // Initialize the ModelScheduler

        let schedule: Schedule = |state, models, threads| {
            let state = state_decoder(state); 
            let mut time_thread = threads.remove("time").unwrap();
            if should_start_thread(state.schedule.time, &mut time_thread) {
                let time_model = models.remove("time").unwrap();
                time_thread =  Some(start_thread(time_model));
                threads.insert("time", time_thread);
            }

        };

        let (state_tx, state_rx) = channel();
        let state_sender_filter: StateSenderFilter = |vec1, vec2| -> bool {
            let old_state = state_decoder(vec1);
            let new_state = state_decoder(vec2);
            old_state.schedule != new_state.schedule
        };

        store.reg_state_sender(state_tx, state_sender_filter);
        let mut model_scheduler = ModelScheduler::new(state_rx, schedule);


        // setup models
        let time_event_sender = event_tx.clone();
        let time_model = TimeModel::new(time_event_sender);        

        model_scheduler.register_model("time", Box::new(time_model));
        model_scheduler
}

fn should_start_thread(model_state: ModelState, thread_handle: &mut Option<JoinHandle<()>>) -> bool {
    match model_state {
        ModelState::Empty => (
            // the thread should not be doing anything
            // the model exists but the thread has not been created
            // the schedule has nothing to do if JoinHandle is None
            if let None = thread_handle {
                // all good
                false
            } else {
                // bad
                panic!("Thread is Started, but should not be");
            }
        ), 
        ModelState::Running => (
            // the thread should be in a running state
            // if JoinHandle is JoinHandle then do nothing, if None, start it up and save the new handle
            if let None = thread_handle {
                true
            } else {
                false
            } 

        ),
        ModelState::Ended => (
            // the thread should be done and dropped
            // currenlty no mechanism to do this
            false
        )
    }
}


fn start_thread(mut model: Box<dyn Model + Send>) -> JoinHandle<()> {
    thread::spawn(move || {
        model.handler();
    })    
}
