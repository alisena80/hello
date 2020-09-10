use super::canvas::Canvas;
use super::gui_tk::{Gui, Button, GuiAction};
use super::state::{State, RootState};
use std::sync::mpsc::{ Receiver };
use super::joy_pad::{ButtonAction, Action};
use std::thread;

/*
     ButtonInitializer {pin: 5, code: 0, key: "b"},
     ButtonInitializer {pin: 6, code: 1, key: "a"},
     ButtonInitializer {pin: 27, code: 2, key: "l"},
     ButtonInitializer {pin: 23, code: 3, key: "r"},
     ButtonInitializer {pin: 17, code: 4, key: "up"},
     ButtonInitializer {pin: 22, code: 5, key: "dn"},
     ButtonInitializer {pin: 4, code:  6, key: "hat"},
*/


enum InputMode {
    Navigate,
    Manipulate
}

pub fn run_view(mut root_view: RootView){
    thread::spawn(move || {
        loop {
            match root_view.input_receiver.try_recv() {
                Ok(button_actions) => {
                    for ba in &button_actions {
                        root_view.handle_button_action(ba);
                    }
                },
                Err(_) => ()
            }

            match root_view.state_receiver.try_recv() {
                Ok(state) => {
                    root_view.update_bar(state.clone());
                    root_view.update_active_view(state.clone());
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
    state_receiver: Receiver<State>,
    input_receiver: Receiver<Vec<ButtonAction>>
}

impl RootView {
    pub fn new(fbdev: &'static str, state_receiver: Receiver<State>, root_state: &mut RootState, input_receiver: Receiver<Vec<ButtonAction>>) -> RootView {
        let canvas: Canvas = Canvas::new(fbdev);
        let mut root_view = RootView {
            bar: InfoBar::new(root_state),
            views: vec![],
            canvas: canvas,
            active: 0,
            state_receiver,
            input_receiver
        };
        root_view.activate_bar();
        root_view
    }

    // input button handling
    pub fn handle_button_action(&mut self, ba: &ButtonAction){
        if self.views.len() > self.active {
            self.views[self.active].handle_button_action(ba);
        } else {
            panic!("Cannot route input to non existent active view");
        }
    }

    // draw it out
    pub fn render(&mut self) {
        self.canvas.render();
    }

    // update the top bar
    pub fn update_bar(&mut self, state: State) -> bool {
        self.bar.update(state, &mut self.canvas)
    }

    pub fn activate_bar(&mut self) -> bool {
        self.bar.activate(&mut self.canvas)
    }

    // this is a move operation
    pub fn add_view<T: 'static >(&mut self, view: T) where T: View + Send {
        if view.objects_len() > 0 {
            self.views.push(Box::new(view));
        } else {
            panic!("Cannot add a view with 0 objects")
        }
    }

    pub fn update_active_view(&mut self, state: State){
        if self.views.len() > self.active {
            self.views[self.active].update(state, &mut self.canvas);
        } else {
            panic!("Cannot activate a view which does not exist");
        }
   
    }

    // for user input routing
    pub fn set_active_view(&mut self, view: usize) {
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
    fn handle_button_action(&mut self, ba: &ButtonAction) -> bool {
        true
    }
    fn objects_len(&self) -> usize {
        0
    }
}




struct InfoBar {
    objects: Vec<Box<dyn Gui + Send>>  
}

impl InfoBar {
    pub fn new(root_state: &mut RootState) -> InfoBar {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];
        let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 0, 140, 28, GuiAction::new("Time Click", None)));
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
        self.objects[0].set_text(state.time.current_time.clone(), canvas);
        self.objects[0].set_gui_state(state.views.bar[0].clone(), canvas);
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
    objects: Vec<Box<dyn Gui + Send>>,
    input_mode: InputMode,
    nav_index: Vec<Vec<Vec<usize>>>,
    selected_row: usize,
    selected_column: usize,
    selected_object: usize
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
        self.objects[0].set_text(state.time.current_time.clone(), canvas);
        self.objects[0].set_gui_state(state.views.settings[0].clone(), canvas);
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

    fn handle_button_action(&mut self, ba: &ButtonAction) -> bool {
        // nav mode or manipulate mode?
        match self.input_mode {
            InputMode::Navigate => (), // up / down / right / left will move the selection from widget to widget -- b = home, a = back == home
            InputMode::Manipulate => () // the element will parse the the input mode for the input bus. -- b = home, a = back == navigate mode
        }
        true    
    }
    fn objects_len(&self) -> usize {
        self.objects.len()
    }   
}

impl SettingsView {
    pub fn new() -> SettingsView {
        let mut objects: Vec<Box<dyn Gui + Send>> = vec![];
        let mut nav_index: Vec<Vec<Vec<usize>>> = vec![
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ],
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ],
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ],
                                                      vec![
                                                            vec![], vec![], vec![], vec![],
                                                          ]
                                                    ];
//        let button: Box<Button> = Box::new(Button::new("00:00:00 XX".to_string(), 0, 28, 200, 32, GuiAction::new("Time Click", None)));
        let selected_row = 0;
        let selected_column = 0;
        let selected_object =  0;   
        SettingsView {
            objects,
            input_mode: InputMode::Navigate,
            nav_index,
            selected_row,
            selected_column,
            selected_object
        }
    }
    pub fn add_object(&mut self, object: Box<dyn Gui + Send>, row: usize, column: usize, root_state: &mut RootState) {
        let object_index = self.objects.len(); //
        root_state.state.views.settings.push(object.get_gui_state());
        self.objects.push(object);
        if self.nav_index.len() > row && self.nav_index[row].len() > column {
            self.nav_index[row][column].push(object_index);
        } else {
            panic!("Row and Column outside bounds");
        }
        
    }

}
