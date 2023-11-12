use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
    thread,
};

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use midir::MidiInput;
use midly::{live::LiveEvent, MidiMessage};

fn main() -> Result<()> {
    let input = MidiInput::new("42synth input")?;
    let ports = input.ports();
    let port = ports.iter().next().ok_or("No ports found").unwrap();
    let port_name = input.port_name(&port)?;

    println!("Opening connection with `{port_name}`");
    let synth = Arc::new(Synth::new());
    let _connection = input.connect(&port, "42synth", on_event, synth.clone())?;

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    println!("Default output device: {}", device.name().unwrap());

    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range
        .next()
        .unwrap()
        .with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0 as f32;

    let stream = device
        .build_output_stream(
            &supported_config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut voices = synth.voices.lock().unwrap();
                // dbg!(&voices);
                for sample in data.iter_mut() {
                    *sample = 0.0;
                    let mut i = 0;
                    while i < voices.len() {
                        let voice = &mut voices[i];
                        if let Some(s) = voice.oscillator.next(sample_rate) {
                            if voice.playing {
                                *sample += s;
                            }
                            i += 1;
                        } else {
                            voices.remove(i);
                        }
                    }
                }
            },
            move |err| eprintln!("an error occurred on output stream: {}", err),
            None,
        )
        .unwrap();
    stream.play()?;

    loop {
        thread::park()
    }
}

struct Synth {
    voices: Mutex<Vec<Voice>>,
}

#[derive(Debug)]
struct Voice {
    key: u8,
    playing: bool,
    oscillator: Oscillator,
}

#[derive(Debug)]
struct Oscillator {
    i: usize,
    tone: f32,
    duration: Option<usize>,
}

fn on_event(_time: u64, data: &[u8], synth: &mut Arc<Synth>) {
    let event = LiveEvent::parse(data).unwrap();
    match event {
        LiveEvent::Midi { channel, message } => match message {
            MidiMessage::NoteOn { key, .. } => {
                println!("hit note {} on channel {}", key, channel);
                synth.note_on(key.as_int());
            }
            MidiMessage::NoteOff { key, .. } => synth.note_off(key.as_int()),

            _ => {}
        },
        _ => {}
    }
}

impl Synth {
    fn new() -> Self {
        Self {
            voices: Mutex::new(Vec::new()),
        }
    }

    fn note_on(&self, key: u8) {
        const NOTE_MAP: [(u8, f32); 11] = [
            (76, 1000.0), // 9
            (75, 888.0),  // 8
            (74, 800.0),  // 7
            (73, 727.0),  // 6
            (71, 615.0),  // 5
            (69, 571.0),  // 4
            (68, 533.0),  // 3
            (67, 470.0),  // 2
            (66, 470.0),  // 2
            (64, 421.0),  // 1
            (62, 320.0),  // 0
        ];

        let note = match NOTE_MAP.iter().find(|(k, _)| *k == key) {
            Some((_, v)) => *v,
            None => return,
        };

        self.voices.lock().unwrap().push(Voice {
            key,
            playing: true,
            oscillator: Oscillator::new(note),
        });
    }

    fn note_off(&self, key: u8) {
        let mut voices = self.voices.lock().unwrap();
        let note = voices.iter_mut().rev().find(|v| v.key == key);
        if let Some(i) = note {
            i.playing = false;
        }
    }
}

impl Oscillator {
    fn new(tone: f32) -> Self {
        Self {
            i: 0,
            tone,
            duration: None,
        }
    }

    fn next(&mut self, sample_rate: f32) -> Option<f32> {
        self.i += 1;

        match self.duration {
            Some(d) if self.i > d => return None,
            _ => {}
        }

        Some((self.i as f32 * self.tone * 2.0 * PI / sample_rate).sin())
    }
}
