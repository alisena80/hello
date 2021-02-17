use lovett::joy_pad::*;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;

pub fn setup() -> (Receiver<Vec<HIDEvent>>, Pad) {
    // setup hw buttons
    let button_initializers = vec![
        ButtonInitializer {pin: 5, code: 0, key: "b"},
        ButtonInitializer {pin: 6, code: 1, key: "a"},
        ButtonInitializer {pin: 27, code: 2, key: "l"},
        ButtonInitializer {pin: 23, code: 3, key: "r"},
        ButtonInitializer {pin: 17, code: 4, key: "up"},
        ButtonInitializer {pin: 22, code: 5, key: "dn"},
        ButtonInitializer {pin: 4, code:  6, key: "hat"},
    ];

    //create channesl for sending raw input buttons to the root_view
    let (input_tx, joy_pad_input_rx) = mpsc::channel();

    // setup the input_pad
    let pad =  match Pad::new(&button_initializers, input_tx) {
        Ok(pad) => pad,
        Err(x) => panic!("Error Starting Input Pad: {}", x)
    };


    
    (joy_pad_input_rx, pad)
}
