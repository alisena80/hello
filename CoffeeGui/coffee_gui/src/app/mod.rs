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




pub fn run_app(app: App) {
    let event_sender = app.dispatcher.get_event_sender();
    run_window_viewer(app.window_viewer);
    run_store(app.store);
    // join the last thread
    run_dispatcher(app.dispatcher);
    let join_handle = run_model_scheduler(app.model_scheduler);
    event_sender.send(Event::new("[schedule.update_thread]", Some(vec!["time".to_string(), "Running".to_string()]))).unwrap();
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

        // setup the state object
        let mut store = Store::new(bincode::serialize(&state).unwrap()); 

        // initialize the mutators so it can mutate
        state::reducers::setup(&mut store);


        // get a state mutation sender for time keeping thread
        // we setup a time keeper thread on a 1 second 
        // resolution to trigger initial state senders

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
