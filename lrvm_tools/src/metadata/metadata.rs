use super::DeviceCategory;
use crate::bytes::bytes_to_words;

pub struct DeviceMetadata {
    pub hw_id: u64,
    pub size: u32,
    pub category: DeviceCategory,
    pub model: Option<u32>,
    pub data: Option<u64>,
}

impl DeviceMetadata {
    pub fn new(
        hw_id: u64,
        size: u32,
        category: DeviceCategory,
        model: Option<u32>,
        data: Option<u64>,
    ) -> Self {
        Self {
            hw_id,
            size,
            category,
            model,
            data,
        }
    }

    pub fn set_size(&mut self, new_size: u32) -> &mut Self {
        self.size = new_size;
        self
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0; 32];

        bytes[0..=7].copy_from_slice(&self.hw_id.to_be_bytes());
        bytes[8..=11].copy_from_slice(&self.size.to_be_bytes());
        bytes[12..=19].copy_from_slice(&self.category.encode().to_be_bytes());
        bytes[20..=23].copy_from_slice(&self.model.unwrap_or(0).to_be_bytes());
        bytes[24..=31].copy_from_slice(&self.data.unwrap_or(0).to_be_bytes());

        bytes
    }

    pub fn encode(&self) -> [u32; 8] {
        let mut words = [0; 8];
        words.copy_from_slice(&bytes_to_words(self.to_bytes()));
        words
    }
}
