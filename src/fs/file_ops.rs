use crate::fs::block_device::BlockDevice;
use crate::fs::superblock::Superblock;
use crate::fs::file_table::FileTable;
use omega::println;
use super::buffer::MyBlockDevice;
use alloc::vec::Vec;

pub fn format_fs<T: BlockDevice>(device: &mut T) {
    let superblock = Superblock::new(1024); 

    
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


pub fn create_file(device: &mut MyBlockDevice, filename: &str) {
    // Lock the file_table to prevent race conditions
    {
    let mut files_table = device.get_file_table().lock();
    // Add the file entry to the file table
    files_table.add_file(filename);
    }
}

pub fn write_file<T: BlockDevice>(device: &mut T, file_name: &str, data: &[u8]) { 
    // TODO: make the function support more than 512 bytes by dividing into multiple blocks
    let mut buffer = [0u8; 512];
    buffer[..data.len()].copy_from_slice(data);

    // Lock the file table to find the file's blocks
    let block = {
        let mut file_table = device.get_file_table().lock(); // Lock file table
        if let Some(entry) = file_table.entries.iter_mut()
            .find(|entry| entry.name.starts_with(file_name.as_bytes()))
        {
            // Update the size of the file entry to the new data length
            entry.size = data.len();
        }
        file_table.entries.iter()
            .find(|entry| entry.name.starts_with(file_name.as_bytes()))
            .and_then(|entry| entry.blocks.first().copied()) // Extract block number
    };

    // Once the lock is released, perform the write operation
    if let Some(block) = block {
        device.write_block(block, &buffer); 
    }
}




pub fn read_file<T: BlockDevice>(device: &T, file_name: &str) -> Option<Vec<u8>> {
    let file_table = device.get_file_table().lock();
    if let Some(file_entry) = file_table.find_file(file_name) {
        let size = file_entry.size;

        let mut buffer = Vec::with_capacity(size); 
        unsafe { buffer.set_len(size); } 

        device.read_block(file_entry.blocks[0], size, &mut buffer);

        Some(buffer)
    } else {
        println!("file not found!");
        None
    }
}

pub fn delete_file<T: BlockDevice>(device: &mut T, file_table: &mut FileTable, file_name: &str) {
    file_table.delete_file_by_name(device, file_name);
}
