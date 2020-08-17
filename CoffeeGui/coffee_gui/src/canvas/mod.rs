#[allow(dead_code)]
use super::fb::FB;
use super::fb::Color;


// Layer
pub struct Layer<T> {
    pub item: Box<T>,
    pub active: bool
}


impl<T> Layer<T> {
    pub fn new(item: T, active: bool) -> Layer<T>{
        Layer{item: Box::new(item), active}
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
}


pub trait Draw {
    fn draw(&self, fb: &mut FB);
    fn slide(&mut self, x: i32, y: i32);
}


// Rectangles
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub filled: bool,
    pub color: Color
}


impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32, filled: bool, color: Color) -> Rect {
        Rect {x ,y, w, h, filled, color}
    }
}

impl Draw for Rect {
    fn draw(&self, fb: &mut FB){
        if self.filled {
            fb.draw_filled_rect(self.x, self.y, self.w, self.h, &self.color );
        } else {
            fb.draw_rect(self.x, self.y, self.w, self.h, &self.color);
        }
    }
    fn slide(&mut self, x:i32, y: i32) {
        //move x
        let i32_x_total = self.x as i32 + x;
        if i32_x_total < 0 {
            self.x = 0;
        } else {
            self.x = i32_x_total as u32;
        }


        //move y
        let i32_y_total = self.y as i32 + y;
        if i32_y_total < 0 {
            self.y = 0;
        } else {
            self.y = i32_y_total as u32;
        }      
    }
       
}


// images
pub struct Image {
    img: bmp::Image,
    // where it goes on the canvas
    x: u32,
    y: u32,
    w: u32,
    h: u32, 
    // where we sample from the image
    img_x: u32,
    img_y: u32
}

impl Image {
    pub fn new(path: &'static str, x: u32, y: u32, w: u32, h: u32, img_x: u32, img_y: u32 ) -> Image {
        let img = bmp::open(path).unwrap();
        Image {
            img, x, y, w, h, img_x, img_y
        }
    }
}

impl Draw for Image {
    fn draw(&self, fb: &mut FB){
        fb.render_image(&self.img, self.x, self.y, self.w, self.h, self.img_x, self.img_y)
    }
    fn slide(&mut self, x: i32, y: i32) {
        //move x
        let i32_x_total = self.x as i32 + x;
        if i32_x_total < 0 {
            self.x = 0;
        } else {
            self.x = i32_x_total as u32;
        }


        //move y
        let i32_y_total = self.y as i32 + y;
        if i32_y_total < 0 {
            self.y = 0;
        } else {
            self.y = i32_y_total as u32;
        }      
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


