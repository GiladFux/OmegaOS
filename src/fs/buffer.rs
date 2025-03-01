use crate::fs::block_device::BlockDevice;

pub struct MyBlockDevice {
    storage: &'static mut [u8],
}

impl MyBlockDevice {
    pub fn new(storage: &'static mut [u8]) -> Self {
        Self { storage}
    }
}

impl BlockDevice for MyBlockDevice {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) {
        let start = block_id * Self::BLOCK_SIZE;
        let end = start + Self::BLOCK_SIZE;
        buf.copy_from_slice(&self.storage[start..end]);
    }

    fn write_block(&mut self, block_id: usize, buf: &[u8]) {
        let start = block_id * Self::BLOCK_SIZE;
        let end = start + Self::BLOCK_SIZE;
        self.storage[start..end].copy_from_slice(buf);
        
    }
}
