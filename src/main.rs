/*

RUSEMI - RUST-BASED USB SERIAL TO COREMIDI CONVERTER

Rusemi is a command line utility written in Rust and designed to receive MIDI-formatted messages from a USB serial port and forward them to a virtual MIDI port via CoreMIDI.
http://github.com/icaroferre/rusemi

Created by @icaroferre
icaroferre.com

*/


use std::io::{self, Write};
use std::time::{Duration};
extern crate coremidi;
extern crate chrono;
use std::sync::mpsc;
use chrono::Local;

fn main() {
    println!("\nRUSEMI - Rust USB Serial to MIDI CLI\nDeveloped by Icaro Ferre");    
    
    // Initialize serial port
    let port_name = get_serial_port();
    let baud_rate = 31250;

    println!("Opening port: {}", port_name);
    let port = serialport::new(port_name.clone(), baud_rate)
        .timeout(Duration::from_millis(10))
        .open();
    
    let (tx, rx) = mpsc::channel();

    // Set up CoreMIDI client and virtual ports
    println!("Setting up CoreMIDI virtual ports...");
    let client = coremidi::Client::new("Rusemi CoreMIDI Client").unwrap();
    let output_port = client.virtual_source("from Rusemi").unwrap();
    
    /* 
    The callback function for MIDI in forwards incoming bytes to a MPSC channel which is then read by the main loop.
    This is done because the callback runs in a separate closure and therefore it can't access the main serial port object (which can't also be cloned). 
    */
    let callback =  move |packet_list: &coremidi::PacketList| {
        for i in packet_list.iter() {
            for b in i.data() {
                tx.send(b.clone() as u8).unwrap();
            }
            
        }
    };
    let _input_port = client.virtual_destination("to Rusemi", callback).unwrap();

    // Main loop
    match port {
        // Check if serial port is available
        Ok(mut port) => {
            // Create serial read buffer
            let mut serial_buf: Vec<u8> = vec![0; 3];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                // Read serial port for new bytes
                match port.read(serial_buf.as_mut()) {
                    Ok(_t) => {
                        let serial_owned = serial_buf.to_owned();
                        // The first byte indicates the type of the MIDI message (Note / CC / etc)
                        let cmd = serial_owned[0];
                        match cmd {
                            // MIDI NOTES
                            144..=159 => {
                                let pitch = serial_owned[1];
                                let vel = serial_owned[2];
                                let note = match vel {
                                    0 => create_note_off(cmd - 144, pitch, 0),
                                    _ => create_note_on(cmd - 144, pitch, vel)
                                };
                                output_port.received(&note).unwrap();
                            }
                            // CONTROL CHANGES
                            176..=191 => {
                                let cc = serial_owned[1];
                                let value = serial_owned[2];
                                let cc_msg = create_cc(cmd - 176, cc, value);
                                output_port.received(&cc_msg).unwrap();
                            }
                            _ => {println!("Unknown MIDI message: {:?}", serial_owned);}
                        }
                    }, 
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                // Reads bytes sent from the MIDI IN callback to the MPSC channel and forwards them to the serial port
                for i in rx.try_iter() {
                    let s = vec![i];
                    port.write(&s[0..1]).unwrap();
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }

}


// Generates a list of the available serial ports and prompts the user to select one
fn get_serial_port() -> String {
    println!("\nAvailable ports: ");
    let ports = serialport::available_ports().expect("No ports found!");
    for p in 0..ports.len() {
        println!("[{}] {}", p, ports[p].port_name);
    }
    println!("\nEnter port number: ");
    let mut input_line = String::new();
    std::io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let port_index : usize = input_line.trim().parse().unwrap();
    println!("Selected port: {}\n", ports[port_index].port_name);
    let port_name = ports[port_index].port_name.clone();
    port_name
}


fn get_timecode() -> String {
    let date = Local::now();
    format!("[{}]", date.format("%Y-%m-%d %H:%M:%S"))
}

fn create_note_on(channel: u8, note: u8, velocity: u8) -> coremidi::PacketBuffer {
    println!("{} NOTE ON [Ch: {} | Pitch: {} | Vel: {}]", get_timecode(), channel + 1, note, velocity);
    let data = &[0x90 | (channel & 0x0f), note & 0x7f, velocity & 0x7f];
    coremidi::PacketBuffer::new(0, data)
}

fn create_note_off(channel: u8, note: u8, velocity: u8) -> coremidi::PacketBuffer {
    println!("{} NOTE OFF [Ch: {} | Pitch: {}]", get_timecode(), channel + 1, note);
    let data = &[0x80 | (channel & 0x0f), note & 0x7f, velocity & 0x7f];
    coremidi::PacketBuffer::new(0, data)
}

fn create_cc(channel: u8, cc: u8, value: u8) -> coremidi::PacketBuffer {
    println!("{} CONTROL CHANGE [Ch: {} | CC: {} | Value: {}]", get_timecode(), channel + 1, cc, value);
    let data = &[0xB0 | (channel & 0x0f), cc & 0x7f, value & 0x7f];
    coremidi::PacketBuffer::new(0, data)
}