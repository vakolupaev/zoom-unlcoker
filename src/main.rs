extern crate serial;
use std::{io::{stdin, Write}, thread, time::Duration};
use serial::{PortSettings, SerialPort};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello");
    let mut port = serial::open("/dev/ttyS2").unwrap();
    let pelco_settings = PortSettings {
        baud_rate: serial::BaudRate::Baud57600,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone
    };
    let visca_settings = PortSettings {
        baud_rate: serial::BaudRate::Baud9600,
        char_size: serial::CharSize::Bits8,
        parity: serial::Parity::ParityNone,
        stop_bits: serial::StopBits::Stop1,
        flow_control: serial::FlowControl::FlowNone
    };


    port.configure(&pelco_settings).unwrap();
    init(&mut port);
    thread::sleep(Duration::from_secs(1));
    pelco_d_disable_af(&mut port);
    pelco_d_disable_af(&mut port);
    pelco_d_disable_af(&mut port);
    pelco_d_disable_af(&mut port);
    pelco_d_disable_af(&mut port);
    thread::sleep(Duration::from_secs(1));

    port.configure(&visca_settings).unwrap();
    thread::sleep(Duration::from_secs(5));
    visca_disable_af(&mut port);
    visca_disable_af(&mut port);
    visca_disable_af(&mut port);
    visca_disable_af(&mut port);
    visca_disable_af(&mut port);
    thread::sleep(Duration::from_secs(1));

    let mut data: Vec<u8>;
    let mut s1 = String::new();
    let mut s2 = String::new();

    loop {
        println!("enter zoom value (hex)");
        stdin().read_line(&mut s1).expect("Did not enter a correct string");
        let zoom_value: u32 = u32::from_str_radix(s1.trim(), 16).unwrap_or(0);
        let zoom_bytes = zoom_value.to_le_bytes();

        println!("enter focus value (hex)");
        stdin().read_line(&mut s2).expect("Did not enter a correct string");
        let focus_value: u32 = u32::from_str_radix(s2.trim(), 16).unwrap_or(0);
        let focus_bytes = focus_value.to_le_bytes();

        data = vec![0x81, 0x01, 0x04, 0x47, zoom_bytes[3], zoom_bytes[2], zoom_bytes[1], zoom_bytes[0], 0xFF];
        println!("{:X?}", data);
        send(&mut port, &data).unwrap();
        send(&mut port, &data).unwrap();
        send(&mut port, &data).unwrap();
        thread::sleep(Duration::from_millis(100));
        data = vec![0x81, 0x01, 0x04, 0x48, focus_bytes[3], focus_bytes[2], focus_bytes[1], focus_bytes[0], 0xFF];
        println!("{:X?}", data);
        send(&mut port, &data).unwrap();
        send(&mut port, &data).unwrap();
        send(&mut port, &data).unwrap();
        
        s1.clear();
        s2.clear();
    }
}

fn send(port: &mut serial::unix::TTYPort, data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    match port.write(data) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error writing to serial port: {}", e);
            return Err(Box::new(e));
        }
    }

    port.flush().unwrap();
    Ok(())
}

fn init(port: &mut serial::unix::TTYPort) {
    let mut data: Vec<u8>;

    data = vec![0x45, 0x4E, 0x44, 0xD7, 0x45, 0x4E, 0x44, 0xD7, 0x45, 0x4E, 0x44, 0xD7, 0x45, 0x4E, 0x44, 0xD7];
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    data = vec![0x56, 0x45, 0x52, 0xED];
    send(port, &data).unwrap();

    thread::sleep(Duration::from_secs(1));

    data = vec![0x45, 0x4E, 0x44, 0xD7, 0x45, 0x4E, 0x44, 0xD7, 0x45, 0x4E, 0x44, 0xD7, 0x45, 0x4E, 0x44, 0xD7];
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    data = vec![0x45, 0x58, 0x56, 0x45, 0x52, 0x8A];
    send(port, &data).unwrap();

    thread::sleep(Duration::from_secs(10));

    data = vec![0x51, 0x01, 0x04, 0x78, 0x01, 0x0D, 0x05, 0x61, 0x01, 0x1E, 0x03, 0x00, 0x03, 0x00, 0x00, 0x67];
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
}

fn pelco_d_disable_af(port: &mut serial::unix::TTYPort) {
    let data: Vec<u8>;
    data = vec![0xFF, 0x01, 0x00, 0x2B, 0x00, 0x01, 0x2D];
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
}

fn visca_disable_af(port: &mut serial::unix::TTYPort) {
    let data: Vec<u8> = vec![0x81, 0x01, 0x04, 0x38, 0x03, 0xFF];
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
    send(port, &data).unwrap();
}