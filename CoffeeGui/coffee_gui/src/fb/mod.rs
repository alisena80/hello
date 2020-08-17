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
        FB {
            fb: framebuffer,
            w: w,
            h: h,
            ll: line_length,
            bpp: bytespp,
            frame: frame,
            background: Color::new(0,0,0)
        } 
    }


    pub fn flush(&mut self){
         let _ = self.fb.write_frame(&self.frame);
    }
    
    pub fn find_point(&self, x1: u32, y1: u32) -> usize{
         ((y1 * self.ll) + (x1 * self.bpp)) as usize
    }

    #[allow(dead_code)]
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

    pub fn draw_filled_rect(&mut self, x1: u32, y1: u32, width: u32, height: u32, color: &Color) {
        for i in 0..(height as u32) {
            self.draw_h_line(x1, y1 + i, width, color);
        }
    }

    pub fn draw_h_line(&mut self, x1: u32, y1: u32, width: u32, color: &Color){
        let x = self.check_x(x1);
        let y = self.check_y(y1);
        let w = self.check_w(x, width);
        let index = self.find_point(x, y);
        let color = color.to_16b();
        for i in 0..(w as usize) {
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
        for i in 0..(h  as usize) {
            self.frame[index + (480 * i)] = color as u8;
            self.frame[index + (480 * i) + 1] = (color >> 8) as u8;
        }
    }
                  
    pub fn render_image(&mut self, img: &bmp::Image, x1: u32, y1: u32, w1:u32, h1:u32, img_x:u32, img_y:u32){
        let start_x = self.check_x(x1);
        let start_y = self.check_y(y1);
        let w = self.check_h(start_x, w1);
        let h = self.check_h(start_y, h1);
        
        for x in start_x..(w - 1) {
            for y in start_y..(h -1) {
                let px = img.get_pixel(img_x + x, img_y + y);
                let index = self.find_point(x,y);
                let color = Color::new(px.r, px.g, px.b);
                let rgb565 = color.to_16b();
                self.frame[index] = rgb565 as u8;
                self.frame[index + 1] = (rgb565 >> 8) as u8;
            }
        }
    }
}
