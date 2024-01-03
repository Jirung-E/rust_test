// iot hub

mod device;
use device::{ Interface, Hub, Terminal };


pub fn test() {
    println!(" [ iot hub ] ");

    let mut hub = Hub::new(0x0, "IOT Hub Home");

    let l1 = Terminal::new(0x06008742, "IOT Light1");
    hub.connect_device(Box::new(l1));
    // hub.execute();
    // println!("{}", hub.status());

    {
        let mut h1 = Hub::new(0x01, "IOT Hub 1");

        let l2 = Terminal::new(0x06008742, "IOT Light 2");
        let l3 = Terminal::new(0x06008742, "IOT Light 3");
        h1.connect_device(Box::new(l2));
        h1.connect_device(Box::new(l3));

        let mut h2 = Hub::new(0x02, "IOT Hub 2");

        let l4 = Terminal::new(0x06008742, "IOT Light 4");
        h2.connect_device(Box::new(l4));

        h1.connect_device(Box::new(h2));

        hub.connect_device(Box::new(h1));
    }

    let s1 = Terminal::new(0x54106418, "Switch 1");
    let s2 = Terminal::new(0x54106418, "Switch 2");

    hub.connect_device(Box::new(s1));
    hub.connect_device(Box::new(s2));

    hub.execute();

    println!("\n{}\n", hub.status());
}
