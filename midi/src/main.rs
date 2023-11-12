use std::fs;

use midly::{Format, MetaMessage, MidiMessage, Smf, Timing, TrackEvent, TrackEventKind};

const MIDI_FILE: &str = r"V:\Downloads\Doja Cat - Say So.mid";

// Maps midi notes to 42s notes (0-9)
const NOTE_MAP: &[(u8, u8)] = &[(71, 5), (69, 4), (66, 2), (64, 1)];

fn main() {
    let raw = fs::read(MIDI_FILE).unwrap();
    let midi = Smf::parse(&raw).unwrap();

    assert_eq!(midi.header.format, Format::SingleTrack);
    let track = &midi.tracks[0];
    let ticks_per_quarter = match midi.header.timing {
        Timing::Metrical(e) => e.as_int(),
        Timing::Timecode(_, _) => todo!("Timecode timing not supported"),
    };

    let mut notes = Vec::new();
    let mut tempo = 0; // us per quarter note
    let mut time = 0;
    let mut min_note = u8::MAX;
    for event in track {
        match event.kind {
            TrackEventKind::Midi {
                channel: _,
                message: MidiMessage::NoteOn { key, vel: _ },
            } => {
                min_note = min_note.min(key.as_int());
                time += event.delta.as_int();
                notes.push(Note {
                    start: (time as f32 / ticks_per_quarter as f32 * 2.0),
                    length: 0.0,
                    note: key.as_int(),
                })
            }
            TrackEventKind::Midi {
                channel: _,
                message: MidiMessage::NoteOff { key, vel: _ },
            } => {
                time += event.delta.as_int();
                let mut note = notes.pop().unwrap();
                assert_eq!(note.note, key.as_int());
                note.length = (time as f32 / ticks_per_quarter as f32 * 2.0) - note.start;
                notes.push(note);
            }
            TrackEventKind::Meta(MetaMessage::Tempo(t)) => {
                tempo = t.as_int();
                println!("Tempo: {}", tempo);
            }
            _ => {}
        }
    }

    notes
        .iter_mut()
        .for_each(|n| n.note = NOTE_MAP.iter().find(|(a, _)| *a == n.note).unwrap().1);

    for note in &notes {
        println!(
            "[{: >2}]: {} {}",
            note.start.round(),
            note.note,
            note.length.round()
        );
    }

    notes.iter_mut().for_each(|n| {
        n.start = n.start.round();
        n.length = n.length.round();
    });

    let mut prg = Vec::new();
    let mut last = 0;
    for note in notes {
        if note.start as u32 > last {
            for _ in 0..note.start as u32 - last {
                prg.push("XEQ 00".to_string());
            }
        }

        prg.push(format!("@ {}", note.note));
        for _ in 0..note.length as u32 {
            prg.push(format!("TONE {}", note.note));
        }
        prg.push(format!("@ END"));
        last = note.start as u32 + note.length as u32;
    }

    const END: &str = "
LBL 00
2000000
LBL 01
1
-
X>0?
GTO 01
RTN";
    fs::write("out.free42", prg.join("\n") + END).unwrap();
}

#[derive(Debug)]
struct Note {
    // Start and end are time since beginning of song in 1/4 seconds
    start: f32,
    length: f32,
    // 0-9
    note: u8,
}
