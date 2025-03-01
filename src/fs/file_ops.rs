use crate::fs::block_device::BlockDevice;
use crate::fs::superblock::Superblock;
use crate::fs::file_table::FileTable;


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


pub fn create_file<T: BlockDevice>(file_table: &mut FileTable, filename: &str, start_block: usize) {
    file_table.add_file(filename, start_block);
}

pub fn write_file<T: BlockDevice>(device: &mut T, start_block: usize, data: &[u8]) {
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
