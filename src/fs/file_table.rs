#[repr(C)]
pub struct FileEntry {
    pub name: [u8; 16],   // Filename (null-terminated)
    pub start_block: u32, // First block of file
    pub size: u32,        // File size in bytes
    pub flags: u8,        // File properties
}

impl FileEntry {
    pub fn new(name: &str, start_block: u32) -> Self {
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
