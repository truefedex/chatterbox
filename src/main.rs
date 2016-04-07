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
        println!("Chatterbox v0.1.0\nИспользование:\n  chatterbox <текст>\nP.S.: понимает только кириллицу");
        return;
    }
    
    let input_text = &args[1];
    const OUT_FILENAME :  &'static str = "output.wav";
    println!("Рендерю: \"{}\" в {}...", input_text, OUT_FILENAME);
    
    let backend = &mut chatterbox::backends::Synthetic as &mut chatterbox::Backend;
    backend.synth(&input_text, &mut WavChatterboxOutput::new(OUT_FILENAME) as &mut chatterbox::Output);
}
