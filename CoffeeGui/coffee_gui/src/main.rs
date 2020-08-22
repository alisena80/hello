use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod joy_pad;
use joy_pad::Pad;
use joy_pad::ButtonInitializer;
use joy_pad::Action;


mod fb;
mod canvas;
use canvas::Canvas;
use canvas::Layer;
use canvas::Rect;
use fb::Color;

use rand::Rng;

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
  let mut canvas = Canvas::new("/dev/fb1");
  canvas.clear();
  // random other things
  canvas.layers.push(Layer::new(Box::new(Rect::new(89, 150, 30, 10, true,Color::new(255,255,25)),), true, "float"   ));
  canvas.layers.push(Layer::new(Box::new(Rect::new(90, 200, 10, 40, true,Color::new(255,25,255)),), true, "float"   ));
  canvas.layers.push(Layer::new(Box::new(Rect::new(9, 150, 60, 4, true,Color::new(25,255,255)),), true, "float"   ));

  canvas.layers.push(
    Layer::new(
        Box::new(
            Rect::new(
                0,0,10,10, true, Color::new(255,255,0)
            ),
        ),
        true,
        "box"
    )
);
  canvas.layers.push(
    Layer::new(
        Box::new(
            Rect::new(
                20,20,10,10, false, Color::new(255,0,0)
            ),
        ),
        true,
        "box"
    )

  );


  loop {
    match input_rx.try_recv() {
        Ok(button_actions) => {
            for ba in &button_actions { 
                match ba.action {
                    Action::Pressed => {
                        match ba.code {
                            2 => canvas.slide_layer_group("box", -1, 0),
                            3 => canvas.slide_layer_group("box", 1, 0),
                            4 => canvas.slide_layer_group("box", 0, -1),
                            5 => canvas.slide_layer_group("box", 0, 1),
                            _ => ()
                        };
                        
                    },
                    Action::Repeated => {
                        match ba.code {
                            2 => canvas.slide_layer_group("box", -10, 0),
                            3 => canvas.slide_layer_group("box", 10, 0),
                            4 => canvas.slide_layer_group("box", 0, -10),
                            5 => canvas.slide_layer_group("box", 0, 10),
                            _ => ()
                        };


                    
                    },
                    Action::Released => {
                    }
                }
            }
            joy_pad::helpers::ba_to_console(button_actions, &button_initializers);
        },
        Err(_) => ()
    }
    let mut rng = rand::thread_rng();
    let float_x: i32  = rng.gen_range(-300, 340);
    let float_y: i32  = rng.gen_range(-300, 300);
    canvas.slide_layer_group("float", float_x, float_y);
    canvas.render();
    thread::sleep(Duration::from_millis(5));

  };

}



