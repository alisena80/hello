#[allow(dead_code)]
use super::fb::FB;
use super::fb::Color;


// Layer
pub struct Layer<T> {
    pub item: Box<T>,
    pub active: bool,
    pub group: &'static str
}


impl<T> Layer<T> {
    pub fn new(item: T, active: bool, group: &'static str) -> Layer<T>{
        Layer{item: Box::new(item), active, group}
    }
}

// Canvas
pub struct Canvas {
    screen: FB,
    pub layers: Vec<Layer<Box<dyn Draw>>>
}

impl Canvas {
    pub fn new(dev: &'static str) -> Canvas{
        let fb = FB::new(dev);
        let layers: Vec<Layer<Box<dyn Draw>>> = vec![];
        Canvas {
            screen: fb,
            layers
            
        }
    }

    pub fn render(&mut self) {
        self.screen.clear();
        for layer in &self.layers {
            layer.item.draw(&mut self.screen);
        }
        self.screen.flush();

    }

    pub fn clear(&mut self){
        self.screen.clear();
        self.screen.flush();
    }

    pub fn slide_layer_group(&mut self, group: &'static str, x: i32, y: i32) {
        for layer in &mut self.layers {
            if layer.group == group {
                layer.item.slide(x, y);
            }
        }

    }


}


pub trait Draw {
    fn draw(&self, fb: &mut FB);
    fn slide(&mut self, x: i32, y: i32);
}


// Rectangles
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub filled: bool,
    pub color: Color
}


impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32, filled: bool, color: Color) -> Rect {
        Rect {x ,y, w, h, filled, color}
    }

    fn clipped(&self, fb: &mut FB) -> Option<(u32, u32, u32, u32)>{
        let x: u32;
        let y: u32;
        let mut w: u32;
        let mut h: u32;

        // determine how much if any of the object is on screen
        // x negative direction

        if self.x < 0 {
            x = 0;
            w = (self.w + self.x) as u32;
        } else if self.x >= fb.w as i32 {  // x positive direction
            return None
        } else {
            x = self.x as u32;
            w = self.w as u32;            
        }
        // width greater than display
        if x + w > fb.w {
            w = fb.w - x;
        } 

        // y negative direction
        if self.y < 0 {
            y = 0;
            h = (self.h + self.y) as u32;
        } else if self.y >= fb.h as i32 {  // y positive direction
            return None
        } else {
            y = self.y as u32;
            h = self.h as u32;            
        }
        // width greater than display
        if y + h > fb.h {
            h = fb.h - y;
        } 
        Some((x, y, w, h))
    }
}

impl Draw for Rect {
    fn draw(&self, fb: &mut FB){
        //clip actual coordinates to render what is on screen or do nothing
        match self.clipped(fb) {
            Some((x, y, w, h)) => {
                if self.filled {
                    fb.draw_filled_rect(x, y, w,h, &self.color );
                } else {
                    fb.draw_rect(x, y, w, h, &self.color);
                }
            },
            None => ()
        }

    }
    fn slide(&mut self, x: i32, y: i32) {
        //move x
        self.x = self.x + x;

        //move y
        self.y = self.y + y;
    }
       
}


// images
pub struct Image {
    img: bmp::Image,
    // where it goes on the canvas
    x: i32,
    y: i32,
    w: i32,
    h: i32, 
    // where we sample from the image
    img_x: u32,
    img_y: u32
}

impl Image {
    #[allow(dead_code)] 
    pub fn new(path: &'static str, x: i32, y: i32, w: i32, h: i32, img_x: u32, img_y: u32 ) -> Image {
        let img = bmp::open(path).unwrap();
        Image {
            img, x, y, w, h, img_x, img_y
        }
    }
    fn clipped(&self, fb: &mut FB) -> Option<(u32, u32, u32, u32)>{
        let x: u32;
        let y: u32;
        let mut w: u32;
        let mut h: u32;

        // determine how much if any of the object is on screen
        // x negative direction

        if self.x < 0 {
            x = 0;
            w = (self.w + self.x) as u32;
        } else if self.x >= fb.w as i32 {  // x positive direction
            return None
        } else {
            x = self.x as u32;
            w = self.w as u32;            
        }
        // width greater than display
        if x + w > fb.w {
            w = fb.w - x;
        } 

        // y negative direction
        if self.y < 0 {
            y = 0;
            h = (self.h + self.y) as u32;
        } else if self.y >= fb.h as i32 {  // y positive direction
            return None
        } else {
            y = self.y as u32;
            h = self.h as u32;            
        }
        // width greater than display
        if y + h > fb.h {
            h = fb.h - y;
        } 
        Some((x, y, w, h))
    }

}

impl Draw for Image {
    fn draw(&self, fb: &mut FB){
        match self.clipped(fb) {
            Some((x, y, w, h)) => {
                    fb.render_image(&self.img, x, y, w, h, self.img_x, self.img_y)
                },
            None => ()
        }
    }
    fn slide(&mut self, x: i32, y: i32) {
        //move x
        self.x = self.x + x;

        //move y
        self.y = self.y + y;
    }

}

/* old image code
    pub fn pan_image(&mut self, x: i32, y: i32){
         //move x
        let i32_x_total = self.offset_x as i32 + x;
    
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
        self.draw_image();
        self.flush(); 
    }

    pub fn draw_image(&mut self){
        for (x, y) in self.img.coordinates() {
            if x < 240 && y < 240 {
                let px = self.img.get_pixel(x + self.offset_x, y + self.offset_y);
                let start_index = ((y * self.ll) + (x * self.bpp)) as usize;
                let color = Color::new(px.r, px.g, px.b);
                let rgb565 = color.to_16b();
                self.frame[start_index] = rgb565 as u8;
                self.frame[start_index + 1] = (rgb565 >> 8) as u8;
            }
        }
    }
*/


