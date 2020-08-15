use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod joy_pad;
use joy_pad::Pad;
use joy_pad::ButtonInitializer;
use joy_pad::Action;

mod fb;
use fb::FB;

extern crate bmp;
extern crate framebuffer;





fn main()  -> Result<(), Box<dyn Error>> { 
  let button_initializers = vec![
     ButtonInitializer {pin: 5, code: 0, key: "b"},
     ButtonInitializer {pin: 6, code: 1, key: "a"},
     ButtonInitializer {pin: 27, code: 2, key: "l"},
     ButtonInitializer {pin: 23, code: 3, key: "r"},
     ButtonInitializer {pin: 17, code: 4, key: "up"},
     ButtonInitializer {pin: 22, code: 5, key: "dn"},
     ButtonInitializer {pin: 4, code:  6, key: "hat"},
  
  ];

  let mut pad =  Pad::new(&button_initializers)?;
  //create channesl for threads to send data to central loop
  let (input_tx, input_rx) = mpsc::channel();
  thread::spawn(move || {
            loop {
                let button_actions = pad.detect_changes();
                input_tx.send(button_actions).unwrap();
                thread::sleep(Duration::from_millis(20));
            }

  });
  let mut fb = FB::new("/dev/fb1");
  fb.set_color(fb::Color::new(255,0,0));
  fb.draw_rect(1, 1, 90, 90);
  fb.flush();

  loop {
    match input_rx.try_recv() {
        Ok(button_actions) => {
            for ba in &button_actions { 
                match ba.action {
                    Action::Pressed => {
                        match ba.code {
                            2 => {
                                fb.draw_rect(90, 90, 90, 90);
                                fb.flush();
                            },
                            _ => ()
                        }
                    },
                    Action::Released => ()
                }
            }
            joy_pad::helpers::ba_to_console(button_actions, &button_initializers);
        },
        Err(_) => ()
    }
    thread::sleep(Duration::from_millis(5));

  };

}


