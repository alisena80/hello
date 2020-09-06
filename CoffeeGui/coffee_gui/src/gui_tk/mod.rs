use uuid::Uuid;

use super::canvas::{Rect, Layer, Draw, Canvas, Text};
use super::fb::Color;
use super::state::State;

#[derive(Clone, Debug)]
pub enum GuiState{
    Base,
    Clicked,
    Selected
}

//color pallete
struct Palette {
    base: Color,
    base_text: Color,
    selected: Color,
    selected_text: Color,
    background: Color,
}

impl Palette {
    fn new() -> Palette {
        Palette {
            base: Color::new(78, 156, 183),
            base_text: Color::new(255, 255, 255),
            selected: Color::new(78, 156, 183),
            selected_text: Color::new(0, 0, 0),
            background: Color::new(30, 50, 50)

        }
    }
}
// Gui Objects

// abstract trait for keeping it Dry

// Gui Trait
pub trait Gui {
/*    fn select(&mut self, canvas: &mut Canvas);
    fn deselect(&mut self, canvas: &mut Canvas);
    fn click(&mut self, canvas: &mut Canvas);
    fn lock(&mut self, canvas: &mut Canvas);
    fn unlock(&mut self, canvas: &mut Canvas);
    fn show(&mut self, canvas: &mut Canvas);
    fn hide(&mut self, canvas: &mut Canvas);
*/
//    fn draw(&self, canvas: &mut Canvas) -> bool;

    fn select(&mut self, canvas: &mut Canvas) {}    

    fn deselect(&mut self, canvas: &mut Canvas) {}    

    fn click(&mut self, canvas: &mut Canvas) {}    

    fn lock(&mut self, canvas: &mut Canvas) {}    

    fn unlock(&mut self, canvas: &mut Canvas) {}    

    fn show(&mut self, canvas: &mut Canvas) {}    

    fn hide(&mut self, canvas: &mut Canvas) {}    

    // move the layers over to the canvas
    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
        true
    } 

    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        // sets the current state layers to active on the canvas
        true
    }

    fn deactivate(&mut self, canvas: &mut Canvas) -> bool {
        true
    }

    fn setText(&mut self,  text: String, canvas: &mut Canvas){
        ()
    }
    fn setGuiState(&mut self, gui_state: GuiState, canvas: &mut Canvas){
        ()
    }

}

//Base Gui Impl

pub struct Button {
    pub text: String,
    pub action: GuiAction,
    pub name: String,
    pub regular_name: String,
    pub selected_name: String,
    pub clicked_name: String,
    // cloned and appended to canvas
    pub layers: Vec<Layer<Box<dyn Draw + Send>>>,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub gui_state: GuiState
}

impl Button {
    pub fn new(text: String, x: i32, y: i32, w: i32, h: i32, action: GuiAction) -> Button {
        let uuid_string = Uuid::new_v4().to_hyphenated().to_string();
        let name = format!("Button - {}", uuid_string); 

        let regular_name = format!("{} - regular", name);
        let selected_name = format!("{} - selected", name);
        let clicked_name = format!("{} - clicked", name);

        let gui_state =  GuiState::Base;


        let mut layers: Vec<Layer<Box<dyn Draw + Send>>> = vec![];
        let mut button = Button {
            text,
            action,
            name,
            regular_name,
            clicked_name,
            selected_name,
            layers,
            x,
            y,
            w,
            h,
            gui_state
        };
        button.gen_layers();
        button
    }
    pub fn reinit(&mut self, canvas: &mut Canvas){
        canvas.drop_layer_group(self.regular_name.clone());
        canvas.drop_layer_group(self.selected_name.clone());
        canvas.drop_layer_group(self.clicked_name.clone());
 
        //gen new layers
        self.gen_layers();
        self.initialize(canvas);


    }

    pub fn gen_layers(&mut self)  {
        let palette = Palette::new();
        // basic background box
        self.layers.push(
             Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, true, palette.background.clone())), true, self.regular_name.clone())
        );
        // basic outline box
        self.layers.push(
             Layer::new(Box::new(Rect::new(self.x, self.y, self.w, self.h, false, palette.base.clone())), true, self.regular_name.clone())
        );
        //basic text
        self.layers.push(
             Layer::new(Box::new(Text::new(self.x, self.y, self.h as f32, self.text.clone(), "./assets/fonts/Antic_Slab/AnticSlab-Regular.ttf",  palette.base_text.clone(), 2),), true, self.regular_name.clone())
        );
    }

}
impl Gui for Button {
    fn initialize(&mut self, canvas: &mut Canvas) -> bool {
        // add all layers to the canvas
        // self.layers is now empty
        canvas.layers.append(&mut self.layers);
        true
    }
    fn activate(&mut self, canvas: &mut Canvas) -> bool {
        //set the correct layers to active        
        match self.gui_state {
            GuiState::Base => {
                canvas.activate_layer_group(self.regular_name.clone());
                canvas.deactivate_layer_group(self.selected_name.clone());
                canvas.deactivate_layer_group(self.clicked_name.clone());
            },
            GuiState::Clicked => {
                canvas.deactivate_layer_group(self.regular_name.clone());
                canvas.deactivate_layer_group(self.selected_name.clone());
                canvas.activate_layer_group(self.clicked_name.clone());
            },
            GuiState::Selected => {
                canvas.deactivate_layer_group(self.regular_name.clone());
                canvas.activate_layer_group(self.selected_name.clone());
                canvas.deactivate_layer_group(self.clicked_name.clone());
            }
            
        };
        true
    }

    fn deactivate(&mut self, canvas: &mut Canvas) -> bool{
        canvas.deactivate_layer_group(self.regular_name.clone());
        canvas.deactivate_layer_group(self.selected_name.clone());
        canvas.deactivate_layer_group(self.clicked_name.clone());
        true 
    }

    fn setText(&mut self, text: String, canvas: &mut Canvas) {
        self.text = text;
        self.reinit(canvas);
    }
    fn setGuiState(&mut self, gui_state: GuiState, canvas: &mut Canvas){
        self.gui_state = gui_state;
        self.activate(canvas);
    }

}


pub struct Menu {
    pub items: Vec<MenuItem>,
    pub action: GuiAction,
    pub name: &'static str
}

pub struct MenuItem {
    pub text: &'static str,
    pub action: GuiAction,
    pub name: &'static str
}

pub struct GuiImage {
    pub path: &'static str,
    pub action: GuiAction,
    pub name: &'static str
}

pub struct TextBox {
    pub text: &'static str,
    pub action: GuiAction,
    pub name: &'static str
}


pub struct GuiAction {
    name: &'static str,
    values: Option<Vec<&'static str>>
}

impl GuiAction {
    pub fn new(name: &'static str, values: Option<Vec<&'static str>>) -> GuiAction {
        GuiAction {
            name, values
        }
    }
}
