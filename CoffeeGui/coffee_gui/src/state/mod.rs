use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::thread::JoinHandle;

use std::time::{Instant, SystemTime, Duration};

use chrono::format::strftime;
use chrono::DateTime;
use chrono::Local;

use super::gui_tk::GuiState;

pub fn runState(mut root_state:  RootState) {
        thread::spawn(move || {
            loop {

                // lisen for mutators
                let mutator: Mutator = match root_state.mutation_receiver.try_recv() {
                    Ok(mutator) => mutator,
                    Err(_) => Mutator {name: "", value: "".to_string()}

                };
                         // process mutation
                        root_state.mutate(mutator);
                        println!("Got Mutator");
                        // send state clone
                        for sender in &root_state.state_senders {
                            sender.send(root_state.state.clone()).unwrap();
                        }

   
            }
        });

}



pub struct RootState {
    pub state: State,
    pub state_senders: Vec<Sender<State>>,
    pub mutation_receiver: Receiver<Mutator>,
    mutation_sender: Sender<Mutator>   
}

#[derive(Clone, Debug)]
pub struct State {
    pub boiler: BoilerState,
    pub tank: TankState,
    pub time: TimeState,
    pub settings: SettingsState,
    pub views: ViewsState
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
    pub current_time: String
}

#[derive(Clone, Debug)]
pub struct SettingsState {
    pub running: bool,
    pub p: u32,
    pub i: u32,
    pub d: u32 
}
#[derive(Clone, Debug)]
pub struct ViewsState {
    pub bar: Vec<GuiState>,
    pub boiler: Vec<GuiState>,
    pub steamer: Vec<GuiState>,
    pub settings: Vec<GuiState>
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
                    current_time: "00:00".to_string() 
                },
                settings: SettingsState {
                    running: false,
                    p: 0,
                    i: 0,
                    d: 0
                },
                views: ViewsState{
                    bar: vec![],
                    boiler: vec![],
                    steamer: vec![],
                    settings: vec![]
                }
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


    pub fn mutate(&mut self, mutator: Mutator) {
        match mutator.name {
            "[time.current_time]" => {
                println!("Matched Mutator");
                self.state.time.current_time = mutator.value;
            },
            _ => ()
        }        
    }
}

pub struct Mutator {
    name: &'static str,
    value: String
}

fn get_current_time() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%H:%M").to_string()
}

pub fn time_keeper(mutation_sender: Sender<Mutator>) {
    let join_handle = thread::spawn( move|| {
        loop {
            mutation_sender.send(
                Mutator{
                    name: "[time.current_time]",
                    value: get_current_time()
                }
            ).unwrap();
            println!("Tick");
            thread::sleep(Duration::from_millis(1000));        
        };

    });
}
