// Crates
extern crate framebuffer;
extern crate image;
extern crate rusttype;

// Modules
mod joy_pad;
mod fb;
mod canvas;
mod views;
mod gui_tk;
mod controller;
mod state;
mod app;

// for building and running the app struct
use app::*;

fn main()  { 
    let app = App::new();
    run_app(app);
}
