// iot hub

mod device;
use device::{ Interface, Hub, Terminal };


pub fn test() {
    println!(" [ iot hub ] ");

    let mut hub = Hub::new(0x0, "IOT Hub Home");
    let mut h1 = Hub::new(0x01, "IOT Hub 1");
    let mut h2 = Hub::new(0x02, "IOT Hub 2");

    let l1 = Terminal::new(0x06008742, "IOT Light1");
    let l2 = Terminal::new(0x06008742, "IOT Light 2");
    let l3 = Terminal::new(0x06008742, "IOT Light 3");
    let l4 = Terminal::new(0x06008742, "IOT Light 4");
    let s1 = Terminal::new(0x54106418, "Switch 1");
    let s2 = Terminal::new(0x54106418, "Switch 2");

    hub.connect_device(l1);
    h1.connect_device(l2);
    h1.connect_device(l3);
    h2.connect_device(l4);
    hub.connect_device(s1);
    hub.connect_device(s2);

    h1.connect_device(h2);
    hub.connect_device(h1);


    hub.execute();

    println!("\n{}\n", hub.status());
}
