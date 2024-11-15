use crate::constants::{INPUT_BUF_SIZE, OUTPUT_BUF_SIZE};

#[derive(Clone, Copy)]
pub struct OutputPacket(pub [u8; OUTPUT_BUF_SIZE], pub usize);

impl OutputPacket {
    pub fn new() -> Self {
        Self([0; OUTPUT_BUF_SIZE], 0)
    }
}

#[derive(Clone, Copy)]
pub struct InputPacket(pub [u8; INPUT_BUF_SIZE], pub usize);

impl InputPacket {
    pub fn new() -> Self {
        Self([0; INPUT_BUF_SIZE], 0)
    }
}
