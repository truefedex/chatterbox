#[macro_use]
extern crate log;
extern crate rustc_serialize;

use std::io;
use rustc_serialize::json;
use std::error;
use std::fmt;

pub mod backends;


pub trait Output {
    fn write_sample(&mut self, sample: i16);
}

pub trait Backend {
    fn synth(&self, input : &str, out: &mut Output);
}

#[derive(Debug)]
pub enum SynthError {
    Io(io::Error),
    PatternCollectionDecode(json::DecoderError),
}

impl fmt::Display for SynthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            SynthError::Io(ref err) => write!(f, "IO error: {}", err),
            SynthError::PatternCollectionDecode(ref err) => write!(f, "Error while parsing pattern collection: {}", err),
        }
    }
}

impl error::Error for SynthError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            SynthError::Io(ref err) => err.description(),
            SynthError::PatternCollectionDecode(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            SynthError::Io(ref err) => Some(err),
            SynthError::PatternCollectionDecode(ref err) => Some(err),
        }
    }
}
    

