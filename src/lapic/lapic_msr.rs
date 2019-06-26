#![allow(dead_code)]

use bit::BitIndex;
use bitflags::bitflags;
use core::ops::Range;
use paste;
use x86_64::registers::model_specific::Msr;

#[derive(Debug)]
pub struct LocalApicRegisters {
    base: Msr,
    id: Msr,
    version: Msr,
    tpr: Msr,
    ppr: Msr,
    eoi: Msr,
    ldr: Msr,
    sivr: Msr,
    isr0: Msr,
    isr1: Msr,
    isr2: Msr,
    isr3: Msr,
    isr4: Msr,
    isr5: Msr,
    isr6: Msr,
    isr7: Msr,
    tmr0: Msr,
    tmr1: Msr,
    tmr2: Msr,
    tmr3: Msr,
    tmr4: Msr,
    tmr5: Msr,
    tmr6: Msr,
    tmr7: Msr,
    irr0: Msr,
    irr1: Msr,
    irr2: Msr,
    irr3: Msr,
    irr4: Msr,
    irr5: Msr,
    irr6: Msr,
    irr7: Msr,
    error: Msr,
    icr: Msr,
    lvt_timer: Msr,
    lvt_thermal: Msr,
    lvt_perf: Msr,
    lvt_lint0: Msr,
    lvt_lint1: Msr,
    lvt_error: Msr,
    ticr: Msr,
    tccr: Msr,
    tdcr: Msr,
    self_ipi: Msr,
}

macro_rules! read {
    ($name:ident) => {
        paste::item! {
            pub unsafe fn $name(&self) -> u64 {
                self.$name.read()
            }

            pub unsafe fn [<$name _bit>](&self, bit: usize) -> bool {
                self.$name().bit(bit)
            }

            pub unsafe fn [<$name _bit_range>](
                &self,
                pos: Range<usize>,
            ) -> u64 {
                self.$name().bit_range(pos)
            }
        }
    };
}

macro_rules! write {
    ($name:ident) => {
        paste::item! {
            pub unsafe fn [<write_ $name>](&mut self, value: u64) {
                self.$name.write(value);
            }
        }
    };
}

macro_rules! read_write {
    ($name:ident) => {
        read!($name);
        write!($name);

        paste::item! {
            pub unsafe fn [<set_ $name _bit>](
                &mut self,
                bit: usize,
                val: bool,
            ) {
                let mut reg_val = self.$name();

                reg_val.set_bit(bit, val);

                self.[<write_ $name>](reg_val);
            }

            pub unsafe fn [<set_ $name _bit_range>](
                &mut self,
                pos: Range<usize>,
                val: u64,
            ) {
                let mut reg_val = self.$name();

                reg_val.set_bit_range(pos, val);

                self.[<write_ $name>](reg_val);
            }
        }
    };
}

impl LocalApicRegisters {
    pub fn new() -> Self {
        LocalApicRegisters {
            base: Msr::new(IA32_APIC_BASE),
            id: Msr::new(ID),
            version: Msr::new(VERSION),
            tpr: Msr::new(TPR),
            ppr: Msr::new(PPR),
            eoi: Msr::new(EOI),
            ldr: Msr::new(LDR),
            sivr: Msr::new(SIVR),
            isr0: Msr::new(ISR_0),
            isr1: Msr::new(ISR_1),
            isr2: Msr::new(ISR_2),
            isr3: Msr::new(ISR_3),
            isr4: Msr::new(ISR_4),
            isr5: Msr::new(ISR_5),
            isr6: Msr::new(ISR_6),
            isr7: Msr::new(ISR_7),
            tmr0: Msr::new(TMR_0),
            tmr1: Msr::new(TMR_1),
            tmr2: Msr::new(TMR_2),
            tmr3: Msr::new(TMR_3),
            tmr4: Msr::new(TMR_4),
            tmr5: Msr::new(TMR_5),
            tmr6: Msr::new(TMR_6),
            tmr7: Msr::new(TMR_7),
            irr0: Msr::new(IRR_0),
            irr1: Msr::new(IRR_1),
            irr2: Msr::new(IRR_2),
            irr3: Msr::new(IRR_3),
            irr4: Msr::new(IRR_4),
            irr5: Msr::new(IRR_5),
            irr6: Msr::new(IRR_6),
            irr7: Msr::new(IRR_7),
            error: Msr::new(ERROR),
            icr: Msr::new(ICR),
            lvt_timer: Msr::new(LVT_TIMER),
            lvt_thermal: Msr::new(LVT_THERMAL),
            lvt_perf: Msr::new(LVT_PERF),
            lvt_lint0: Msr::new(LVT_LINT0),
            lvt_lint1: Msr::new(LVT_LINT1),
            lvt_error: Msr::new(LVT_ERROR),
            ticr: Msr::new(TICR),
            tccr: Msr::new(TCCR),
            tdcr: Msr::new(TDCR),
            self_ipi: Msr::new(SELF_IPI),
        }
    }

