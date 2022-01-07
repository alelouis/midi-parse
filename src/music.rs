use crate::messages::Data;
use std::fmt;

pub const KEYBOARD: [Letter; 12] = [
    Letter::C,
    Letter::Db,
    Letter::D,
    Letter::Eb,
    Letter::E,
    Letter::F,
    Letter::Gb,
    Letter::G,
    Letter::Ab,
    Letter::A,
    Letter::Bb,
    Letter::B
];

// Note abstraction with letter and octave
#[derive(Debug)]
pub struct Note {
    pub letter: Letter,
    pub octave: u8,
}

// Chord abstraction
#[derive(Debug)]
pub struct Chord {
    pub notes: Vec<Note>,
}

// Name of a note
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Letter {
    A,
    Bb,
    B,
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
}

impl Note {
    // Find letter and octave from midi key number (Data::KeyNumber)
    pub fn new(letter: Letter, octave: u8) -> Self {
        Note { letter, octave }
    }

    pub fn from_str(s : &str) -> Option<Self> {
        let letter_str: &str = &s[0..s.len()-1];
        let octave_str: &str = &s[s.len()-1..];
        let letter: Option<Letter> = match letter_str {
            "C"  => Some(Letter::C),
            "Db" => Some(Letter::Db),
            "D"  => Some(Letter::D),
            "Eb" => Some(Letter::Eb),
            "E"  => Some(Letter::E),
            "F"  => Some(Letter::F),
            "Gb" => Some(Letter::Gb),
            "G"  => Some(Letter::G),
            "Ab" => Some(Letter::Ab),
            "A"  => Some(Letter::A),
            "Bb" => Some(Letter::Bb),
            "B"  => Some(Letter::B),
            _ => None
        };
        match letter {
            Some(l) => Some(Note::new(l, octave_str.parse::<u8>().unwrap())),
            None => None
        }
    }

    pub fn from_key_number(kn: &Data) -> Option<Self> {
        match kn {
            Data::KeyNumber(x) => {
                let index: usize = ((x - 21) % 12) as usize;
                Some(Note {
                    letter: KEYBOARD[index],
                    octave: (x - 21) / 12,
                })
            }
            _ => None,
        }
    }

    // Compute distance in semitones between two notes
    pub fn dist_to(&self, other: &Note) -> u8 {
        let octave_difference: i8 = self.octave as i8 - other.octave as i8;
        let self_index: i8 = KEYBOARD.iter().position(|&x| x == self.letter).unwrap() as i8;
        let other_index: i8 = KEYBOARD.iter().position(|&x| x == other.letter).unwrap() as i8;
        (self_index - other_index + octave_difference * 12).abs().try_into().unwrap()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{}", self.letter, self.octave)
    }
}

impl Chord {
    pub fn new(notes: Vec<Note>) -> Self {
        Chord { notes }
    }

    pub fn from_str(notes: Vec<&str>) -> Self {
        Chord::new(notes.iter().map(|note| Note::from_str(note).unwrap()).collect())
    }
}
