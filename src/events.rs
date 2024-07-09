use crate::{
    telegram101::{Config, Telegram101},
    telegram104::Telegram104_I,
    types::{datatype::DataType, DataBuffer, COT},
};

/// Server event
#[derive(Clone, Debug)]
pub struct Event {
    adsu: u16,
    iou_addr: u32,
    tid: DataType,
    cot: COT,
    data: DataBuffer,
}

impl Event {
    /// Create a new event
    pub fn new(
        adsu: u16,
        iou_addr: u32,
        tid: DataType,
        cot: COT,
        data: impl Into<DataBuffer>,
    ) -> Self {
        Self {
            adsu,
            iou_addr,
            tid,
            cot,
            data: data.into(),
        }
    }
    /// Event ADSU
    pub fn adsu(&self) -> u16 {
        self.adsu
    }
    /// Event IOU address
    pub fn iou_addr(&self) -> u32 {
        self.iou_addr
    }
    /// Event data type
    pub fn tid(&self) -> DataType {
        self.tid
    }
    /// Event COT
    pub fn cot(&self) -> COT {
        self.cot
    }
    /// Event data
    pub fn data(&self) -> DataBuffer {
        self.data
    }
    /// Convert to IEC 60870-5-104 I-telegram
    pub fn into_telegram_104_i(self) -> Telegram104_I {
        self.into()
    }
    /// Convert to IEC 60870-5-101 telegram
    pub fn into_telegram_101(self, config: Config) -> Telegram101 {
        let mut telegram = Telegram101::new(self.tid, self.cot, self.adsu, config);
        telegram.append_iou(self.iou_addr, self.data);
        telegram
    }
}

impl From<Event> for Telegram104_I {
    fn from(event: Event) -> Self {
        let mut telegram = Telegram104_I::new(event.tid, event.cot, event.adsu);
        telegram.append_iou(event.iou_addr, event.data);
        telegram
    }
}
