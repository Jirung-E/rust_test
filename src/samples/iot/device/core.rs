use super::Interface;


pub struct Core {
    pub id: u32,
    pub name: String,
    pub running: bool,
    pub connected_devices: Vec<Box<dyn Interface>>
}

impl Core {
    pub fn new(id: u32, name: &str) -> Core {
        Core {
            id,
            name: name.to_string(),
            running: false,
            connected_devices: Vec::new()
        }
    }
}


impl Interface for Core {
    fn execute(&mut self) -> bool {
        true
    }

    fn shutdown(&mut self) {
        self.running = false;
    }


    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_running(&self) -> bool {
        self.running
    }

    fn get_connected_devices(&self) -> &Vec<Box<dyn Interface>> {
        &self.connected_devices
    }

    fn connect_device(&mut self, device: Box<dyn Interface>) {
        self.connected_devices.push(device);
    }
}