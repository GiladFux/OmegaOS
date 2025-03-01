use alloc::vec::Vec;
use omega::println;
use crate::fs::block_device::BlockDevice;
#[repr(C)]
pub struct FileEntry {
    pub name: [u8; 16],   // Filename (null-terminated)
    pub start_block: usize, // First block of file
    pub size: usize,        // File size in bytes
    pub flags: u8,        // File properties
}

impl FileEntry {
    pub fn new(name: &str, start_block: usize) -> Self {
        let mut name_buf = [0u8; 16];
        let bytes = name.as_bytes();
        name_buf[..bytes.len()].copy_from_slice(bytes);

        Self {
            name: name_buf,
            start_block,
            size: 0,
            flags: 0,
        }
    }
}

pub struct FileTable {
    pub entries: Vec<FileEntry>,  // A vector holding all file entries
}
impl FileTable {
    pub fn new() -> Self {
        FileTable {
            entries: Vec::new(),
        }
    }

    pub fn add_file(&mut self, filename: &str, start_block: usize) {
        let new_file = FileEntry::new(filename, start_block);
        self.entries.push(new_file);
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

    pub fn get_file_size(&self, filename: &str) -> Option<usize> {
        self.find_file(filename).map(|file| file.size)
    }
    pub fn delete_file_by_block<T: BlockDevice>(&mut self, device: &mut T, block_id: usize) {
        // Find the file entry by start_block
        if let Some(index) = self.entries.iter().position(|entry| entry.start_block == block_id) {
            let file_entry = self.entries.remove(index); // Remove file from table

            // Mark the block as empty (fill it with 0)
            let empty_block = [0u8; 512]; // Assuming block size is 512 bytes
            device.write_block(file_entry.start_block, &empty_block);

            println!("File '{}' removed, block {} marked as empty.", 
                core::str::from_utf8(&file_entry.name).unwrap_or("Unknown"), 
                file_entry.start_block);
        } else {
            println!("File with block ID {} not found.", block_id);
        }
    }
}