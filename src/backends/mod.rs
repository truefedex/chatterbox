extern crate rand;

use super::*;
use std::f32::consts::PI;
use std::i16;
use self::rand::Rng;

pub struct Synthetic;

pub struct PatternBased;


impl Synthetic {
    fn synth_char(&self, chr : char, out: &mut Output) {
        match chr {
            'А' | 'а' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 1268.0 * 2.0 * PI).sin();
                        let sample_b = (t * 1153.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b) / 2.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Б' | 'б' => for (t, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
                        let sample_a = (t * 300.0 * 2.0 * PI).sin();
                        let sample_b = (t * 325.0 * 2.0 * PI).sin();
                        let sample_c = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * j * amplitude) as i16);
                    },
            'В' | 'в' => for t in (0 .. 4000).map(|x| (x as f32 / 44100.0)) {
                        let sample_a = (t * 300.0 * 2.0 * PI).sin();
                        let sample_b = (t * 325.0 * 2.0 * PI).sin();
                        let sample_c = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Г' | 'г' => for t in (0 .. 4000).map(|x| (x as f32 / 44100.0)) {
                        let sample_a = (t * 400.0 * 2.0 * PI).sin();
                        let sample_b = (t * 425.0 * 2.0 * PI).sin();
                        let sample_c = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Д' | 'д' => for (t, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
                        let sample_a = (t * 500.0 * 2.0 * PI).sin();
                        let sample_b = (t * 525.0 * 2.0 * PI).sin();
                        let sample_c = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * j * amplitude) as i16);
                    },
            'Е' | 'е' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 394.0 * 2.0 * PI).sin();
                        let sample_b = (t * 384.0 * 2.0 * PI).sin();
                        let sample_c = (t * 372.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            /*'Ё' | 'ё' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 394.0 * 2.0 * PI).sin();
                        let sample_b = (t * 384.0 * 2.0 * PI).sin();
                        let sample_c = (t * 372.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },*/
            _ => for _ in 0 .. 44100 {//1 sec of silence
                    out.write_sample(0);
                },
        }        
    }
}

impl super::Backend for Synthetic {
    fn synth(&self, input : &str, out: &mut Output) {
        for chr in input.chars() {
            self.synth_char(chr, out)
        }
    }       
}
