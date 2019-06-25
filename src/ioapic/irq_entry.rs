use super::TABLE_BASE;
use bitflags::bitflags;

pub const IRQ_MASK_BIT: u32 = 0x0001_0000;
pub const IRQ_MODE_MASK: u32 = 0x0000_0700;

/// IOAPIC interrupt modes.
#[derive(Debug)]
#[repr(u32)]
pub enum IrqMode {
    /// Asserts the INTR signal on all allowed processors.
    Fixed = 0x0000_0000,
    /// Asserts the INTR signal on the lowest priority processor allowed.
    LowestPriority = 0x0000_0100,
    /// System management interrupt.
    /// Requires edge-triggering.
    SystemManagement = 0x0000_0200,
    /// Asserts the NMI signal on all allowed processors.
    /// Requires edge-triggering.
    NonMaskable = 0x0000_0400,
    /// Asserts the INIT signal on all allowed processors.
    /// Requires edge-triggering.
    Init = 0x0000_0500,
    /// Asserts the INTR signal as a signal that originated in an
    /// externally-connected interrupt controller.
    /// Requires edge-triggering.
    External = 0x0000_0700,
}

impl IrqMode {
    pub(super) fn as_u32(self) -> u32 {
        self as u32
    }
}

bitflags! {
    /// Redirection table entry flags.
    pub struct IrqFlags: u32 {
        /// Level-triggered interrupt (vs edge-triggered)
        const LEVEL_TRIGGERED = 0x0000_8000;
        /// Low-polarity interrupt signal (vs high-polarity)
        const LOW_ACTIVE = 0x0000_2000;
        /// Logical destination mode (vs physical)
        const LOGICAL_DEST = 0x0000_0800;
    }
}

// Gets the lower segment selector for `irq`
pub fn lo(irq: u8) -> u32 {
    TABLE_BASE + (2 * u32::from(irq))
}

// Gets the upper segment selector for `irq`
pub fn hi(irq: u8) -> u32 {
    lo(irq) + 1
}
