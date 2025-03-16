use crate::fs::block_device::BlockDevice;
use super::file_table::FileTable;
use omega::{print, println};
use spin::Mutex;
pub struct MyBlockDevice {
    storage: &'static mut [u8],
    files_table: Mutex<FileTable>, // Mutex to protect access to the file table
}
const BLOCKS_AMOUNT: usize = 1024;
impl MyBlockDevice {
    pub fn new(storage: &'static mut [u8]) -> Self {
        let files_table = FileTable::new(BLOCKS_AMOUNT);
        Self {
            storage,
            files_table: Mutex::new(files_table), // Initialize Mutex
        }
    }
    

    
}

impl BlockDevice for MyBlockDevice {
    fn read_block(&self, block_id: usize, data_size: usize, buf: &mut [u8]) {
        let start = block_id * Self::BLOCK_SIZE;
        let end = start + data_size;
        println!("before copy and slice");
        println!("start {} end {}", start, end);
        buf[..data_size].copy_from_slice(&self.storage[start..end]);

        println!("after copy and slice");

    }

    fn write_block(&mut self, block_id: usize, buf: &[u8]) {
        let start = block_id * Self::BLOCK_SIZE;
        let end = start + buf.len();  
        self.storage[start..end].copy_from_slice(buf);
    }
    
    fn get_file_table(&self) -> &Mutex<FileTable>
    {
        &self.files_table // Return a reference to the Mutex
    }

}

