pub trait Output {
    fn write_sample(&mut self, sample: i16);
}

pub trait Backend {
    fn synth(&self, input : &str, out: &mut Output);
}
    
pub mod backends{
    use super::*;
    use std::f32::consts::PI;
    use std::i16;
    
    pub struct Synthetic {
        pub x: f64,
    }
    
    pub struct PatternBased {
        x: f64,
    }
    
    
    impl super::Backend for Synthetic {
        fn synth(&self, input : &str, out: &mut Output) {
            for _ in 0 .. 10 {
                for t in (0 .. 44100).map(|x| x as f32 / 44100.0) {
                    let sample = (t * 440.0 * 2.0 * PI).sin();
                    let amplitude = i16::MAX as f32;
                    out.write_sample((sample * amplitude) as i16);
                }
            }
        }       
    }
}
