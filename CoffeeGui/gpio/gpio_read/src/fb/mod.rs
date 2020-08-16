use bmp::Image;
use framebuffer::{Framebuffer};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8

}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b}
    }

    pub fn to_16b(&self) -> u16 {
        let r: u8 = ((31.0 * self.r as f32) / 255 as f32) as u8;
        let g: u8 = ((63.0 * self.g as f32) / 255 as f32) as u8;
        let b: u8 = ((31.0 * self.b as f32) / 255 as f32) as u8; 
        let rgb565: u16 = ((r as u16) << 11) + ((g as u16) << 5) + b as u16;
        rgb565
    }
}

pub struct FB {
    fb: Framebuffer,
    w: u32,
    h: u32,
    ll: u32,
    bpp: u32,
    frame: Vec<u8>,
    img: Image,
    offset_x: u32,
    offset_y: u32,
    color: Color,
    background: Color
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
            color: Color::new(255,255,255),
            background: Color::new(0,0,0)
        } 
    }

    pub fn pan_image(&mut self, x: i32, y: i32){
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


    pub fn flush(&mut self){
         let _ = self.fb.write_frame(&self.frame);
    }
    
    pub fn find_point(&self, x1: u32, y1: u32) -> usize{
         ((y1 * self.ll) + (x1 * self.bpp)) as usize
    }


    pub fn set_color(&mut self, color: Color){
        self.color = color
    }

    pub fn set_background(&mut self, color: Color){
        self.background = color
    }

    fn check_x(&self, x: u32) -> u32 {
        if x >= self.w {
            return self.w - 1
        }
        return x
    }
    fn check_y(&self, y: u32) -> u32 {
        if y >= self.h {
            return  self.h - 1
        }
        return y
    }

    fn check_w(&self, x: u32, w: u32) -> u32 {
        if x + w > self.w {
            return self.w - x
        }
        return w
    }
    
    fn check_h(&self, y: u32, h: u32) -> u32 {
        if y + h > self.h {
            return self.h - y
        }
        return h
    }

    pub fn clear(&mut self) {
        self.draw_filled_rect(0, 0, 240, 240, &Color::new(self.background.r, self.background.g, self.background.b));
    }

    pub fn draw_rect(&mut self, x1: u32, y1: u32, width: u32,  height: u32, color: &Color) {
        self.draw_h_line(x1, y1, width, color);
        self.draw_h_line(x1, y1 + height - 1, width, color);
        self.draw_v_line(x1, y1, height, color);
        self.draw_v_line(x1 + width - 1, y1, height, color);
    }

    pub fn draw_filled_rect(&mut self, x1: u32, y1: u32, mut width: u32, mut height: u32, color: &Color) {
        for i in 0..(height - 1 as u32) {
            self.draw_h_line(x1, y1 + i, width, color);
        }
    }

    pub fn draw_h_line(&mut self, x1: u32, y1: u32, width: u32, color: &Color){
        let x = self.check_x(x1);
        let y = self.check_y(y1);
        let w = self.check_w(x, width);
        let index = self.find_point(x, y);
        let color = color.to_16b();
        for i in 0..((w - 1) as usize) {
            self.frame[index + (2 * i)] = color as u8;
            self.frame[index + (2 * i) + 1] = (color >> 8) as u8;
        }
    }

    pub fn draw_v_line(&mut self, x1: u32, y1: u32, height: u32, color: &Color) {
        let x = self.check_x(x1);
        let y = self.check_y(y1);
        let h = self.check_h(y, height);
        let index = self.find_point(x, y);
        let color = color.to_16b();
        for i in 0..((h - 1) as usize) {
            self.frame[index + (480 * i)] = color as u8;
            self.frame[index + (480 * i) + 1] = (color >> 8) as u8;
        }
    }
                  

}
