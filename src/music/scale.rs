//! Root note and a set of Intervals

use itertools::Itertools;
use std::fmt;

use crate::music::chord::Chord;
use crate::music::common::Interval;
use crate::music::common::Interval::*;
use crate::music::note::Note;

/// A scale consists in a root Note and a vector of Intervals
pub struct Scale {
    pub root: Note,
    pub intervals: Vec<Interval>,
}

impl Scale {
    /// Default constructor
    pub fn new(root: Note, intervals: Vec<Interval>) -> Self {
        Scale { root, intervals }
    }

    /// Major scale intervals
    pub const MAJOR: [Interval; 7] = [
        Unison,
        MajorSecond,
        MajorThird,
        Fourth,
        Fifth,
        MajorSixth,
        MajorSeventh,
    ];
    /// Minor (natural) scale intervals
    pub const MINOR: [Interval; 7] = [
        Unison,
        MajorSecond,
        MinorThird,
        Fourth,
        Fifth,
        MinorSixth,
        MinorSeventh,
    ];
    /// Minor (harmonic) scale intervals
    pub const MINOR_HARMONIC: [Interval; 7] = [
        Unison,
        MajorSecond,
        MinorThird,
        Fourth,
        Fifth,
        MinorSixth,
        MajorSeventh,
    ];

    /// Get major scale from root Note
    pub fn major(root: Note) -> Self {
        Self::new(root, Self::MAJOR.to_vec())
    }

    /// Get minor (natural) scale from root Note
    pub fn minor(root: Note) -> Self {
        Self::new(root, Self::MINOR.to_vec())
    }

    /// Get minor (harmonic) scale from root Note
    pub fn minor_harmonic(root: Note) -> Self {
        Self::new(root, Self::MINOR_HARMONIC.to_vec())
    }

    /// One chord built by thirds (if scale built by thirds).
    pub fn one(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(0, 2, len))
    }

    /// Two chord built by thirds (if scale built by thirds).
    pub fn two(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(1, 2, len))
    }

    /// Three chord built by thirds (if scale built by thirds).
    pub fn three(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(2, 2, len))
    }

    /// Four chord built by thirds (if scale built by thirds).
    pub fn four(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(3, 2, len))
    }

    /// Five chord built by thirds (if scale built by thirds).
    pub fn five(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(4, 2, len))
    }

    /// Six chord built by thirds (if scale built by thirds).
    pub fn six(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(5, 2, len))
    }

    /// Seven chord built by thirds (if scale built by thirds).
    pub fn seven(&self, len: usize) -> Chord {
        Chord::new(self.build_by_steps(6, 2, len))
    }

    fn build_by_steps(&self, root: usize, step: usize, length: usize) -> Vec<Note> {
        // We add all intervals of the scale one octave higher for chord creation
        let mut intervals_octave_up: Vec<Interval> = self
            .intervals
            .clone()
            .into_iter()
            .filter_map(|i| num::FromPrimitive::from_u32(i as u32 + 12))
            .collect_vec();

        // Concatenate all intervals
        let mut intervals = self.intervals.clone();
        intervals.append(&mut intervals_octave_up);

        // Step in interval vector
        intervals
            .clone()
            .into_iter()
            .cycle() // Wraps around if step / length combination exceeds two octave span
            .skip(root)
            .step_by(step)
            .map(|n| self.root + n)
            .take(length)
            .collect_vec()
    }

    /// Get Note vector from Scale
    pub fn notes(&self) -> Vec<Note> {
        self.intervals
            .clone()
            .into_iter()
            .map(|interval| self.root + interval)
            .collect_vec()
    }
}

/// Display trait for Scale
impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut notes: String = "".to_string();
        for (i, note) in (&self.notes()).iter().enumerate() {
            notes += &format!("{}", note).to_string();
            if i != self.notes().len() - 1 {
                notes += ","
            }
        }
        write!(f, "Scale({})", notes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::common::Letter;

    #[test]
    fn get_notes() {
        let root = Note::try_from("C0").unwrap();
        let intervals = vec![Unison, MajorSecond];
        let scale = Scale::new(root, intervals);
        let notes = scale.notes();
        assert_eq!(notes[0].letter, Letter::C);
        assert_eq!(notes[1].letter, Letter::D);
    }

    #[test]
    fn get_one() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let one_chord = major_scale.one(3);
        assert_eq!(one_chord.notes[0].letter, Letter::C);
        assert_eq!(one_chord.notes[1].letter, Letter::E);
        assert_eq!(one_chord.notes[2].letter, Letter::G);
    }

    #[test]
    fn get_two() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let two_chord = major_scale.two(3);
        assert_eq!(two_chord.notes[0].letter, Letter::D);
        assert_eq!(two_chord.notes[1].letter, Letter::F);
        assert_eq!(two_chord.notes[2].letter, Letter::A);
    }

    #[test]
    fn major() {
        let root = Note::try_from("C0").unwrap();
        let major_scale = Scale::major(root);
        let c_major_scale = [
            Letter::C,
            Letter::D,
            Letter::E,
            Letter::F,
            Letter::G,
            Letter::A,
            Letter::B,
        ];
        for i in 0..7 {
            assert_eq!(major_scale.notes()[i as usize].letter, c_major_scale[i]);
        }
    }

    #[test]
    fn minor() {
        let root = Note::try_from("A0").unwrap();
        let major_scale = Scale::minor(root);
        let a_minor_scale = [
            Letter::A,
            Letter::B,
            Letter::C,
            Letter::D,
            Letter::E,
            Letter::F,
            Letter::G,
        ];
        for i in 0..7 {
            assert_eq!(major_scale.notes()[i as usize].letter, a_minor_scale[i]);
        }
    }
}
