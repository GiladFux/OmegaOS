use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::gdt::SegmentSelector;
use lazy_static::lazy_static;

// Define the index for the Interrupt Stack Table (IST) used for double faults
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    // Lazy static initialization of the Task State Segment (TSS)
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new(); // Create a new, empty TSS
        // Create a stack for handling double faults and assign it to the IST[0]
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5; // Define stack size (5 pages)
            // Define the static stack in memory
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            // Get the start and end addresses of the stack
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK }); // Convert stack pointer to virtual address
            let stack_end = stack_start + STACK_SIZE; // End of the stack
            stack_end // Return the stack's end address
        };
        tss // Return the initialized TSS
    };
}

lazy_static! {
    // Lazy static initialization of the Global Descriptor Table (GDT)
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new(); // Create a new GDT
        // Add a code segment entry to the GDT for kernel code
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        // Add a TSS segment entry to the GDT for the TSS
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        // Return the GDT and its selectors
        (gdt, Selectors { code_selector, tss_selector })
    };
}

// Struct to store segment selectors for the GDT
struct Selectors {
    code_selector: SegmentSelector, // Selector for the code segment
    tss_selector: SegmentSelector,  // Selector for the TSS segment
}

// Function to initialize the GDT and load the segment selectors
pub fn init() {
    use x86_64::instructions::tables::load_tss; // Import function to load the TSS
    use x86_64::instructions::segmentation::{CS, Segment}; // Import functions for segmentation

    GDT.0.load(); // Load the GDT into the CPU
    unsafe {
        // Set the Code Segment (CS) register to the kernel code segment
        CS::set_reg(GDT.1.code_selector);
        // Load the TSS (Task State Segment) using the TSS selector
        load_tss(GDT.1.tss_selector);
    }
}
