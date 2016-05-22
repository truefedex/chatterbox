use std::io::*;
use std::fs;
use chatterbox::*;
use hound::{ WavWriter, WavSpec };

pub struct WavChatterboxOutput <T> where T: Write + Seek {
    writer: WavWriter<T>,
}

impl<'b> WavChatterboxOutput<BufWriter<&'b mut Cursor<Vec<u8>>>> {    
    pub fn new_for_mem<'a>(buf: &'a mut Cursor<Vec<u8>>) -> WavChatterboxOutput<BufWriter<&'a mut Cursor<Vec<u8>>>> {
		let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16
        };
        let buf_writer = BufWriter::new(buf);        
        let writer = WavWriter::new(buf_writer, spec);
        WavChatterboxOutput { writer: writer, }
	}
}

impl WavChatterboxOutput<BufWriter<fs::File>> {
    pub fn new_for_file(file_name: &str) -> WavChatterboxOutput<BufWriter<fs::File>> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16
        };
        let writer = WavWriter::create(file_name, spec).unwrap();
        WavChatterboxOutput { writer: writer, }
    }
}

impl<T> Output for WavChatterboxOutput<T> where T: Write + Seek {
    fn write_sample(&mut self, sample: i16) {
        self.writer.write_sample(sample).unwrap();
    }       
}
