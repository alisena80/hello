use uuid::Uuid;

use super::canvas::Canvas;


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
    fn draw(&self, canvas: &mut Canvas) -> bool;

    fn select(&mut self, canvas: &mut Canvas) {}    

    fn deselect(&mut self, canvas: &mut Canvas) {}    

    fn click(&mut self, canvas: &mut Canvas) {}    

    fn lock(&mut self, canvas: &mut Canvas) {}    

    fn unlock(&mut self, canvas: &mut Canvas) {}    

    fn show(&mut self, canvas: &mut Canvas) {}    

    fn hide(&mut self, canvas: &mut Canvas) {}    

    fn update(&mut self) -> bool {
        true
    }
}

//Base Gui Impl



pub struct Button {
    pub text: &'static str,
    pub action: GuiAction,
    pub name: String
}

impl Button {
    pub fn new(text: &'static str, action: GuiAction) -> Button {
            
        let name = format!("Button - {}", Uuid::new_v4()); 
        // add elets to canvas
        
          // text

          // Rect border

          // Rect Background



        Button {
            text,
            action,
            name
        }

    }
}
impl Gui for Button {
    fn draw(&self, canvas: &mut Canvas) -> bool{
        true
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
