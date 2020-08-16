use super::fb::FB;
use super::fb::Color;


pub struct Layer<T> {
    pub item: Box<T>,
    pub active: bool
}


impl<T> Layer<T> {
    pub fn new(item: T, active: bool) -> Layer<T>{
        Layer{item: Box::new(item), active}
    }
}

pub struct Canvas {
    screen: FB,
    pub layers: Vec<Layer<Box<dyn Draw>>>
}

impl Canvas {
    pub fn new(dev: &'static str) -> Canvas{
        let mut fb = FB::new(dev);
        let mut layers: Vec<Layer<Box<dyn Draw>>> = vec![];
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
}


pub trait Draw {
    fn draw(&self, fb: &mut FB);

}

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
}

