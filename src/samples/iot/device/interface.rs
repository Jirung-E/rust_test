pub trait Interface {
    fn execute(&mut self) -> bool;
    fn shutdown(&mut self);

    fn run_command(&mut self, command: &str) -> bool {
        match command {
            "shutdown" => {
                self.shutdown();
                true
            },
            "status" => {
                println!("{}", self.status());
                true
            },
            _ => false
        }
    }

    fn get_id(&self) -> u32;
    fn get_name(&self) -> String;
    fn is_running(&self) -> bool;
    fn status(&self) -> String {
        let mut result = format!("{}(0x{:0x}) ", self.get_name(), self.get_id());
        if self.is_running() {
            result += "[running]";
        } else {
            result += "[stopped]"
        }
        result
    }

    fn get_connected_devices(&self) -> &Vec<Box<dyn Interface>>;
    fn connect_device(&mut self, device: Box<dyn Interface>);
}