mod oscillator;
mod note_frequencies;

use std::io;
use std::thread;
use crate::oscillator::run_oscillator;
use note_frequencies::{NoteFrequencies, Notes};

fn main() {
    println!("Please input your chord length");

    let mut chord_len_input = String::new();

    io::stdin()
        .read_line(&mut chord_len_input)
        .expect("Failed to read chord length");

    let chord_len: usize = chord_len_input.trim().parse().expect("Not a number.");
    let mut chord: Vec<f32> = vec![0.0; chord_len];

    let mut handles = vec![];

    for i in 0..chord_len {
        println!("Note {}", i+1);
        let freq: f32;
        let mut note_input = String::new();
        io::stdin()
            .read_line(&mut note_input)
            .expect("Failed to read frequency");

        let note_input = note_input.trim().to_lowercase();

        let frequencies = match note_input.as_str() {
            "c" => Some(&Notes::C.frequencies),
            "db" | "c#" => Some(&Notes::Db.frequencies),
            "d" => Some(&Notes::D.frequencies),
            "eb" | "d#" => Some(&Notes::Eb.frequencies),
            "e" => Some(&Notes::E.frequencies),
            "f" => Some(&Notes::F.frequencies),
            "gb" | "f#" => Some(&Notes::Gb.frequencies),
            "g" => Some(&Notes::G.frequencies),
            "ab" | "g#" => Some(&Notes::Ab.frequencies),
            "a" => Some(&Notes::A.frequencies),
            "bb" | "a#" => Some(&Notes::Bb.frequencies),
            "b" => Some(&Notes::B.frequencies),
            _ => None,
        };

        match frequencies {
            Some(freq) => chord.push(freq[4]),
            None => println!("Invalid note entered."),
        }
    }

    for f in chord {
        let handle = thread::spawn(move || {
            run_oscillator(f);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}
