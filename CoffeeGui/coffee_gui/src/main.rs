// Crates
extern crate lovett;
extern crate serde;
extern crate bincode;
extern crate env_logger;

#[macro_use]
extern crate lazy_static;

mod app;

use app::*;

fn main()  {
    env_logger::init(); 
    let app = App::new();
    run_app(app);
}
