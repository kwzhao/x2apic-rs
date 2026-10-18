use core::ptr::{self, NonNull};

#[derive(Debug)]
pub struct IoApicRegisters {
    ioregsel: NonNull<u32>,
    ioregwin: NonNull<u32>,
}

// SAFETY: `IoApic::new` gives this value sole ownership of the registers and
// requires their mapping to be valid in every address space it is used from.
unsafe impl Send for IoApicRegisters {}

// SAFETY: Every method that touches IOREGSEL/IOWIN takes `&mut self`. A shared
// reference cannot access the registers.
unsafe impl Sync for IoApicRegisters {}

impl IoApicRegisters {
    /// See `IoApic::new` for the safety contract.
    pub unsafe fn new(base_addr: u64) -> Self {
        IoApicRegisters {
            ioregsel: NonNull::new_unchecked(base_addr as *mut u32),
            ioregwin: NonNull::new_unchecked((base_addr + 0x10) as *mut u32),
        }
    }

    pub unsafe fn read(&mut self, selector: u32) -> u32 {
        ptr::write_volatile(self.ioregsel.as_ptr(), selector);
        ptr::read_volatile(self.ioregwin.as_ptr())
    }

    pub unsafe fn write(&mut self, selector: u32, value: u32) {
        ptr::write_volatile(self.ioregsel.as_ptr(), selector);
        ptr::write_volatile(self.ioregwin.as_ptr(), value);
    }

    pub unsafe fn set(&mut self, selector: u32, mask: u32) {
        ptr::write_volatile(self.ioregsel.as_ptr(), selector);

        let val = ptr::read_volatile(self.ioregwin.as_ptr());
        ptr::write_volatile(self.ioregwin.as_ptr(), val | mask);
    }

    pub unsafe fn clear(&mut self, selector: u32, mask: u32) {
        ptr::write_volatile(self.ioregsel.as_ptr(), selector);

        let val = ptr::read_volatile(self.ioregwin.as_ptr());
        ptr::write_volatile(self.ioregwin.as_ptr(), val & !mask);
    }
}

// Register selectors
pub const ID: u32 = 0x00;
pub const VERSION: u32 = 0x01;
pub const ARBITRATION: u32 = 0x02;
pub const TABLE_BASE: u32 = 0x10;
