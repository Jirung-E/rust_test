mod button;
pub use button::Button;
mod select_box;
pub use select_box::SelectBox;

pub trait Draw {
    fn draw(&self);
}


pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
