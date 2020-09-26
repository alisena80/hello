// Crates
extern crate lovett;
extern crate serde;
extern crate bincode;
extern crate env_logger;
mod app;

use app::*;

fn main()  {
    env_logger::init(); 
    let app = App::new();
    run_app(app);
}
