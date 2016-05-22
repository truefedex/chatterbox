extern crate hound;
extern crate chatterbox;
#[macro_use] extern crate log;
extern crate simplelog;
extern crate getopts;
extern crate libc;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate router;
extern crate url;

mod output;
mod server;

use std::env;
use std::io;
use std::io::*;
use std::sync::Arc;
use chatterbox::*;
use simplelog::{TermLogger, LogLevelFilter};
use getopts::Options;
use output::*;

fn print_usage(program: &str, opts: Options) {
	const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let brief = format!("Chatterbox v{}\nUsage: {} TEXT [options]", VERSION, program);
    print!("{}", opts.usage(&brief));
}

fn interactive_loop(backend: Box<Backend>) -> io::Result<()> {
    let output = &mut BassChatterboxOutput::new();
    loop {
        print!(">");
        let mut input = String::new();
        try!(io::stdin().read_line(&mut input));
        if input.trim().is_empty() {
            break;
        }
        backend.synth(&input, output);
        output.flush();
    }
    Ok(())
}

fn run_interactive(backend: Box<Backend>) {    
    if let Some(err) = interactive_loop(backend).err() {
        println!("Oops! Error: {}", err);
    }    
}

fn main() {
	const DEFAULT_OUT_FILENAME :  &'static str = "output.wav";
    const DEFAULT_PATTERNS_PATH :  &'static str = "patterns/";
    const DEFAULT_MODE :  &'static str = "p";
	
	TermLogger::init(LogLevelFilter::Info/*Debug*/).unwrap();
	info!("Starting up...");
	
	let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "out", "set output file name (default is output.wav)", "NAME");
    opts.optopt("m", "mode", "set synth mode - (s)ynthetic or (p)atternbased (default is patternbased)", "MODE");
    opts.optflag("s", "server", "run in http server mode");
    opts.optflag("h", "help", "print this help menu");
    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }    
	
    let mode = matches.opt_str("m").unwrap_or(DEFAULT_MODE.to_string());    
    let backend: Box<Backend + Send + Sync> = 
        if mode == "s" {
            Box::new(backends::Synthetic)
        } else {
            Box::new(backends::PatternBased::from_patterns_path(DEFAULT_PATTERNS_PATH))
        };
        
    if matches.opt_present("s") {
		server::run(Arc::new(backend));
		return;
	}
    
	let input_text = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        run_interactive(backend);
        return;
    };
    
    let output_filename = matches.opt_str("o").unwrap_or(DEFAULT_OUT_FILENAME.to_string());
    let mut output = WavChatterboxOutput::new_for_file(&output_filename);
    backend.synth(&input_text, &mut output);
}
