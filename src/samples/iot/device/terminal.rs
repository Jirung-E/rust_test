use std::thread::sleep;
use rand::Rng;

use super::{ Interface, Core };

pub struct Terminal {
    core: Core
}

impl Terminal {
    pub fn new(id: u32, name: &str) -> Box<Terminal> {
        Box::new(Terminal { core: Core::new(id, name) })
    }
}

impl Interface for Terminal {
    fn execute(&mut self) -> bool {
        let id = format!("{}(0x{:0x})", self.get_name(), self.get_id());

        if self.is_running() {
            println!("{} is already running", id);
            return false;
        }

        println!("executing {}...", id);
        let result = rand::thread_rng().gen_range(0..100);
        if result == 0 {
            println!("failed");
            return false;
        }

        self.core.running = true;

        let bar_length = 50;
        let item_count = rand::thread_rng().gen_range(100..200);
        let mut loaded_item_count = 0;

        while loaded_item_count < item_count {
            loaded_item_count += 1;

            let per = loaded_item_count as f32 / item_count as f32;

            let loaded_bar_length = (per * bar_length as f32) as usize;
            let bar = format!("{}{}", "#".repeat(loaded_bar_length), ".".repeat(bar_length - loaded_bar_length));

            print!("\r {:.1}% [{}]", per * 100.0, bar);
            // let bar = format!("{}{}", "━".repeat(loaded_bar_length), " ".repeat(bar_length - loaded_bar_length));
            // print!("\r {:.1}% [{}]", loaded_item_count * 100 / item_count, bar);
            sleep(std::time::Duration::from_millis(5));
        }

        println!("\n{} successfully executed", id);
        true
    }

    fn shutdown(&mut self) {
        self.core.running = false;
    }

    fn get_id(&self) -> u32 {
        self.core.get_id()
    }

    fn get_name(&self) -> String {
        self.core.get_name()
    }

    fn is_running(&self) -> bool {
        self.core.is_running()
    }

    fn get_connected_devices(&self) -> &Vec<Box<dyn Interface>> {
        self.core.get_connected_devices()
    }

    fn connect_device(&mut self, device: Box<dyn Interface>) {
        self.core.connect_device(device);
    }
}