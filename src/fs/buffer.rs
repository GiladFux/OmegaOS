use crate::fs::block_device::BlockDevice;
use crate::fs::file_table;

use super::file_table::FileTable;
pub struct MyBlockDevice {
    storage: &'static mut [u8],
    current_block_id: usize,
    files_table: FileTable
}

impl MyBlockDevice {
    pub fn new(storage: &'static mut [u8]) -> Self {
        let mut files_table = FileTable::new();
        Self { storage, current_block_id: 1, files_table}
    }
    pub fn get_cur_block_id(&self) -> usize
    {
        self.current_block_id
    }
    pub fn get_file_table(&self) -> FileTable
    {
        self.files_table
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

