use super::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button 을 그렸습니다.");
    }
}