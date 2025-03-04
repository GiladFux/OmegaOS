use crate::fs::block_device::BlockDevice;
use crate::fs::superblock::Superblock;
use crate::fs::file_table::FileTable;
use crate::fs::file_table::FileEntry;

use super::buffer::MyBlockDevice;

pub fn format_fs<T: BlockDevice>(device: &mut T) {
    let superblock = Superblock::new(1024); // Example: 1024 blocks

    
    // Serialize the superblock
    let mut buffer = [0u8; 512];
    unsafe {
        core::ptr::copy_nonoverlapping(
            &superblock as *const _ as *const u8,
            buffer.as_mut_ptr(),
            core::mem::size_of::<Superblock>(),
        );
    }

    device.write_block(0, &buffer);


}


pub fn create_file(storage: MyBlockDevice, filename: &str, start_block: usize) {
    file_table.add_file(filename, storage);
}


pub fn write_file<T: BlockDevice>(device: &mut T, start_block: usize, data: &[u8]) { // write the data in the storage as bytes
    let mut buffer = [0u8; 512];
    buffer[..data.len()].copy_from_slice(data);

    device.write_block(start_block, &buffer);
}


pub fn read_file<T: BlockDevice>(device: &T, start_block: usize, buf: &mut [u8]) {
    device.read_block(start_block, buf);
}

pub fn delete_file<T: BlockDevice>(device: &mut T, file_table: &mut FileTable, file_block: usize) {
    file_table.delete_file_by_block(device, file_block);
}
