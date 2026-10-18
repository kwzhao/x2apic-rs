/// See <https://wiki.osdev.org/APIC>
#[derive(Debug)]
pub enum LogicalDestinationMode {
    /// Every CPU is identified with a 1 bit out of 8 bits
    FlatModel,
    /// CPUs are grouped into clusters with up to 4 unique IDs within clusters
    ClusterModel,
}

impl LogicalDestinationMode {
    pub(crate) fn from_dfr(dfr_value: u32) -> Self {
        match dfr_value >> 28 {
            0b1111 => Self::FlatModel,
            0b0000 => Self::ClusterModel,
            _ => panic!("Unknown destination format register value"),
        }
    }
}
