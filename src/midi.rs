//! Midi send and receive helpers

use crate::messages;
use midir::{MidiInput, MidiOutput, MidiOutputPort, MidiInputPort, MidiIO, MidiOutputConnection};
use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use crate::music::note::Note;
use crate::music::chord::Chord;
use crate::messages::{Status};

/// Trait for sending Music struct to Midi
pub trait MidiSend {
    fn send_midi(&self, conn_out: &mut MidiOutputConnection, duration: u64, velocity: u8);
}

impl MidiSend for Note {
    fn send_midi(&self, conn_out: &mut MidiOutputConnection, duration: u64, velocity: u8) {
        let _ = conn_out.send(&[Status::NoteOn as u8, self.to_key_number(), velocity]);
        sleep(Duration::from_millis(duration));
        let _ = conn_out.send(&[Status::NoteOff as u8, self.to_key_number(), velocity]);
    }
}

impl MidiSend for Chord {
    fn send_midi(&self, conn_out: &mut MidiOutputConnection, duration: u64, velocity: u8) {
        for note in &self.notes {
            let _ = conn_out.send(&[0x90, note.to_key_number(), velocity]);
        }  
        sleep(Duration::from_millis(duration));
        for note in &self.notes {
            let _ = conn_out.send(&[0x80, note.to_key_number(), velocity]);
        }  
    }
}

/// Lists available input port devices
pub fn show_input_ports() {
    let midi_in = MidiInput::new("midi_in").expect("Could not open midi input.");
    for (i, p) in midi_in.ports().iter().enumerate() {
        println!("in ({}) : {}", i, midi_in.port_name(&p).unwrap());
    }
}

/// Lists available output port devices
pub fn show_output_ports() {
    let midi_out = MidiOutput::new("midi_out").expect("Could not open midi input.");
    for (i, p) in midi_out.ports().iter().enumerate() {
        println!("out ({}) : {}", i, midi_out.port_name(&p).unwrap());
    }
}

/// Finds port for a given string name
fn get_port_index_by_name<T: MidiIO>(midi_in: &T, name: String) -> Option<usize> {
    let mut port_index: Option<usize> = None;
    for (i, p) in midi_in.ports().iter().enumerate() {
        if midi_in.port_name(&p).unwrap().eq(&name) {
            port_index = Some(i);
            break;
        }
    }
    port_index
}

/// Midi stream send
pub fn send(port: String) {
    let midi_out = MidiOutput::new("midi_out").expect("Could not open midi output.");
    let input_ports = midi_out.ports();

    // Getting input device port
    let device_port: Option<&MidiOutputPort> = match get_port_index_by_name(&midi_out, port) {
        Some(i) => input_ports.get(i),
        None => None,
    };

    // Opening connection with input midi device
    let mut conn_out = midi_out.connect(device_port.unwrap(), "midir-test").unwrap();
    println!("Connection open. Listen!");

    // Tests
    Note::from_str("C4").unwrap().send_midi(&mut conn_out, 100, 127);
    Note::from_str("E4").unwrap().send_midi(&mut conn_out, 100, 127);
    Note::from_str("G4").unwrap().send_midi(&mut conn_out, 100, 127);
    Chord::from_str(vec!["C4", "E4", "G4", "B4"]).send_midi(&mut conn_out, 500, 127);
}

/// Midi stream receive and parse
pub fn receive(name: String) {
    let mut input = String::new();
    let midi_in = MidiInput::new("midi_in").expect("Could not open midi input.");
    let input_ports = midi_in.ports();

    // Getting input device port
    let device_port: Option<&MidiInputPort> = match get_port_index_by_name(&midi_in, name) {
        Some(i) => input_ports.get(i),
        None => None,
    };

    // Opening connection with input midi device
    let _conn_in = midi_in
        .connect(
            device_port.expect("Couldn't get device from name."),
            "midi_conn",
            move |stamp, message, _| {
                let raw_message = messages::Raw::new(stamp, message[0], message[1..].to_vec());
                let parsed: messages::Midi = raw_message.parse();
                println!("{}", parsed);
            },
            (),
        )
        .unwrap();

    println!("Press any key to terminate.");
    input.clear();
    stdin().read_line(&mut input).unwrap();
}
