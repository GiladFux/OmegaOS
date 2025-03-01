#[repr(C)]
pub struct Superblock {
    pub magic: u32,       // Unique identifier
    pub block_count: u32, // Total blocks
    pub free_blocks: u32, // Number of free blocks
}

impl Superblock {
    pub fn new(block_count: u32) -> Self {
        Self {
            magic: 0x6969, // Example magic number
            block_count,
            free_blocks: block_count - 2, // Reserve space for metadata
        }
    }
}
