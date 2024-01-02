// iot hub

mod terminal;
mod hub;

use terminal::TerminalNode;
use crate::samples::iot::terminal::Terminal;

pub fn test() {
    println!(" [ station ] ");

    let mut term = TerminalNode::new(0x54106418, "terminal node 1");
    term.execute();
}