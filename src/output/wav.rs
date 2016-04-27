use std::io::*;
use std::fs;
use chatterbox::*;
use hound::{ WavWriter, WavSpec };

pub struct WavChatterboxOutput {
    writer: WavWriter<BufWriter<fs::File>>,
}

impl WavChatterboxOutput {
    pub fn new(file_name : &str) -> WavChatterboxOutput {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16
        };
        let writer : WavWriter<BufWriter<fs::File>> = WavWriter::create(file_name, spec).unwrap();
        WavChatterboxOutput { writer: writer, }
    }
}

impl Output for WavChatterboxOutput {
    fn write_sample(&mut self, sample: i16) {
        self.writer.write_sample(sample).unwrap();
    }       
}