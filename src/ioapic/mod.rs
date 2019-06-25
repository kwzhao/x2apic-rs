//! The IOAPIC interface.

mod ioapic_regs;
use ioapic_regs::*;

mod irq_entry;
use irq_entry::*;
pub use irq_entry::{IrqFlags, IrqMode};

/// The IOAPIC structure.
#[derive(Debug)]
pub struct IoApic {
    regs: IoApicRegisters,
}

impl IoApic {
    /// Returns an IOAPIC with the given MMIO address `base_addr`.
    ///
    /// **The given MMIO address must already be mapped.**
    pub unsafe fn new(base_addr: u64) -> Self {
        IoApic {
            regs: IoApicRegisters::new(base_addr),
        }
    }

    /// Initialize the IOAPIC's redirection table entries with the given
    /// interrupt offset.
    ///
    /// Each entry `i` is redirected to `i + offset`.
    pub unsafe fn init(&mut self, offset: u8) {
        let end = self.max_table_entry() + 1;

        for i in 0..end {
            self.regs.set(irq_entry::lo(i), u32::from(i + offset));
            self.regs.write(irq_entry::hi(i), 0);
        }
    }

    /// Returns the IOAPIC ID.
    pub unsafe fn id(&mut self) -> u8 {
        ((self.regs.read(ID) >> 24) & 0xf) as u8
    }

    /// Sets the IOAPIC ID to `id`.
    pub unsafe fn set_id(&mut self, id: u8) {
        self.regs.write(ID, u32::from(id) << 24);
    }

    /// Returns the IOAPIC version.
    pub unsafe fn version(&mut self) -> u8 {
        (self.regs.read(VERSION) & 0xff) as u8
    }

    /// Returns the entry number (starting at zero) of the highest entry in the
    /// redirection table.
    pub unsafe fn max_table_entry(&mut self) -> u8 {
        ((self.regs.read(VERSION) >> 16) & 0xff) as u8
    }

    /// Returns the IOAPIC arbitration ID.
    pub unsafe fn arbitration_id(&mut self) -> u8 {
        ((self.regs.read(ARBITRATION) >> 24) & 0xf) as u8
    }

    /// Sets the IOAPIC arbitration ID to `id`.
    pub unsafe fn set_arbitration_id(&mut self, id: u8) {
        self.regs.write(ARBITRATION, u32::from(id) << 24);
    }

    /// Enable interrupt number `irq` on the CPUs specified by `dest`.
    pub unsafe fn enable_irq(
        &mut self,
        irq: u8,
        dest: u32,
        mode: IrqMode,
        options: IrqFlags,
    ) {
        let lo = irq_entry::lo(irq);
        let hi = irq_entry::hi(irq);

        self.regs.set(hi, dest << 24);

        self.regs.clear(lo, IRQ_MODE_MASK | IrqFlags::all().bits());
        self.regs.set(lo, mode.as_u32() | options.bits());

        self.regs.clear(lo, IRQ_MASK_BIT);
    }

    /// Disable interrupt number `irq`.
    pub unsafe fn disable_irq(&mut self, irq: u8) {
        self.regs.set(irq_entry::lo(irq), IRQ_MASK_BIT);
    }
}
