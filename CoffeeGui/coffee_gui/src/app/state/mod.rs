pub mod mutators;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use lovett::gui_tk::*;
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
    pub views: HashMap<String, Vec<GuiState>>
}


impl State {
    pub fn new() -> State {
        
        // setup view object handlers 
        let mut views: HashMap<String, Vec<GuiState>> = HashMap::new();
        views.insert("bar".to_string(), vec![]);
        views.insert("boiler".to_string(), vec![]);
        views.insert("steamer".to_string(), vec![]);
        views.insert("settings".to_string(), vec![]);
        
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
                p: 0,
                i: 0,
                d: 0
            },
            views,
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
pub struct SettingsState {
    pub running: bool,
    pub p: u32,
    pub i: u32,
    pub d: u32 
}

