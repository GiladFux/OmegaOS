use alloc::vec::Vec;
use omega::println;
use crate::fs::block_device::BlockDevice;
#[repr(C)]
pub struct FileEntry {
    pub name: [u8; 16],   
    pub blocks: Vec<usize>, // all blocks of the file by id
    pub size: usize,        // file size in bytes
    pub flags: u8,        
}

impl FileEntry {
    pub fn new(name: &str, start_block: usize) -> Self {
        let mut name_buf = [0u8; 16];
        let bytes = name.as_bytes();
        name_buf[..bytes.len()].copy_from_slice(bytes);

        Self {
            name: name_buf,
            blocks: Vec::from([start_block]), 
            size: 0,
            flags: 0,
        }
    }
}
pub struct FileTable {
    pub entries: Vec<FileEntry>,  
    pub available_blocks: Vec<usize>
}
impl FileTable {
    pub fn new(blocks_amount: usize) -> Self {
        FileTable {
            entries: Vec::new(),
            available_blocks: Vec::from_iter(1..=blocks_amount), // Initializes with numbers 1 to max block
        }
    }

    pub fn add_file(&mut self, filename: &str) {
        if let Some(start_block) = self.available_blocks.pop() { // Removes last element 
            let new_file = FileEntry::new(filename, start_block);
            self.entries.push(new_file);
        }
        else {
            println!("No available blocks for new file");
        }
    }
    

    pub fn find_file(&self, filename: &str) -> Option<&FileEntry> {
        for entry in &self.entries {
            let name = core::str::from_utf8(&entry.name).ok()?;
            if name == filename {
                return Some(entry);
            }
        }
        None
    }

    pub fn get_file_size(&self, filename: &str) -> usize {
        // Find the file entry in the table by the given filename
        if let Some(file_entry) = self.find_file(filename) {
            file_entry.size
        } else {
            0 
        }
    }
    pub fn delete_file_by_name<T: BlockDevice>(&mut self, device: &mut T, file_name: &str) {
        if let Some(index) = self.entries.iter().position(|entry| {
            let name_str = core::str::from_utf8(&entry.name).unwrap_or("").trim_end_matches('\0');
            name_str == file_name
        }) {
            // Get the blocks of the file
            let file_entry = &self.entries[index];
            let file_blocks = file_entry.blocks.clone(); // Clone to avoid borrowing issues
    
            // Overwrite all blocks with zeroes
            let empty_block = [0u8; 512]; // Assuming block size is 512 bytes
            for &block in &file_blocks {
                device.write_block(block, &empty_block);
            }
    
            // Remove the file entry from the table
            self.entries.remove(index);
    
            // Return freed blocks to the available list
            self.available_blocks.extend(file_blocks);
        }
    }    
}