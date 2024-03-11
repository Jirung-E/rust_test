use super::Draw;

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox 를 그렸습니다.");
    }
}