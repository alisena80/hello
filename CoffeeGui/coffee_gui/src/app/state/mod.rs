pub mod reducers;
pub mod filters;

use serde::{Serialize, Deserialize};

use std::time::{SystemTime};


pub fn state_decoder(state: &[u8]) -> State{
    bincode::deserialize(state).unwrap()
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub boiler: BoilerState,
    pub tank: TankState,
    pub time: TimeState,
    pub settings: SettingsState,
    pub schedule: Schedule,
}

impl State {
    pub fn new() -> State {
        
        // setup view object handlers 
        // handles the gui element states
        // ie selected, clicked, released
        
        let state = State {
            boiler: BoilerState {
                element_on: false,
                temperature: 0
            },
            tank: TankState {
                level: 0
            },
            time: TimeState {
                turned_on: SystemTime::now(),
                current_time: "00:00:00 XX".to_string()
            },
            settings: SettingsState {
                running: false,
                target_temp: 200,
                p: 0,
                i: 0,
                d: 0
            },
            schedule: Schedule {
                time: ModelState::Empty, //we want this to start running
                boiler: ModelState::Empty,
                tank: ModelState::Empty,
            }
        };
        state
    }


}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoilerState {
    pub element_on: bool,
    pub temperature: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TankState {
    pub level: i32
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeState {
    pub turned_on: SystemTime,
    pub current_time: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub time: ModelState,
    pub boiler: ModelState,
    pub tank: ModelState
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
            && self.boiler == other.boiler
            && self.tank == other.tank
    }
}
impl Eq for Schedule {}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelState {
    Empty,
    Running,
    Ended
}

impl PartialEq for ModelState {
    fn eq(&self, other: &Self) -> bool {
        if let ModelState::Empty = self {
            match other {
                ModelState::Empty => true,
                _ => false
            }
        } else  if let ModelState::Running = self {
            match other {
                ModelState::Running => true,
                _ => false
            }
        } else  if let ModelState::Ended = self {
            match other {
                ModelState::Ended => true,
                _ => false
            }
        } else {
            false
        }

    }
}

impl Eq for ModelState {}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SettingsState {
    pub running: bool,
    pub target_temp: u32,
    pub p: u32,
    pub i: u32,
    pub d: u32 
}