    read_write!(base);
    read!(id);
    read!(version);
    read_write!(tpr);
    read!(ppr);
    write!(eoi);
    read_write!(ldr);
    read_write!(sivr);
    read!(isr0);
    read!(isr1);
    read!(isr2);
    read!(isr3);
    read!(isr4);
    read!(isr5);
    read!(isr6);
    read!(isr7);
    read!(tmr0);
    read!(tmr1);
    read!(tmr2);
    read!(tmr3);
    read!(tmr4);
    read!(tmr5);
    read!(tmr6);
    read!(tmr7);
    read!(irr0);
    read!(irr1);
    read!(irr2);
    read!(irr3);
    read!(irr4);
    read!(irr5);
    read!(irr6);
    read!(irr7);
    read!(error);
    read_write!(icr);
    read_write!(lvt_timer);
    read_write!(lvt_thermal);
    read_write!(lvt_perf);
    read_write!(lvt_lint0);
    read_write!(lvt_lint1);
    read_write!(lvt_error);
    read_write!(ticr);
    read!(tccr);
    read_write!(tdcr);
    write!(self_ipi);
}

bitflags! {
    /// Error flags in the APIC error status register.
    pub struct ErrorFlags: u8 {
        /// P6 and Pentium only.
        /// Local APIC detected a checksum error during send.
        const SEND_CHECKSUM_ERROR       = 0b0000_0001;
        /// P6 and Pentium only.
        /// Local APIC detected a checksum error during receive.
        const RECEIVE_CHECKSUM_ERROR    = 0b0000_0010;
        /// P6 and Pentium only.
        /// Local APIC sent a message that was not accepted by any APIC.
        const SEND_ACCEPT_ERROR         = 0b0000_0100;
        /// P6 and Pentium only.
        /// Local APIC received a message that was not accepted by any APIC.
        const RECEIVE_ACCEPT_ERROR      = 0b0000_1000;
        /// Local APIC does not support lowest-priority mode.
        const REDIRECTABLE_IPI          = 0b0001_0000;
        /// Local APIC detected an illegal interrupt vector (0-15) during send.
        const SEND_ILLEGAL_VECTOR       = 0b0010_0000;
        /// Local APIC detected an illegal interrupt vector (0-15) during
        /// receive.
        const RECEIVED_ILLEGAL_VECTOR   = 0b0100_0000;
        /// xAPIC mode only.
        /// Local APIC tried to access a reserved register.
        const ILLEGAL_REGISTER_ADDRESS  = 0b1000_0000;
    }
}

/// Local APIC timer modes.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum TimerMode {
    /// Timer only fires once.
    OneShot = 0b00,
    /// Timer fires periodically.
    Periodic = 0b01,
    /// Timer fires at an absolute time.
    TscDeadline = 0b10,
}

impl TimerMode {
    pub(super) fn into_u64(self) -> u64 {
        self as u64
    }
}

/// Local APIC timer divide configurations.
///
/// Defines the APIC timer frequency as the processor frequency divided by a
/// specified value.
#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum TimerDivide {
    /// Divide by 2.
    Div2 = 0b0000,
    /// Divide by 4.
    Div4 = 0b0001,
    /// Divide by 8.
    Div8 = 0b0010,
    /// Divide by 16.
    Div16 = 0b0011,
    /// Divide by 32.
    Div32 = 0b1000,
    /// Divide by 64.
    Div64 = 0b1001,
    /// Divide by 128.
    Div128 = 0b1010,
    /// Divide by 256.
    Div256 = 0b1011,
}

impl TimerDivide {
    pub(super) fn into_u64(self) -> u64 {
        self as u64
    }
}

/// Inter-processor interrupt destination mode.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum IpiDestMode {
    /// Physical destination mode.
    Physical = 0b0,
    /// Logical destination mode.    
    Logical = 0b1,
}

/// Inter-processor interrupt delivery modes.
#[derive(Debug)]
#[repr(u8)]
pub enum IpiDeliveryMode {
    /// Delivers to the processors specified in the vector field.
    Fixed = 0b000,
    /// Same as fixed, except interrupt is delivered to the processor with the
    /// lowest priority.
    LowestPriority = 0b001,
    /// Delivers a system management interrupt to the target processors.
    SystemManagement = 0b010,
    /// Delivers a non-maskable interrupt to the target processors.
    NonMaskable = 0b100,
    /// Delivers an INIT interrupt to the target processor(s).
    Init = 0b101,
    /// Delivers a start-up IPI to the target processor(s).
    StartUp = 0b110,
}

