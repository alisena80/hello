use std::error::Error;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

mod joy_pad;
use joy_pad::Pad;
use joy_pad::ButtonInitializer;

extern crate bmp;
extern crate framebuffer;

use bmp::BmpResult;
use bmp::Image;

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

  loop {
    match input_rx.try_recv() {
        Ok(button_actions) => { 
           joy_pad::helpers::ba_to_console(button_actions, &button_initializers);
        },
        Err(_) => ()
    }
    thread::sleep(Duration::from_millis(5));

  };



}

struct FB {
    fb: Framebuffer,
    w: usize,
    h: usize,
    ll: usize,
    bpp: usize,
    frame: Vec<u8>,
    img: BmpResult<Image>
    
}

impl FB {
    pub fn new(dev: &str) -> FB {
    
        let mut framebuffer = Framebuffer::new(dev).unwrap();

        let w = framebuffer.var_screen_info.xres;
        let h = framebuffer.var_screen_info.yres;
        let line_length = framebuffer.fix_screen_info.line_length;
        let bytespp = framebuffer.var_screen_info.bits_per_pixel / 8;

        let mut frame = vec![0u8; (line_length * h) as usize];
        let img = bmp::open("examples/rust-logo/rust-logo.bmp").unwrap();
        FB {
            fb: framebuffer,
            w: w,
            h: h,
            ll: line_length,
            bpp: bytespp,
            frame: frame,
            img: img,
            offset_x: usize,
            offset_y: usize,
        } 
    }

    pub fn pan(&mut self, x: i32, y: i32){
         
    }

    pub fn draw(&mut self){
       
        for (x, y) in img.coordinates() {
            if x < 240 && y < 240 {
                let px = img.get_pixel(x, y);
                let start_index = ((y * line_length) + (x * bytespp)) as usize;
                let r: u8 = ((31.0 * px.r as f32) / 255 as f32) as u8;
                let g: u8 = ((63.0 * px.g as f32) / 255 as f32) as u8;
                let b: u8 = ((31.0 * px.b as f32) / 255 as f32) as u8; 
                let rgb565: u16 = ((r as u16) << 11) + ((g as u16) << 5) + b as u16;
                frame[start_index] = rgb565 as u8;
                frame[start_index + 1] = (rgb565 >> 8) as u8;
            }
        }

        let _ = framebuffer.write_frame(&frame);

    
    }

}
