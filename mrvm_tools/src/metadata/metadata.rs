use crate::bytes::bytes_to_words;
use super::DeviceCategory;

pub struct DeviceMetadata {
    pub hw_id: u64,
    pub dev_size: u32,
    pub dev_category: DeviceCategory,
    pub dev_model: u32,
    pub additional_data: Option<u64>
}

impl DeviceMetadata {
    pub fn new(hw_id: u64, dev_size: u32, dev_category: DeviceCategory, dev_model: u32, additional_data: Option<u64>) -> Self {
        Self { hw_id, dev_size, dev_category, dev_model, additional_data }
    }

    pub fn set_size(&mut self, new_size: u32) -> &mut Self {
        self.dev_size = new_size;
        self
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0; 32];

        &bytes[00..=07].copy_from_slice(&self.hw_id.to_be_bytes());
        &bytes[08..=11].copy_from_slice(&self.dev_size.to_be_bytes());
        &bytes[12..=19].copy_from_slice(&self.dev_category.encode().to_be_bytes());
        &bytes[20..=23].copy_from_slice(&self.dev_model.to_be_bytes());
        &bytes[24..=31].copy_from_slice(&self.additional_data.unwrap_or(0).to_be_bytes());

        bytes
    }

    pub fn encode(&self) -> [u32; 8] {
        let mut words = [0; 8];
        &words.copy_from_slice(&bytes_to_words(&self.to_bytes()));
        words
    }
}
