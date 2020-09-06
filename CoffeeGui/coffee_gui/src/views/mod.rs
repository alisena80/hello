use super::canvas::Canvas;
use super::gui_tk::{Gui, Button, GuiAction};
use super::state::{State, Mutator, RootState};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

pub fn run_view(mut root_view: RootView){
    thread::spawn(move || {
        loop {
            match root_view.state_receiver.try_recv() {
                Ok(state) => {
                    root_view.updateBar(state.clone());
                    root_view.updateActiveView(state.clone());
                    root_view.render();
                    println!("State Update");
                },
                Err(_) => ()
            };
        };
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
    pub fn new(fbdev: &'static str, state_receiver: Receiver<State>, root_state: &mut RootState) -> RootView {
        let canvas: Canvas = Canvas::new(fbdev);
        let mut root_view = RootView {
            bar: InfoBar::new(root_state),
            views: vec![],
            canvas: canvas,
            active: 0,
            state_receiver
        };
        root_view.activateBar();
        root_view
    }
    // draw it out
    pub fn render(&mut self) {
        self.canvas.render();
    }

    // update the top bar
    pub fn updateBar(&mut self, state: State) -> bool {
        self.bar.update(state, &mut self.canvas)
    }

    pub fn activateBar(&mut self) -> bool {
        self.bar.activate(&mut self.canvas)
    }

    // this is a move operation
    pub fn addView<T: 'static >(&mut self, view: T) where T: View + Send {
        self.views.push(Box::new(view));
    }

    pub fn updateActiveView(&mut self, state: State){
        if self.views.len() > self.active {
            self.views[self.active].update(state, &mut self.canvas);
        } else {
            panic!("Cannot activate a view which does not exist");
        }
   
    }

    // for user input routing
    pub fn setActiveView(&mut self, view: usize) {
        if self.views.len() <= view {
            panic!("Cannot activate a view which does not exist");
        }
        for i in (0 as usize)..self.views.len() {
            if i == view{
               self.active = view;
               self.views[self.active].activate(&mut self.canvas);
            } else {
                self.views[i].deactivate(&mut self.canvas);
            }
        }
    }
}

//abstract trait to impl the view Trait while keeping it dry

// View Trait ... all Views can do this!
pub trait View {
    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        true
    }
    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
        true
    }

    fn update(&mut self, state: State, canvas: &mut Canvas) -> bool {
        true 
    }
    fn deactivate(&mut self, canvas: &mut Canvas) -> bool {
        true
    }
}




struct InfoBar {
    objects: Vec<Box<dyn Gui + Send>>  
}

impl InfoBar {
    pub fn new(root_state: &mut RootState) -> InfoBar {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];
        let button: Box<Button> = Box::new(Button::new("00:00".to_string(), 0, 0, 100, 24, GuiAction::new("Time Click", None)));
        root_state.state.views.bar.push(button.gui_state.clone());
        objects.push(button);

        InfoBar {
            objects: objects
        }
    }
    // since infobar is always in view initialize and activate functions are combined
    // there is no deactivate
    pub fn activate(&mut self,  canvas: &mut Canvas) -> bool {
         for i in (0 as usize)..(self.objects.len() as usize) {
            if !(self.objects[i].initialize(canvas) && self.objects[i].activate(canvas)) {
                println!("Could not get bar objects initialized and activated!");
                return false;
            }
        }
        true
    }
    pub fn update(&mut self, state: State, canvas: &mut Canvas) -> bool{
        //update each object in the view with the correct state data
        self.objects[0].setText(state.time.current_time.clone(), canvas);
        self.objects[0].setGuiState(state.views.bar[0].clone(), canvas);
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
impl View for SettingsView {
    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
         for i in (0 as usize)..self.objects.len() {
            if !self.objects[i].initialize(canvas) {
                return false;
            }
        }
        true
    }

    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        for i in (0 as usize)..self.objects.len() {
            if !self.objects[i].activate(canvas) {
                return false;
            }
        }
        true
       // all objects 
    }
    fn update(&mut self, state: State, canvas: &mut Canvas) -> bool {
        //update each object in the view with the correct state data
        self.objects[0].setText(state.time.current_time.clone(), canvas);
        self.objects[0].setGuiState(state.views.settings[0].clone(), canvas);
        true 
    }
    fn deactivate(&mut self, canvas: &mut Canvas) -> bool {
         for i in (0 as usize)..self.objects.len() {
            if !self.objects[i].deactivate(canvas) {
                return false;
            }
        }
        true
    }
   
}

impl SettingsView {
    pub fn new(root_state: &mut RootState) -> SettingsView {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];
        let button: Box<Button> = Box::new(Button::new("00:00".to_string(), 0, 30, 100, 32, GuiAction::new("Time Click", None)));
        root_state.state.views.settings.push(button.gui_state.clone());
        objects.push(button);

        SettingsView {
            objects
        }
    }
}
