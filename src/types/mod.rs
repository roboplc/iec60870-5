mod cot;
/// IEC 60870-5 101/104 common data types
pub mod datatype;
/// IEC 60870-5 101/104 time types
pub mod time;

pub use cot::COT;

/// Maximum length of IEC 60870-5 IOU data
pub const MAX_IEC_DATA_LEN: usize = 12;

/// IEC 60870-5 IOU data buffer
pub type DataBuffer = [u8; MAX_IEC_DATA_LEN];

/// IEC 60870-5 IOU
#[derive(Debug, Clone)]
pub struct Iou {
    pub(crate) address: u32,
    pub(crate) value: DataBuffer,
}

impl Iou {
    /// Create a new IEC 60870-5 IOU
    pub fn new(address: u32, value: impl Into<DataBuffer>) -> Self {
        Self {
            address,
            value: value.into(),
        }
    }
    /// Get the address
    pub fn address(&self) -> u32 {
        self.address
    }
    /// Get the value
    pub fn value(&self) -> DataBuffer {
        self.value
    }
}
