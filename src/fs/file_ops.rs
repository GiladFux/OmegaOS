use crate::fs::block_device::BlockDevice;
use crate::fs::superblock::Superblock;
use crate::fs::file_table::FileEntry;

pub fn format_fs<T: BlockDevice>(device: &mut T) {
    let superblock = Superblock::new(1024); // Example: 1024 blocks

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


pub fn create_file<T: BlockDevice>(device: &mut T, filename: &str, start_block: u32) {
    let file_entry = FileEntry::new(filename, start_block);

    let mut buffer = [0u8; 512];
    unsafe {
        core::ptr::copy_nonoverlapping(
            &file_entry as *const _ as *const u8,
            buffer.as_mut_ptr(),
            core::mem::size_of::<FileEntry>(),
        );
    }

    device.write_block(1, &buffer); // Store file entry in block 1
}

pub fn write_file<T: BlockDevice>(device: &mut T, start_block: usize, data: &[u8]) {
    let mut buffer = [0u8; 512];
    buffer[..data.len()].copy_from_slice(data);

    device.write_block(start_block, &buffer);
}


pub fn read_file<T: BlockDevice>(device: &T, start_block: usize, buf: &mut [u8]) {
    device.read_block(start_block, buf);
}

pub fn delete_file<T: BlockDevice>(device: &mut T, file_block: usize) {
    let empty = [0u8; 512];
    device.write_block(file_block, &empty);
}
