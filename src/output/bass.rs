use libc::*;
use chatterbox::*;

pub struct BassChatterboxOutput {
    stream: HSTREAM,
    buffer: Vec<i16>,
}

impl BassChatterboxOutput {
    pub fn new() -> BassChatterboxOutput {
        let mut res = unsafe { BASS_Init(-1, 44100, 0, 0, 0) };
        debug!("Bass inited(?) - {}", res == 1);
        let stream = unsafe { BASS_StreamCreate(44100, 1, 0, STREAMPROC_PUSH, 0) };
        debug!("Bass stream created - {}", stream);
        res = unsafe { BASS_ChannelPlay(stream, 0) };
        debug!("Bass stream play - {}", res);
        BassChatterboxOutput { stream: stream, buffer: Vec::new()}
    }
    
    pub fn flush(&mut self) {
        if self.buffer.is_empty() {
            return;
        }
        let result = unsafe { BASS_StreamPutData(self.stream, self.buffer.as_mut_ptr(), (self.buffer.len() * 2) as uint32_t) };
        debug!("Bass data flushed - {}", result);
        self.buffer.clear();
    }
}

impl Output for BassChatterboxOutput {
    fn write_sample(&mut self, sample: i16) {
        self.buffer.push(sample);
    }       
}

impl Drop for BassChatterboxOutput {
    fn drop(&mut self) {
        unsafe { BASS_StreamFree(self.stream) };
        unsafe { BASS_Free() };
    }
}


pub type HSTREAM = uint32_t;

pub const STREAMPROC_PUSH :  intptr_t = -1;

#[link(name = "bass")]
extern {
    pub fn BASS_Init(device: int64_t, freq: uint32_t, flags: uint32_t, win: uint64_t, clsid: intptr_t) -> uint8_t;
    pub fn BASS_Free() -> uint8_t;
    pub fn BASS_StreamCreate(freq: uint32_t, chans: uint32_t, flags: uint32_t, proc_: intptr_t, user: intptr_t) -> HSTREAM;
    pub fn BASS_StreamFree(handle: HSTREAM) -> uint8_t;
    pub fn BASS_StreamPutData(handle: HSTREAM, buffer: *const i16, length: uint32_t) -> uint32_t;
    pub fn BASS_ChannelPlay(handle: uint32_t, restart: uint8_t) -> uint8_t;
}
