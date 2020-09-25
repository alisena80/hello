// Crates
extern crate lovett;
extern crate serde;
extern crate bincode;

mod app;

use app::*;

fn main()  { 
    let app = App::new();
    run_app(app);
}
