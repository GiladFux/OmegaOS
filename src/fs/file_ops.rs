use crate::fs::block_device::BlockDevice;
use crate::fs::superblock::Superblock;
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
        
        // Find the file entry by exact name match
        let file_entry = file_table.entries.iter_mut()
            .find(|entry| {
                // Convert entry name to string and trim null bytes
                let entry_name = core::str::from_utf8(&entry.name)
                    .unwrap_or("")
                    .trim_end_matches('\0');
                
                // Compare exact names
                entry_name == file_name
            });
        
        // Update size and get block in one pass
        if let Some(entry) = file_entry {
            entry.size = data.len();
            entry.blocks.first().copied()
        } else {
            println!("Error: File '{}' not found", file_name);
            None
        }
    };

    // Once the lock is released, perform the write operation
    if let Some(block) = block {
        device.write_block(block, &buffer); 
    } else {
        println!("No block found for file: '{}'", file_name);
    }
}



pub fn read_file<T: BlockDevice>(device: &T, file_name: &str) -> Option<Vec<u8>> {     // TODO: make the function support more than 512 bytes by dividing into multiple blocks

    let file_table = device.get_file_table_immutable().lock();
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

pub fn delete_file<T: BlockDevice>(device: &mut T, file_name: &str) {
    let blocks_to_delete = {
        let file_table = device.get_file_table();
        let mut locked_table = file_table.lock();
        

        locked_table.find_and_remove_file(file_name)
    };
    
    if let Some(blocks) = blocks_to_delete {
        let empty_block = [0u8; 512]; 
        for &block in &blocks {
            device.write_block(block, &empty_block);
        }
    }
}