impl IpiDeliveryMode {
    pub(super) fn into_u64(self) -> u64 {
        self as u64
    }
}

/// Specifies the destination when calling `send_ipi_all`.
#[derive(Debug)]
#[repr(u8)]
pub enum IpiAllShorthand {
    /// Send inter-processor interrupt all processors.
    AllIncludingSelf = 0b10,
    /// Send inter-processor interrupt to all processor except this one.
    AllExcludingSelf = 0b11,
}

impl IpiAllShorthand {
    pub(super) fn into_u64(self) -> u64 {
        self as u64
    }
}

// MSR port addresses

pub const IA32_APIC_BASE: u32 = 0x1B;

pub const ID: u32 = 0x802;
pub const VERSION: u32 = 0x803;
pub const TPR: u32 = 0x808;
pub const PPR: u32 = 0x80A;
pub const EOI: u32 = 0x80B;
pub const LDR: u32 = 0x80D;
pub const SIVR: u32 = 0x80F;

pub const ISR_0: u32 = 0x810;
pub const ISR_1: u32 = 0x811;
pub const ISR_2: u32 = 0x812;
pub const ISR_3: u32 = 0x813;
pub const ISR_4: u32 = 0x814;
pub const ISR_5: u32 = 0x815;
pub const ISR_6: u32 = 0x816;
pub const ISR_7: u32 = 0x817;

pub const TMR_0: u32 = 0x818;
pub const TMR_1: u32 = 0x819;
pub const TMR_2: u32 = 0x81A;
pub const TMR_3: u32 = 0x81B;
pub const TMR_4: u32 = 0x81C;
pub const TMR_5: u32 = 0x81D;
pub const TMR_6: u32 = 0x81E;
pub const TMR_7: u32 = 0x81F;

pub const IRR_0: u32 = 0x820;
pub const IRR_1: u32 = 0x821;
pub const IRR_2: u32 = 0x822;
pub const IRR_3: u32 = 0x823;
pub const IRR_4: u32 = 0x824;
pub const IRR_5: u32 = 0x825;
pub const IRR_6: u32 = 0x826;
pub const IRR_7: u32 = 0x827;

pub const ERROR: u32 = 0x828;
pub const ICR: u32 = 0x830;

pub const LVT_TIMER: u32 = 0x832;
pub const LVT_THERMAL: u32 = 0x833;
pub const LVT_PERF: u32 = 0x834;
pub const LVT_LINT0: u32 = 0x835;
pub const LVT_LINT1: u32 = 0x836;
pub const LVT_ERROR: u32 = 0x837;

pub const TICR: u32 = 0x838;
pub const TCCR: u32 = 0x839;
pub const TDCR: u32 = 0x83E;

pub const SELF_IPI: u32 = 0x83F;

// Register bits and bit ranges.

pub const BASE_APIC_ENABLE: usize = 11;
pub const BASE_X2APIC_ENABLE: usize = 10;
pub const BASE_BSP: usize = 8;

pub const VERSION_NR: Range<usize> = 0..8;
pub const VERSION_MAX_LVT_ENTRY: Range<usize> = 16..24;
pub const VERSION_EOI_BCAST_SUPPRESSION: usize = 24;

pub const SIVR_EOI_BCAST_SUPPRESSION: usize = 12;
pub const SIVR_FOCUS_PROCESSOR_CHECKING: usize = 9;
pub const SIVR_APIC_SOFTWARE_ENABLE: usize = 8;
pub const SIVR_VECTOR: Range<usize> = 0..8;

pub const ICR_DESTINATION: Range<usize> = 32..64;
pub const ICR_DEST_SHORTHAND: Range<usize> = 18..20;
pub const ICR_TRIGGER_MODE: usize = 15;
pub const ICR_LEVEL: usize = 14;
pub const ICR_DESTINATION_MODE: usize = 11;
pub const ICR_DELIVERY_MODE: Range<usize> = 8..11;
pub const ICR_VECTOR: Range<usize> = 0..8;

pub const LVT_TIMER_MODE: Range<usize> = 17..19;
pub const LVT_TIMER_MASK: usize = 16;
pub const LVT_TIMER_VECTOR: Range<usize> = 0..8;

pub const LVT_ERROR_VECTOR: Range<usize> = 0..8;

pub const TDCR_DIVIDE_VALUE: Range<usize> = 0..4;
