use std::thread::sleep;
use rand::Rng;

pub trait Terminal {
    fn execute(&mut self) -> bool;
    fn get_id(&self) -> u32;
    fn get_name(&self) -> String;
    fn shutdown(&mut self);
}

pub struct TerminalNode {
    id: u32,
    name: String,
    shutdown: bool
}

impl TerminalNode {
    pub fn new(id: u32, name: &str) -> TerminalNode {
        TerminalNode {
            id,
            name: name.to_string(),
            shutdown: true
        }
    }
}

impl Terminal for TerminalNode {
    fn execute(&mut self) -> bool {
        let id = format!("{}(0x{:0x})", self.name, self.id);

        println!("executing {}...", id);
        let result = rand::thread_rng().gen_range(0..100);
        if result == 0 {
            println!("failed");
            return false;
        }

        self.shutdown = false;

        let bar_length = 50;
        for _ in 0..5 {
            let item_count = rand::thread_rng().gen_range(100..700);
            let mut loaded_item_count = 0;

            while loaded_item_count < item_count {
                loaded_item_count += 1;

                let loaded_bar_length = loaded_item_count * bar_length / item_count;
                let bar = format!("{}{}", "=".repeat(loaded_bar_length), " ".repeat(bar_length - loaded_bar_length));
                // print!("\r {} / {} [ {} ] {:.1}%", loaded_item_count, item_count, bar, loaded_item_count as f64 * 100.0 / item_count as f64);
                print!("\r {} / {} [{}] {:.1}%", loaded_item_count, item_count, bar, loaded_item_count as f64 * 100.0 / item_count as f64);
                sleep(std::time::Duration::from_millis(5));
            }
            println!();
        }

        println!("\n{} successfully executed", id);
        true
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn shutdown(&mut self) {
        self.shutdown = true;
    }
}