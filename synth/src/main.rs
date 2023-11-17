use std::{
    f32::consts::PI,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    thread,
};

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use midir::MidiInput;
use midly::{live::LiveEvent, MidiMessage};

/// Length of each note in seconds
const NOTE_LENGTH: f32 = 0.250;
/// Length between each note in seconds
const NOTE_GAP: f32 = 0.05;
/// Maps tone index to frequency
/// (1865 used for error beep)
const NOTE_MAP: [u32; 10] = [165, 220, 247, 277, 294, 330, 370, 415, 440, 554];

fn main() -> Result<()> {
    let input = MidiInput::new("42synth input")?;
    let ports = input.ports();
    let port = ports.iter().next().ok_or("No ports found").unwrap();
    let port_name = input.port_name(&port)?;

    println!("[*] Opening connection with `{port_name}`");
    let synth = Arc::new(Synth::new());
    let _connection = input.connect(&port, "42synth", on_event, synth.clone())?;

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    println!("[*] Output device is `{}`", device.name().unwrap());

    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range
        .next()
        .unwrap()
        .with_max_sample_rate();
    let sample_rate = supported_config.sample_rate().0;
    let channels = supported_config.channels();
    synth.update_sample_rate(sample_rate);
    println!("[*] Sample rate: {}", sample_rate);

    let stream = device
        .build_output_stream(
            &supported_config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut queue = synth.queue.lock().unwrap();
                for sample in data.chunks_exact_mut(channels as usize) {
                    loop {
                        let next = match queue.iter_mut().next() {
                            Some(v) => v,
                            None => {
                                sample.iter_mut().for_each(|x| *x = 0.0);
                                break;
                            }
                        };

                        let val = match next.oscillator.next() {
                            Some(s) => s,
                            None => {
                                if next.playing {
                                    next.oscillator.reset();
                                    continue;
                                }

                                queue.remove(0);
                                continue;
                            }
                        } * synth.master_volume;
                        sample.iter_mut().for_each(|x| *x = val);
                        break;
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
    queue: Mutex<Vec<Voice>>,
    master_volume: f32,
    sample_rate: AtomicU32,
}

#[derive(Debug)]
struct Voice {
    key: u8,
    playing: bool,
    oscillator: Oscillator,
}

#[derive(Debug)]
struct Oscillator {
    i: u32,
    tone: f32,
    sample_rate: u32,
    duration: Option<u32>,
}

fn on_event(_time: u64, data: &[u8], synth: &mut Arc<Synth>) {
    let event = LiveEvent::parse(data).unwrap();
    match event {
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, .. } => synth.note_on(key.as_int()),
            MidiMessage::NoteOff { key, .. } => synth.note_off(key.as_int()),
            MidiMessage::Controller {
                controller,
                value: _,
            } => match controller.as_int() {
                // Stop all notes and clear queue
                123 | 122 => {
                    let mut queue = synth.queue.lock().unwrap();
                    queue.retain(|x| x.playing);
                    queue.iter_mut().for_each(|x| x.playing = false)
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
}

impl Synth {
    fn new() -> Self {
        Self {
            queue: Mutex::new(Vec::new()),
            master_volume: 1.0,
            sample_rate: AtomicU32::new(0),
        }
    }

    fn update_sample_rate(&self, sample_rate: u32) {
        self.sample_rate.store(sample_rate, Ordering::Relaxed);
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate.load(Ordering::Relaxed)
    }

    fn note_on(&self, key: u8) {
        let note = key.checked_sub(60).and_then(|x| NOTE_MAP.get(x as usize));
        let note = match note {
            Some(n) => *n,
            None => return,
        };

        let mut queue = self.queue.lock().unwrap();
        queue.push(Voice {
            key,
            playing: true,
            oscillator: Oscillator::new(note as f32, self.sample_rate()),
        });
    }

    fn note_off(&self, key: u8) {
        let mut voices = self.queue.lock().unwrap();
        let note = voices.iter_mut().rev().find(|v| v.key == key);
        if let Some(i) = note {
            i.playing = false;
        }
    }
}

impl Oscillator {
    fn new(tone: f32, sample_rate: u32) -> Self {
        Self {
            tone,
            i: 0,
            sample_rate: sample_rate,
            duration: Some((sample_rate as f32 * NOTE_LENGTH) as u32),
        }
    }

    fn reset(&mut self) {
        self.i = 0;
    }
}

impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i.saturating_add(1);

        const DELTA: f32 = 0.05;
        let signal = (self.i as f32 * self.tone * 2.0 * PI / self.sample_rate as f32).sin();
        let signal = (0.1 / (1.0 / DELTA).atan()) * (signal / DELTA).atan();

        match self.duration {
            Some(d) if self.i > (self.sample_rate as f32 * NOTE_GAP) as u32 + d => return None,
            Some(d) if self.i > d => return Some(0.0),
            _ => {}
        }

        Some(signal)
    }
}
