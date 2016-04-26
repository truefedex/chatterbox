extern crate hound;
extern crate chatterbox;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate getopts;

use std::env;
use chatterbox::*;
use hound::{ WavWriter, WavSpec };
use simplelog::{TermLogger, LogLevelFilter};
use getopts::Options;

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

fn print_usage(program: &str, opts: Options) {
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let brief = format!("Chatterbox v{}\nUsage: {} TEXT [options]", VERSION, program);
    print!("{}", opts.usage(&brief));
}

fn main() {
	const DEFAULT_OUT_FILENAME :  &'static str = "output.wav";
    const DEFAULT_PATTERNS_PATH :  &'static str = "patterns/";
    const DEFAULT_MODE :  &'static str = "p";
	
	TermLogger::init(LogLevelFilter::Debug).unwrap();
	info!("Starting up...");
	
	let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "out", "set output file name (default is output.wav)", "NAME");
    opts.optopt("m", "mode", "set synth mode - (s)ynthetic or (p)atternbased (default is patternbased)", "MODE");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    let output = matches.opt_str("o").unwrap_or(DEFAULT_OUT_FILENAME.to_string());
	
	let input_text = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    let mode = matches.opt_str("m").unwrap_or(DEFAULT_MODE.to_string());
    
    let backend: Box<Backend> = 
    if mode == "s" {
		Box::new(backends::Synthetic)
	} else {
		Box::new(backends::PatternBased::from_patterns_path(DEFAULT_PATTERNS_PATH))
	};
    
    backend.synth(&input_text, &mut WavChatterboxOutput::new(&output) as &mut Output);
}
