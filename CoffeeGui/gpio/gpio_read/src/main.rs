use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod joy_pad;
use joy_pad::Pad;
use joy_pad::ButtonInitializer;
use joy_pad::Action;

extern crate bmp;
extern crate framebuffer;

use bmp::Image;
use framebuffer::{Framebuffer};

use std::time::{SystemTime};



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
  fb.draw();

  loop {
    match input_rx.try_recv() {
        Ok(button_actions) => {
            for ba in &button_actions { 
                match ba.action {
                    Action::Pressed => {
                        match ba.code {
                            2 => fb.pan(-1, 0),
                            3 => fb.pan(1, 0),
                            4 => fb.pan(0, -1),
                            5 => fb.pan(0, 1),
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

struct FB {
    fb: Framebuffer,
    w: u32,
    h: u32,
    ll: u32,
    bpp: u32,
    frame: Vec<u8>,
    img: Image,
    offset_x: u32,
    offset_y: u32,   
}

impl FB {
    pub fn new(dev: &str) -> FB {
    
        let framebuffer = Framebuffer::new(dev).unwrap();

        let w = framebuffer.var_screen_info.xres;
        let h = framebuffer.var_screen_info.yres;
        let line_length = framebuffer.fix_screen_info.line_length;
    let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

        let frame = vec![0u8; (line_length * h) as usize];
        let img = bmp::open("pic/rust-logo.bmp").unwrap();
        FB {
            fb: framebuffer,
            w: w,
            h: h,
            ll: line_length,
            bpp: bytespp,
            frame: frame,
            img: img,
            offset_x: 0,
            offset_y: 0,
        } 
    }

    pub fn pan(&mut self, x: i32, y: i32){
         //move x
        let i32_x_total = self.offset_x as i32 + x;
        if i32_x_total < 0 {
            self.offset_x = 0;
        } else {
            self.offset_x = i32_x_total as u32;
        }
    
        let i32_y_total = self.offset_y as i32 + y;
        if i32_y_total < 0 {
            self.offset_y = 0;
        } else {    
            self.offset_y = i32_y_total as u32;
        }

        if self.offset_x > self.w {
            self.offset_x = self.w;
        }
        if self.offset_y > self.h {
            self.offset_y = self.h;
        }
        self.draw(); 
    }

    pub fn draw(&mut self){
        let now = SystemTime::now();
        for (x, y) in self.img.coordinates() {
            if x < 240 && y < 240 {
                let px = self.img.get_pixel(x + self.offset_x, y + self.offset_y);
                let start_index = ((y * self.ll) + (x * self.bpp)) as usize;
                let r: u8 = ((31.0 * px.r as f32) / 255 as f32) as u8;
                let g: u8 = ((63.0 * px.g as f32) / 255 as f32) as u8;
                let b: u8 = ((31.0 * px.b as f32) / 255 as f32) as u8; 
                let rgb565: u16 = ((r as u16) << 11) + ((g as u16) << 5) + b as u16;
                self.frame[start_index] = rgb565 as u8;
                self.frame[start_index + 1] = (rgb565 >> 8) as u8;
            }
        }
        println!("t1: {}", now.elapsed().unwrap().as_secs());

        let _ = self.fb.write_frame(&self.frame);
        println!("t2: {}", now.elapsed().unwrap().as_secs());

    
    }

}
