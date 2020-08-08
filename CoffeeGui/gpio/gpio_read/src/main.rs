use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
//use rppal::system::DeviceInfo;

mod joy_pad;
use joy_pad::Action;
use joy_pad::Pad;
use joy_pad::ButtonAction;
use joy_pad::ButtonInitializer;


fn main()  -> Result<(), Box<dyn Error>> { 
  let button_initializers = vec![
     ButtonInitializer {pin: 5, code: 0},
     ButtonInitializer {pin: 6, code: 1},
     ButtonInitializer {pin: 27, code: 2},
     ButtonInitializer {pin: 23, code: 3},
     ButtonInitializer {pin: 17, code: 4},
     ButtonInitializer {pin: 22, code: 5},
     ButtonInitializer {pin: 4, code:  6},
  
  ];

  let mut pad =  Pad::new(button_initializers)?;
  //create channesl for threads to send data to central loop
  let (input_tx, input_rx) = mpsc::channel();
  thread::spawn(move || {
            loop {
                let button_actions = pad.detect_changes();
                input_tx.send(button_actions).unwrap();
                thread::sleep(Duration::from_millis(20));
            }

  });

  loop {
    match input_rx.try_recv() {
        Ok(button_actions) => { 
           process_ba(button_actions);
        },
        Err(_) => ()
    }
    thread::sleep(Duration::from_millis(5));

  };
}

fn code_to_key<'l>(code: usize) -> &'l str{
    let keys = ["b", "a", "l", "r", "up", "dn", "hat"];
    return keys[code];
}
fn process_ba(button_actions: Vec<ButtonAction>){
    for ba in button_actions{
        print_ba(&ba.action, ba.code);
    }
}

fn print_ba(action: &Option<Action>, code: u8){
    match action {
        Some(Action::Pressed) => println!("{} was pressed", code_to_key(usize::from(code))),
        Some(Action::Released) => println!("{} was released", code_to_key(usize::from(code))),
        _ => ()
    }
}
