#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub trait Output {
    fn write_sample(&mut self, sample: i16);
}

pub trait Backend {
    fn synth(&self, input : &str, out: &mut Output);
}
    
pub mod backends;
