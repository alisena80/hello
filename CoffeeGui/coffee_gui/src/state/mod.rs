use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

use std::time::{Instant, SystemTime, Duration};

use chrono::format::strftime;
use chrono::DateTime;
use chrono::Local;

pub struct RootState {
    state: State,
    state_senders: Vec<Sender<State>>,
    mutation_receiver: Receiver<Mutator>,
    mutation_sender: Sender<Mutator>
   
}

#[derive(Clone, Debug)]
pub struct State {
    pub boiler: BoilerState,
    pub tank: TankState,
    pub time: TimeState,
    pub settings: SettingsState,
}

#[derive(Clone, Debug)]
pub struct BoilerState {
    pub element_on: bool,
    pub temperature: i32
}

#[derive(Clone, Debug)]
pub struct TankState {
    pub level: i32
}

#[derive(Clone, Debug)]
pub struct TimeState {
    pub turned_on:  Instant,
    pub current_time: &'static str
}

#[derive(Clone, Debug)]
pub struct SettingsState {
    pub running: bool,
    pub p: u32,
    pub i: u32,
    pub d: u32 
}

impl RootState {
    pub fn new() -> RootState {
        let (sender, receiver) = channel();
        RootState {
            state: State {
                boiler: BoilerState {
                    element_on: false,
                    temperature: 0
                },
                tank: TankState {
                    level: 0                    
                },
                time: TimeState {
                    turned_on: Instant::now(),
                    current_time: "00:00" 
                },
                settings: SettingsState {
                    running: false,
                    p: 0,
                    i: 0,
                    d: 0
                },
            },
            state_senders: vec![],
            mutation_receiver: receiver,
            mutation_sender: sender       
            
        }
    }

    pub fn regStateSender(&mut self, sender: Sender<State>) { 
        self.state_senders.push(sender);
    }


    pub fn getMutationSender(&self) -> Sender<Mutator> {
        self.mutation_sender.clone()
    }


    pub fn runState(&'static mut self) {
        thread::spawn(move || {
            loop {

                // lisen for mutators
                match self.mutation_receiver.try_recv() {
                    Ok(mutator) => {
                        // process mutation
                        self.mutate(mutator);

                        // send state clone
                        for sender in &self.state_senders {
                            sender.send(self.state.clone());
                        }


                    },
                    Err(_) => ()

                }
    
            }
        });

    }

    pub fn mutate(&mut self, mutator: Mutator) {
        match mutator.name {
            "[time.current_time]" => {
                self.state.time.current_time = mutator.value;
            },
            _ => ()
        }        
    }
}

pub struct Mutator {
    name: &'static str,
    value: &'static str
}

fn get_current_time() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%H:%M").to_string()
}


