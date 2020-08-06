use std::error::Error;
use std::thread;
use std::time::Duration;

mod joy_pad;
use joy_pad::Action;
use joy_pad::Pad;
use joy_pad::Presses;

fn main()  -> Result<(), Box<dyn Error>> { 

  let mut pad: Pad = Pad::new(5, 6, 27, 23, 17, 22, 4)?;

  loop {
    let presses = pad.detect_changes();
    process_presses(&presses);
    thread::sleep(Duration::from_millis(20));
  };
}


fn process_presses(presses: &Presses){
    print_press(&presses.b, "b");
    print_press(&presses.a, "a");
    print_press(&presses.l, "l");
    print_press(&presses.r, "r");
    print_press(&presses.up, "up");
    print_press(&presses.dn, "dn");
    print_press(&presses.hat, "hat");



}

fn print_press(press: &Option<Action>, name: &str){
    match press {
        Some(Action::Pressed) => println!("{} was pressed", name),
        Some(Action::Released) => println!("{} was released", name),
        _ => ()
    }
}
