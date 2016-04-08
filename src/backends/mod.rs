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
            'Б' | 'б' | 'П' | 'п' => for (t, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
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
                        let sample_a = (t * 494.0 * 2.0 * PI).sin();
                        let sample_b = (t * 584.0 * 2.0 * PI).sin();
                        let sample_c = (t * 372.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Ё' | 'ё' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 394.0 * 2.0 * PI).sin();
                        let sample_b = (t * 384.0 * 2.0 * PI).sin();
                        let sample_c = (t * 472.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Ж' | 'ж' => for t in (0 .. 4000).map(|x| x as f32 / 44100.0) {
                        let sample_a = if (t * 50.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_b = rand::thread_rng().next_f32();
                        let sample_c = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'З' | 'з' => for t in (0 .. 4000).map(|x| x as f32 / 44100.0) {
                        let sample_a = if (t * 50.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_b = if (t * 55.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_c = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'И' | 'и' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 1300.0 * 2.0 * PI).sin();
                        let sample_b = (t * 1400.0 * 2.0 * PI).sin();
                        let sample_c = (t * 372.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Й' | 'й' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = if (t * 50.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_b = (t * 1400.0 * 2.0 * PI).sin();
                        let sample_c = (t * 372.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'К' | 'к' => for (t, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
                        let sample_a = if (t * 20.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a * j * amplitude) as i16);
                    },
            'Л' | 'л' | 'М' | 'м' | 'Н' | 'н' => for (_, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
                        let sample_a = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a * j * amplitude) as i16);
                    },
            'О' | 'о' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 394.0 * 2.0 * PI).sin();
                        let sample_b = (t * 384.0 * 2.0 * PI).sin();
                        let sample_c = (t * 472.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Р' | 'р' => for t in (0 .. 4000).map(|x| x as f32 / 44100.0) {
                        let sample_a = if (t * 7.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a * amplitude) as i16);
                    },
            'С' | 'с' => for _ in (0 .. 4000).map(|x| x as f32 / 44100.0) {
                        let sample_a = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a * amplitude) as i16);
                    },
            'Т' | 'т' => for (t, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
                        let sample_a = if (t * 20.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_b = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b) / 2.0;
                        out.write_sample((normalized_sample * j * amplitude) as i16);
                    },
            'У' | 'у' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 694.0 * 2.0 * PI).sin();
                        let sample_b = (t * 684.0 * 2.0 * PI).sin();
                        let sample_c = (t * 572.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Ф' | 'ф' | 'Х' | 'х' => for _ in (0 .. 4000).map(|x| x as f32 / 44100.0) {
                        let sample_a = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a / 2.0 * amplitude) as i16);
                    },
            'Ц' | 'ц' | 'Ч' | 'ч' | 'Ш' | 'ш' | 'Щ' | 'щ' => for (_, j) in (0 .. 4000).map(|x| (x as f32 / 44100.0, 1.0 - (x as f32 / 4000.0))) {
                        let sample_a = rand::thread_rng().next_f32();
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a * j * amplitude) as i16);
                    },
            'Ъ' | 'ъ' | 'Ь' | 'ь' => (),
            'Ы' | 'ы' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 1200.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a / 2.0 * amplitude) as i16);
                    },
            'Э' | 'э' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = (t * 1100.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        out.write_sample((sample_a / 2.0 * amplitude) as i16);
                    },
            'Ю' | 'ю' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = if (t * 50.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_b = (t * 684.0 * 2.0 * PI).sin();
                        let sample_c = (t * 572.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b + sample_c) / 3.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            'Я' | 'я' => for t in (0 .. 10000).map(|x| x as f32 / 44100.0) {
                        let sample_a = if (t * 50.0 * 2.0 * PI).sin() > 0.0 {1.0} else {-1.0};
                        let sample_b = (t * 1153.0 * 2.0 * PI).sin();
                        let amplitude = i16::MAX as f32;
                        let normalized_sample = (sample_a + sample_b) / 2.0;
                        out.write_sample((normalized_sample * amplitude) as i16);
                    },
            ',' => for _ in 0 .. 22050 {//0.5 sec of silence
                    out.write_sample(0);
                },
            '.' => for _ in 0 .. 32000 {//~0.75 sec of silence
                    out.write_sample(0);
                },
            _ => for _ in 0 .. 5000 {//~0.2 sec of silence
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
