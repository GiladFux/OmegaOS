use spin::Mutex;
use crate::FileTable;
pub trait BlockDevice {
    const BLOCK_SIZE: usize = 512;
    fn read_block(&self, block_id: usize, data_size: usize, buf: &mut [u8]);
    fn write_block(&mut self, block_id: usize, buf: &[u8]);
    fn get_file_table(&self) -> &Mutex<FileTable>;

}
