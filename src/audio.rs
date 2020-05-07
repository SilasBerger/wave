use std::f64::consts::PI;
use std::u16;


fn partial_sample(freq: f64, t: u64, track: &TrackSpec) -> f64 {
    let constant = 2.0 * PI;
    let variable = (t as f64 * freq) / track.sample_rate as f64;
    (variable * constant).sin()
}

fn wave_fragment(fragment: &FragmentSpec, track: &TrackSpec, ms_per_value: f64) -> Vec<u16> {
    let millis = ms_per_value * fragment.value as f64;
    let num_samples = (millis * track.sample_rate as f64) / 1000.0;
    let nsamples = num_samples.round() as usize;
    let mut buf = Vec::with_capacity(nsamples);
    let num_freqs = fragment.frequencies.len() as usize;
    if num_freqs == 0 {
        buf = vec![0u16; nsamples];
        return buf;
    }
    for t in 0..nsamples {
        let mut sample = 0.0;
        for i in 0..num_freqs {
            sample += partial_sample(fragment.frequencies[i], t as u64, track);
        }
        let amp_adjusted = sample * fragment.amplitude;
        let normalized = (amp_adjusted + num_freqs as f64) / (2.0 * num_freqs as f64);
        let scaled = (normalized * u16::MAX as f64).floor() as u16;
        buf.push(scaled);
    }
    buf
}

pub fn bounce(fragments: &[FragmentSpec], track: &TrackSpec) -> Vec<u16>{
    let ms_per_beat = 60_000.0 / track.bpm as f64;
    let values_per_beat = track.subdivision as f64 / 4.0;
    let ms_per_value = ms_per_beat / values_per_beat;
    let mut result = Vec::new();
    for fragment in fragments.iter() {
        result.append(&mut wave_fragment(fragment, track, ms_per_value));
    }
    result
}

pub struct FragmentSpec {
    value: u64,
    frequencies: Vec<f64>,
    amplitude: f64
}

impl FragmentSpec {
    pub fn chord(value: u64, frequencies: Vec<f64>, amplitude: f64) -> FragmentSpec {
        FragmentSpec {
            value,
            frequencies,
            amplitude
        }
    }

    pub fn note(value: u64, frequency: f64, amplitude: f64) -> FragmentSpec {
        FragmentSpec {
            value,
            frequencies: vec![frequency],
            amplitude
        }
    }

    pub fn rest(value: u64) -> FragmentSpec {
        FragmentSpec {
            value,
            frequencies: vec![],
            amplitude: 0f64
        }
    }
}

#[derive(Debug)]
pub struct TrackSpec {
    sample_rate: u16,
    bpm: u16,
    subdivision: u8,
    freq_a4: f64,
    volume: f64
}

impl TrackSpec {
    pub fn new(sample_rate: u16, bpm: u16, subdivision: u8, freq_a4: f64, volume: f64) -> TrackSpec {
        TrackSpec {
            sample_rate,
            bpm,
            subdivision,
            freq_a4,
            volume
        }
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn bpm(&self) -> u16 {
        self.bpm
    }

    pub fn subdivision(&self) -> u8 {
        self.subdivision
    }

    pub fn freq_a4(&self) -> f64 {
        self.freq_a4
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }
}