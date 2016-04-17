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
		const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("Chatterbox v{}\nUsage:\n  chatterbox <текст>\nP.S.: currently understand only cyrillic", VERSION);
        return;
    }
    
    let input_text = &args[1];
    const DEFAULT_OUT_FILENAME :  &'static str = "output.wav";
    const DEFAULT_PATTERNS_PATH :  &'static str = "patterns/";
    
    //let backend = &mut chatterbox::backends::Synthetic as &mut chatterbox::Backend;
    let backend = &mut chatterbox::backends::PatternBased::from_patterns_path(DEFAULT_PATTERNS_PATH) as &mut chatterbox::Backend;
    
    println!("Рендерю: \"{}\" в {}...", input_text, DEFAULT_OUT_FILENAME);
    backend.synth(&input_text, &mut WavChatterboxOutput::new(DEFAULT_OUT_FILENAME) as &mut chatterbox::Output);
}
