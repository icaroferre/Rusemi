# Rusemi - Rust-based USB Serial to CoreMIDI CLI

Rusemi is a lightweight command line utility written in Rust and designed to receive MIDI-formatted messages from a USB serial port and forward them to a virtual MIDI port via CoreMIDI.  
It was primarily created to help debug the development of hardware MIDI devices based on microcontrollers (such as Arduinos, STM32, etc) without the use of a class-compliant USB MIDI bootloader / firmware.  
Rusemi can be used as a replacement for other USB Serial to MIDI convertes such as [Hairless MIDI](https://github.com/projectgus/hairless-midiserial).

# How to use

1 - Download the latest compiled version from the Releases page or clone the repo and compile it (```cargo build```)
2 - Run the compiled binary via the terminal
3 - Select which serial port you'd like to use, and specify baud rate: ```./rusemi 38400``` (Defaults to 32500, the MIDI baud rate)
4 - Messages received from the serial port (incoming messages) will be parsed, printed to the terminal and forwarded to the "from Rusemi" virtual MIDI port. 
5 - Messages sent to the "to Rusemi" virtual MIDI port (outgoing messages) will be forwarded to the serial port.

# Limitations

Rusemi only works on macOS (since it uses CoreMIDI).

Rusemi currenctly can only parse MIDI notes and control messages received from the USB serial port (messages sent to the serial port don't require parsing and therefore it can handle every MIDI message).

# Roadmap

☑️ Improve the parser to support more types of MIDI messages  
☑️ Add support for selecting the serial port via arguments  

-----

Created by Icaro Ferre
[@icaroferre](https://instagram.com/icaroferre)  
[icaroferre.com](https://icaroferre.com)
