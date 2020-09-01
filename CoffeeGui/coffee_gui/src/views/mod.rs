use super::canvas::Canvas;
use super::gui_tk::{Gui, Button, GuiAction};
use super::state::{State, Mutator};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

pub fn run_view(mut root_view: RootView){
    thread::spawn(move || {
        loop {
            match root_view.state_receiver.try_recv() {
                Ok(state) => {
                    root_view.bar.update(state.clone());
                    root_view.updateActiveView(state.clone());
                },
                Err(_) => ()
            }
        }
    });
}


pub struct RootView {
    bar: InfoBar,
    views: Vec<Box<dyn View + Send>>,
    active: usize,
    canvas: Canvas,
    state_receiver: Receiver<State>
}

impl RootView {
    pub fn new(fbdev: &'static str, state_receiver: Receiver<State>) -> RootView {
        let canvas: Canvas = Canvas::new(fbdev);
        RootView {
            bar: InfoBar::new(),
            views: vec![],
            canvas: canvas,
            active: 0,
            state_receiver
        }
    }

    // this is a move operation
    pub fn addView<T: 'static >(&mut self, view: T) where T: View + Send {
        self.views.push(Box::new(view));
    }

    pub fn updateActiveView(&mut self, state: State){
        if self.views.len() > self.active {
            self.views[self.active].update(state);
        } else {
            panic!("Cannot activate a view which does not exist");
        }
   
    }

    // for user input routing
    pub fn setActiveView(&mut self, view: usize) {
        if self.views.len() > view {
            self.active = view;
            self.views[self.active].activate();
        } else {
            panic!("Cannot activate a view which does not exist");
        }
    }
}

//abstract trait to impl the view Trait while keeping it dry

// View Trait ... all Views can do this!
pub trait View {
    fn activate(&mut self) -> bool {
        true
    }
    fn update(&mut self, state: State) -> bool {
        true 
    }
}




struct InfoBar {
    objects: Vec<Box<dyn Gui + Send>>  
}

impl InfoBar {
    pub fn new() -> InfoBar {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];
        objects.push(Box::new(Button::new("00:00", GuiAction::new("Time Click", None))));

        InfoBar {
            objects: objects
        }
    }

    pub fn update(&mut self, state: State) -> bool{
        for i in (0 as usize)..(self.objects.len() as usize) {
            if !self.objects[i].update() {
                return false;
            }
        }
        true
    }
}

struct BoilerView {

}
impl View for BoilerView{}

struct SteamerView {

}
impl View for SteamerView {}


pub struct SettingsView {
    objects: Vec<Box<dyn Gui + Send>>
}
impl View for SettingsView {}

impl SettingsView {
    pub fn new() -> SettingsView {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];
        objects.push(Box::new(Button::new("00:00", GuiAction::new("Time Click", None))));

        SettingsView {
            objects
        }
    }
}
