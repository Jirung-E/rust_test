use super::{ Interface, Core };

pub struct Hub {
    core: Core
}

impl Hub {
    pub fn new(id: u32, name: &str) -> Box<Hub> {
        Box::new(Hub { core: Core::new(id, name) })
    }
}

impl Interface for Hub {
    fn execute(&mut self) -> bool {
        for device in self.core.connected_devices.iter_mut() {
            if !device.is_running() {
                device.execute();
            }
        }

        self.core.running = true;

        true
    }

    fn shutdown(&mut self) {
        for device in self.core.connected_devices.iter_mut() {
            device.shutdown();
        }
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

    fn status(&self) -> String {
        let mut result = self.core.status();
        for (i, device) in self.core.connected_devices.iter().enumerate() {
            result += "\n";
            let res = device.status().replace("\n", "\n  ");
            result += &format!("#{}: {}", i, res);
        }
        result
    }

    fn get_connected_devices(&self) -> &Vec<Box<dyn Interface>> {
        self.core.get_connected_devices()
    }

    fn connect_device(&mut self, device: Box<dyn Interface>) {
        self.core.connect_device(device);
    }
}