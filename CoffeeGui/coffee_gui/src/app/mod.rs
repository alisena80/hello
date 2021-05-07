use lovett::dispatcher::*;
use lovett::window_viewer::*;
use lovett::store::{Store,  run_store};
use lovett::model_scheduler::{run_model_scheduler, ModelScheduler};
use lovett::gui_tk::Event;
mod state;
use state::*;

mod models;
use models::initializer::setup as model_setup;

mod views;

mod dispatcher;
use dispatcher::DispatchHandler;

use lovett::gui_tk::{ GuiConfig, Palette};
use lovett::fb::Color;


lazy_static! {
    pub static ref CONFIG: GuiConfig = GuiConfig::new(
        Palette::new(

            // Base
            Color::new(78, 156, 183),       // outline
            Color::new(255, 255, 255),      // text
            Color::new(15, 25, 25),         // background

            // Selected
            Color::new(78, 156, 183),       // outline
            Color::new(255, 255, 255),      // text
            Color::new(90, 150, 150),       // background

            // Clicked
            Color::new(160, 255, 255),      // outline
            Color::new(0, 0, 0),            // text
            Color::new(150, 250, 250),      // background
        ),
        "./assets/fonts/Nanum_Gothic/NanumGothic-Regular.ttf");
}

pub fn run_app(app: App) {
    
    // Run the Window Viewer for GUI Systems
    run_window_viewer(app.window_viewer);

    // Run the Store to build a the state tree
    run_store(app.store);

    // Create an event sending bus prior to moving the dispatcher object into its thread
    let event_sender = app.dispatcher.get_event_sender();

    // Run the dispatcher to handle events
    run_dispatcher(app.dispatcher);

    // Run the model scheduler to start up all app logic
    let join_handle = run_model_scheduler(app.model_scheduler);

    // kick start additional threads required for the program
    event_sender.send(Event::new("[schedule.update_thread]", Some(vec!["time".to_string(), "Running".to_string()]))).unwrap();
     
    // join the last thread to keep the entire program from dropping itself
    join_handle.join().expect("Couldn't join on the associated thread");
}


pub struct App {
    pub model_scheduler: ModelScheduler,
    pub store: Store,
    pub dispatcher: Dispatcher,
    pub window_viewer: WindowViewer,
}


impl App {

    pub fn new() -> App {

        // Coffee Gui's State
        let state = State::new();

        // setup the Store to manage the state
        let mut store = Store::new(bincode::serialize(&state).unwrap()); 

        // initialize the reducers
        state::reducers::setup(&mut store);

        // get an action sender for the model_scheduler
        let dispatcher_action_tx = store.get_action_sender();
        let dispatch_handler = DispatchHandler {};
        let dispatcher: Dispatcher = Dispatcher::new(dispatcher_action_tx, Box::new(dispatch_handler));


        let event_sender = dispatcher.get_event_sender();
        let model_scheduler = model_setup(event_sender.clone(), &mut store);

        // add our views to the root view
        let window_viewer = views::setup(event_sender,  &mut store);
        App {
            window_viewer,
            model_scheduler,
            store,
            dispatcher
        } 
    }
}
