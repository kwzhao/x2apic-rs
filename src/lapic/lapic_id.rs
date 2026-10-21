use super::LocalApicMode;

/// In xAPIC, a local APIC's id is stored in bits 31..24.
/// In x2APIC, a local APIC's id is stored as a normal u32.  
pub fn decode_lapic_id(raw_id: u32, mode: LocalApicMode) -> u32 {
    match mode {
        LocalApicMode::XApic { xapic_base: _ } => raw_id >> 24,
        LocalApicMode::X2Apic => raw_id,
    }
}

/// In xAPIC, a local APIC's id is stored in bits 31..24.
/// In x2APIC, a local APIC's id is stored as a normal u32.  
pub fn encode_lapic_id(actual_id: u32, mode: LocalApicMode) -> u32 {
    match mode {
        LocalApicMode::XApic { xapic_base: _ } => actual_id << 24,
        LocalApicMode::X2Apic => actual_id,
    }
}
