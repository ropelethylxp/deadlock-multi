use std::ptr;

pub struct GameIntegration {
    pub process_id: u32,
}

impl GameIntegration {
    pub fn new(process_id: u32) -> Self {
        GameIntegration { process_id }
    }

    pub fn inject(&self) {
        // Injection logic here
    }

    pub fn read_memory(&self, address: usize) -> u32 {
        unsafe { ptr::read(address as *const u32) }
    }

    pub fn write_memory(&self, address: usize, value: u32) {
        unsafe { ptr::write(address as *mut u32, value) }
    }
}