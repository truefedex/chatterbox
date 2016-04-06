extern crate hound;
extern crate chatterbox;

use std::env;
use hound::{ WavWriter, WavSpec };

struct WavChatterboxOutput {
    writer: WavWriter<std::io::BufWriter<std::fs::File>>,
}

impl WavChatterboxOutput {
    fn new(file_name : &str) -> WavChatterboxOutput {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16
        };
        let writer : WavWriter<std::io::BufWriter<std::fs::File>> = WavWriter::create(file_name, spec).unwrap();
        WavChatterboxOutput { writer: writer, }
    }
}

impl chatterbox::Output for WavChatterboxOutput {
    fn write_sample(&mut self, sample: i16) {
        self.writer.write_sample(sample).unwrap();
    }       
}

fn main() { 
    let args: Vec<_> = env::args().collect();
    if args.len() <= 1 {
        println!("Chatterbox v 0.1.0\nИспользование:\n  chatterbox <текст>");
        return;
    } else {
        println!("The first argument is {}", args[1]);
    }
    
    let backend : &mut chatterbox::Backend = &mut chatterbox::backends::Synthetic{ x: 0.0 } as &mut chatterbox::Backend;
    backend.synth("test", &mut WavChatterboxOutput::new("output.wav") as &mut chatterbox::Output);
}
