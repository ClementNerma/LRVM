/// A single component mapping.
#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    /// Mapped component's ID
    pub aux_id: usize,
    /// Mapped component's hardware identifier
    pub aux_hw_id: u64,
    /// Mapping start address
    pub addr: u32,
    /// Mapping length
    pub size: u32,
}

impl Mapping {
    /// Get the end address of the mapping
    pub fn end_addr(&self) -> u32 {
        self.addr + self.size - 1
    }
}

/// Error that occurred during mapping
#[derive(Debug, Clone, Copy)]
pub enum MappingError {
    UnknownComponent,
    UnalignedStartAddress,
    UnalignedBusSize,
    UnalignedEndAddress,
    NullOrNegAddressRange,
    AlreadyMapped,
    NullBusSize,
    AddressOverlaps(Mapping),
    MappingTooLarge { aux_size: u32 },
}

/// Mapping range
#[derive(Debug)]
pub struct MappingRange {
    /// Start address
    pub start_addr: u32,
    /// End address
    pub end_addr: u32,
}

/// Status of a continguous mapping
#[derive(Debug)]
pub struct ContiguousMappingResult {
    /// Range of the mapping in case of success, or ID of the faulty components if the mapping failed
    pub mapping: Result<MappingRange, Vec<(usize, MappingError)>>,
    /// List of auxiliary components mapping (succeeded or failed)
    pub aux_mapping: Vec<AuxMappingStatus>,
}

/// Mapping status of a single auxiliary component
#[derive(Debug)]
pub struct AuxMappingStatus {
    /// Auxiliary component's ID
    pub aux_id: usize,
    /// Auxiliary component's hardware identifier
    pub aux_hw_id: u64,
    /// Auxiliary component's generic name
    pub aux_name: String,
    /// Mapping result
    pub aux_mapping: Result<MappingRange, MappingError>,
}
