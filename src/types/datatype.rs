#![allow(non_camel_case_types)]

use crate::Error;

use super::{
    time::{CP16Time2a, CP24Time2a, CP56Time2a},
    DataBuffer,
};

/// Single point information
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum SPI {
    #[default]
    /// Off
    Off = 0,
    /// On
    On = 1,
}

impl From<bool> for SPI {
    fn from(value: bool) -> Self {
        if value {
            SPI::On
        } else {
            SPI::Off
        }
    }
}

impl From<SPI> for bool {
    fn from(value: SPI) -> bool {
        match value {
            SPI::Off => false,
            SPI::On => true,
        }
    }
}

impl From<u8> for SPI {
    fn from(value: u8) -> Self {
        match value {
            0 => SPI::Off,
            _ => SPI::On,
        }
    }
}

/// Double point information
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum DPI {
    #[default]
    /// Indeterminate 0
    Indeterminate0 = 0,
    /// Off
    Off = 1,
    /// On
    On = 2,
    /// Indeterminate 3
    Indeterminate3 = 3,
}

impl From<bool> for DPI {
    fn from(value: bool) -> Self {
        if value {
            DPI::On
        } else {
            DPI::Off
        }
    }
}

impl From<DPI> for bool {
    fn from(value: DPI) -> bool {
        matches!(value, DPI::On)
    }
}

impl From<u8> for DPI {
    fn from(value: u8) -> Self {
        match value {
            0 => DPI::Indeterminate0,
            1 => DPI::Off,
            2 => DPI::On,
            _ => DPI::Indeterminate3,
        }
    }
}

/// Event state (single event of protection equipment)
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum EventState {
    #[default]
    /// Indeterminate 0
    Indeterminate0 = 0,
    /// Off
    Off = 1,
    /// On
    On = 2,
    /// Indeterminate 3
    Indeterminate3 = 3,
}

impl From<bool> for EventState {
    fn from(value: bool) -> Self {
        if value {
            EventState::On
        } else {
            EventState::Off
        }
    }
}

impl From<EventState> for bool {
    fn from(value: EventState) -> bool {
        matches!(value, EventState::On)
    }
}

impl From<u8> for EventState {
    fn from(value: u8) -> Self {
        match value {
            0 => EventState::Indeterminate0,
            1 => EventState::Off,
            2 => EventState::On,
            _ => EventState::Indeterminate3,
        }
    }
}

/// Single point information with quality descriptor
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SIQ {
    /// Invalid
    pub iv: bool,
    /// Not topical
    pub nt: bool,
    /// Substituted
    pub sb: bool,
    /// Blocked
    pub bl: bool,
    /// Single point information
    pub spi: SPI,
}

impl From<u8> for SIQ {
    fn from(value: u8) -> Self {
        SIQ {
            iv: value & 0b1000_0000 != 0,
            nt: value & 0b0100_0000 != 0,
            sb: value & 0b0010_0000 != 0,
            bl: value & 0b0001_0000 != 0,
            spi: SPI::from(value & 0b0000_0001),
        }
    }
}

impl From<SIQ> for u8 {
    fn from(data: SIQ) -> u8 {
        (u8::from(data.iv) << 7)
            | (u8::from(data.nt) << 6)
            | (u8::from(data.sb) << 5)
            | (u8::from(data.bl) << 4)
            | data.spi as u8
    }
}

/// Double point information with quality descriptor
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct DIQ {
    /// Invalid
    pub iv: bool,
    /// Not topical
    pub nt: bool,
    /// Substituted
    pub sb: bool,
    /// Blocked
    pub bl: bool,
    /// Double point information
    pub dpi: DPI,
}

impl From<u8> for DIQ {
    fn from(value: u8) -> Self {
        DIQ {
            iv: value & 0b1000_0000 != 0,
            nt: value & 0b0100_0000 != 0,
            sb: value & 0b0010_0000 != 0,
            bl: value & 0b0001_0000 != 0,
            dpi: DPI::from(value & 0b0000_0011),
        }
    }
}

impl From<DIQ> for u8 {
    fn from(data: DIQ) -> u8 {
        (u8::from(data.iv) << 7)
            | (u8::from(data.nt) << 6)
            | (u8::from(data.sb) << 5)
            | (u8::from(data.bl) << 4)
            | data.dpi as u8
    }
}

/// Single-point information
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_SP_NA_1 {
    /// Single point information with quality descriptor
    pub siq: SIQ,
}

impl From<DataBuffer> for M_SP_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            siq: SIQ::from(buf[0]),
        }
    }
}

impl From<M_SP_NA_1> for DataBuffer {
    fn from(data: M_SP_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.siq);
        buf
    }
}

/// Single-point information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_SP_TA_1 {
    /// Single point information with quality descriptor
    pub siq: SIQ,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_SP_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            siq: SIQ::from(buf[0]),
            time: CP24Time2a::from([buf[1], buf[2], buf[3]]),
        }
    }
}

impl From<M_SP_TA_1> for DataBuffer {
    fn from(data: M_SP_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.siq);
        buf[1..4].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Double-point information
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_DP_NA_1 {
    /// Double point information with quality descriptor
    pub diq: DIQ,
}

impl From<DataBuffer> for M_DP_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            diq: DIQ::from(buf[0]),
        }
    }
}

impl From<M_DP_NA_1> for DataBuffer {
    fn from(data: M_DP_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.diq);
        buf
    }
}

/// Double-point information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_DP_TA_1 {
    /// Double point information with quality descriptor
    pub diq: DIQ,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_DP_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            diq: DIQ::from(buf[0]),
            time: CP24Time2a::from([buf[1], buf[2], buf[3]]),
        }
    }
}

impl From<M_DP_TA_1> for DataBuffer {
    fn from(data: M_DP_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.diq);
        buf[1..4].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Quality descriptor
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QDS {
    /// Invalid
    pub iv: bool,
    /// Not topical
    pub nt: bool,
    /// Substituted
    pub sb: bool,
    /// Blocked
    pub bl: bool,
    /// Overflow
    pub ov: bool,
}

impl From<u8> for QDS {
    fn from(value: u8) -> Self {
        QDS {
            iv: value & 0b1000_0000 != 0,
            nt: value & 0b0100_0000 != 0,
            sb: value & 0b0010_0000 != 0,
            bl: value & 0b0001_0000 != 0,
            ov: value & 0b0000_0001 != 0,
        }
    }
}

impl From<QDS> for u8 {
    fn from(data: QDS) -> u8 {
        (u8::from(data.iv) << 7)
            | (u8::from(data.nt) << 6)
            | (u8::from(data.sb) << 5)
            | (u8::from(data.bl) << 4)
            | u8::from(data.ov)
    }
}

/// Value with transient state indication
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct VTI {
    /// Value
    pub value: u8,
    /// Transient state indication
    pub transient: bool,
    /// Quality descriptor
    pub qds: QDS,
}

impl From<[u8; 2]> for VTI {
    fn from(buf: [u8; 2]) -> Self {
        VTI {
            value: buf[0] & 0b0111_1111,
            transient: buf[1] & 0b1000_0000 != 0,
            qds: QDS::from(buf[1]),
        }
    }
}

impl From<VTI> for [u8; 2] {
    fn from(data: VTI) -> [u8; 2] {
        let mut buf = [0; 2];
        buf[0] = data.value | (u8::from(data.transient) << 7);
        buf[1] = u8::from(data.qds);
        buf
    }
}

/// Step position information
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ST_NA_1 {
    /// Value with transient state indication
    pub vti: VTI,
}

impl From<DataBuffer> for M_ST_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            vti: VTI::from([buf[0], buf[1]]),
        }
    }
}

impl From<M_ST_NA_1> for DataBuffer {
    fn from(data: M_ST_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.vti));
        buf
    }
}

/// Step position information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ST_TA_1 {
    /// Value with transient state indication
    pub vti: VTI,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_ST_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            vti: VTI::from([buf[0], buf[1]]),
            time: CP24Time2a::from([buf[2], buf[3], buf[4]]),
        }
    }
}

impl From<M_ST_TA_1> for DataBuffer {
    fn from(data: M_ST_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.vti));
        buf[2..5].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Bit string of 32 bits
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BSI {
    /// Value
    pub value: u32,
}

impl From<[u8; 4]> for BSI {
    fn from(buf: [u8; 4]) -> Self {
        BSI {
            value: u32::from_le_bytes(buf),
        }
    }
}

impl From<BSI> for [u8; 4] {
    fn from(data: BSI) -> [u8; 4] {
        data.value.to_le_bytes()
    }
}

/// Bit string of 32 bits
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_BO_NA_1 {
    /// Bit string of 32 bits
    pub bsi: BSI,
    /// Quality descriptor
    pub qds: QDS,
}

impl From<DataBuffer> for M_BO_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bsi: BSI::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
        }
    }
}

impl From<M_BO_NA_1> for DataBuffer {
    fn from(data: M_BO_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bsi));
        buf[4] = u8::from(data.qds);
        buf
    }
}

/// Bit string of 32 bits with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_BO_TA_1 {
    /// Bit string of 32 bits
    pub bsi: BSI,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_BO_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bsi: BSI::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
            time: CP24Time2a::from([buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<M_BO_TA_1> for DataBuffer {
    fn from(data: M_BO_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bsi));
        buf[4] = u8::from(data.qds);
        buf[5..8].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Normalized value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct NVA {
    /// Value
    pub value: u16,
}

impl From<[u8; 2]> for NVA {
    fn from(buf: [u8; 2]) -> Self {
        NVA {
            value: u16::from_le_bytes(buf),
        }
    }
}

impl From<NVA> for [u8; 2] {
    fn from(data: NVA) -> [u8; 2] {
        data.value.to_le_bytes()
    }
}

/// Measured value, normalized value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_NA_1 {
    /// Normalized value
    pub nva: NVA,
    /// Quality descriptor
    pub qds: QDS,
}

impl From<DataBuffer> for M_ME_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
            qds: QDS::from(buf[2]),
        }
    }
}

impl From<M_ME_NA_1> for DataBuffer {
    fn from(data: M_ME_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf[2] = u8::from(data.qds);
        buf
    }
}

/// Measured value, normalized value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_TA_1 {
    /// Normalized value
    pub nva: NVA,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_ME_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
            qds: QDS::from(buf[2]),
            time: CP24Time2a::from([buf[3], buf[4], buf[5]]),
        }
    }
}

impl From<M_ME_TA_1> for DataBuffer {
    fn from(data: M_ME_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf[2] = u8::from(data.qds);
        buf[3..6].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Scaled value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SVA {
    /// Value
    pub value: u16,
}

impl From<[u8; 2]> for SVA {
    fn from(buf: [u8; 2]) -> Self {
        SVA {
            value: u16::from_le_bytes(buf),
        }
    }
}

impl From<SVA> for [u8; 2] {
    fn from(data: SVA) -> [u8; 2] {
        data.value.to_le_bytes()
    }
}

/// Measured value, scaled value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_NB_1 {
    /// Scaled value
    pub sva: SVA,
    /// Quality descriptor
    pub qds: QDS,
}

impl From<DataBuffer> for M_ME_NB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sva: SVA::from([buf[0], buf[1]]),
            qds: QDS::from(buf[2]),
        }
    }
}

impl From<M_ME_NB_1> for DataBuffer {
    fn from(data: M_ME_NB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.sva));
        buf[2] = u8::from(data.qds);
        buf
    }
}

/// Measured value, scaled value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_TB_1 {
    /// Scaled value
    pub sva: SVA,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_ME_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sva: SVA::from([buf[0], buf[1]]),
            qds: QDS::from(buf[2]),
            time: CP24Time2a::from([buf[3], buf[4], buf[5]]),
        }
    }
}

impl From<M_ME_TB_1> for DataBuffer {
    fn from(data: M_ME_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.sva));
        buf[2] = u8::from(data.qds);
        buf[3..6].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Short floating point
#[derive(Debug, Clone, PartialEq, Default)]
pub struct R32 {
    /// Value
    pub value: f32,
}

impl Eq for R32 {}

impl From<[u8; 4]> for R32 {
    fn from(buf: [u8; 4]) -> Self {
        R32 {
            value: f32::from_le_bytes(buf),
        }
    }
}

impl From<R32> for [u8; 4] {
    fn from(data: R32) -> [u8; 4] {
        data.value.to_le_bytes()
    }
}

/// Measured value, short floating point
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_NC_1 {
    /// Short floating point
    pub r32: R32,
    /// Quality descriptor
    pub qds: QDS,
}

impl From<DataBuffer> for M_ME_NC_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            r32: R32::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
        }
    }
}

impl From<M_ME_NC_1> for DataBuffer {
    fn from(data: M_ME_NC_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.r32));
        buf[4] = u8::from(data.qds);
        buf
    }
}

/// Measured value, short floating point with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_TC_1 {
    /// Short floating point
    pub r32: R32,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_ME_TC_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            r32: R32::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
            time: CP24Time2a::from([buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<M_ME_TC_1> for DataBuffer {
    fn from(data: M_ME_TC_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.r32));
        buf[4] = u8::from(data.qds);
        buf[5..8].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Sequence quality descriptor
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SeqQD {
    /// Invalid
    pub iv: bool,
    /// Adjusted flag
    pub ca: bool,
    /// Carry flag
    pub cy: bool,
    /// Sequence
    pub seq: u8,
}

impl From<u8> for SeqQD {
    fn from(value: u8) -> Self {
        SeqQD {
            iv: value & 0b1000_0000 != 0,
            ca: value & 0b0100_0000 != 0,
            cy: value & 0b0010_0000 != 0,
            seq: value & 0b0001_1111,
        }
    }
}

impl From<SeqQD> for u8 {
    fn from(data: SeqQD) -> u8 {
        (u8::from(data.iv) << 7) | (u8::from(data.ca) << 6) | (u8::from(data.cy) << 5) | data.seq
    }
}

/// Binary counter reading
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct BCR {
    /// Value
    pub value: u32,
}

impl From<[u8; 4]> for BCR {
    fn from(buf: [u8; 4]) -> Self {
        BCR {
            value: u32::from_le_bytes(buf),
        }
    }
}

impl From<BCR> for [u8; 4] {
    fn from(data: BCR) -> [u8; 4] {
        data.value.to_le_bytes()
    }
}

/// Integrated total
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_IT_NA_1 {
    /// Binary counter reading
    pub bcr: BCR,
    /// Sequence quality descriptor
    pub seq_qd: SeqQD,
}

impl From<DataBuffer> for M_IT_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bcr: BCR::from([buf[0], buf[1], buf[2], buf[3]]),
            seq_qd: SeqQD::from(buf[4]),
        }
    }
}

impl From<M_IT_NA_1> for DataBuffer {
    fn from(data: M_IT_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bcr));
        buf[4] = u8::from(data.seq_qd);
        buf
    }
}

/// Integrated total with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_IT_TA_1 {
    /// Binary counter reading
    pub bcr: BCR,
    /// Sequence quality descriptor
    pub seq_qd: SeqQD,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_IT_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bcr: BCR::from([buf[0], buf[1], buf[2], buf[3]]),
            seq_qd: SeqQD::from(buf[4]),
            time: CP24Time2a::from([buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<M_IT_TA_1> for DataBuffer {
    fn from(data: M_IT_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bcr));
        buf[4] = u8::from(data.seq_qd);
        buf[5..8].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Single event of protection equipment
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SEP {
    /// Invalid
    pub iv: bool,
    /// Not topical
    pub nt: bool,
    /// Substituted
    pub sb: bool,
    /// Blocked
    pub bl: bool,
    /// Elapsed flag
    pub ei: bool,
    /// Event state
    pub es: EventState,
}

impl From<u8> for SEP {
    fn from(value: u8) -> Self {
        SEP {
            iv: value & 0b1000_0000 != 0,
            nt: value & 0b0100_0000 != 0,
            sb: value & 0b0010_0000 != 0,
            bl: value & 0b0001_0000 != 0,
            ei: value & 0b0000_1000 != 0,
            es: EventState::from(value & 0b0000_0011),
        }
    }
}

impl From<SEP> for u8 {
    fn from(data: SEP) -> u8 {
        (u8::from(data.iv) << 7)
            | (u8::from(data.nt) << 6)
            | (u8::from(data.sb) << 5)
            | (u8::from(data.bl) << 4)
            | (u8::from(data.ei) << 3)
            | data.es as u8
    }
}

/// Event of protection equipment with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EP_TA_1 {
    /// Single event of protection equipment
    pub sep: SEP,
    /// Elapsed time
    pub elapsed: CP16Time2a,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_EP_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sep: SEP::from(buf[0]),
            elapsed: CP16Time2a::from([buf[1], buf[2]]),
            time: CP24Time2a::from([buf[3], buf[4], buf[5]]),
        }
    }
}

impl From<M_EP_TA_1> for DataBuffer {
    fn from(data: M_EP_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.sep);
        buf[1..3].copy_from_slice(&<[u8; 2]>::from(data.elapsed));
        buf[3..6].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Start events of protection equipment
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct StartEP {
    /// SRD
    pub srd: bool,
    /// SIE
    pub sie: bool,
    /// SL3
    pub sl3: bool,
    /// SL2
    pub sl2: bool,
    /// SL1
    pub sl1: bool,
    /// GS
    pub gs: bool,
}

impl From<u8> for StartEP {
    fn from(value: u8) -> Self {
        StartEP {
            srd: value & 0b0010_0000 != 0,
            sie: value & 0b0001_0000 != 0,
            sl3: value & 0b0000_1000 != 0,
            sl2: value & 0b0000_0100 != 0,
            sl1: value & 0b0000_0010 != 0,
            gs: value & 0b0000_0001 != 0,
        }
    }
}

impl From<StartEP> for u8 {
    fn from(data: StartEP) -> u8 {
        (u8::from(data.srd) << 5)
            | (u8::from(data.sie) << 4)
            | (u8::from(data.sl3) << 3)
            | (u8::from(data.sl2) << 2)
            | (u8::from(data.sl1) << 1)
            | u8::from(data.gs)
    }
}

/// Quality descriptor of protection equipment
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QDP {
    /// Invalid
    pub iv: bool,
    /// Not topical
    pub nt: bool,
    /// Substituted
    pub sb: bool,
    /// Blocked
    pub bl: bool,
    /// Elapsed flag
    pub ei: bool,
}

impl From<u8> for QDP {
    fn from(value: u8) -> Self {
        QDP {
            iv: value & 0b1000_0000 != 0,
            nt: value & 0b0100_0000 != 0,
            sb: value & 0b0010_0000 != 0,
            bl: value & 0b0001_0000 != 0,
            ei: value & 0b0000_1000 != 0,
        }
    }
}

impl From<QDP> for u8 {
    fn from(data: QDP) -> u8 {
        (u8::from(data.iv) << 7)
            | (u8::from(data.nt) << 6)
            | (u8::from(data.sb) << 5)
            | (u8::from(data.bl) << 4)
            | (u8::from(data.ei) << 3)
    }
}

/// Packed start events of protection equipment with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EP_TB_1 {
    /// Start events of protection equipment
    pub start_ep: StartEP,
    /// Quality descriptor of protection equipment
    pub qdp: QDP,
    ///  Relay duration time
    pub relay_duration: CP16Time2a,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_EP_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            start_ep: StartEP::from(buf[0]),
            qdp: QDP::from(buf[1]),
            relay_duration: CP16Time2a::from([buf[2], buf[3]]),
            time: CP24Time2a::from([buf[4], buf[5], buf[6]]),
        }
    }
}

impl From<M_EP_TB_1> for DataBuffer {
    fn from(data: M_EP_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.start_ep);
        buf[1] = u8::from(data.qdp);
        buf[2..4].copy_from_slice(&<[u8; 2]>::from(data.relay_duration));
        buf[4..7].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Output circuit information
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct OCI {
    /// CL3
    pub cl3: bool,
    /// CL2
    pub cl2: bool,
    /// CL1
    pub cl1: bool,
    /// GC
    pub gc: bool,
}

impl From<u8> for OCI {
    fn from(value: u8) -> Self {
        OCI {
            cl3: value & 0b0000_1000 != 0,
            cl2: value & 0b0000_0100 != 0,
            cl1: value & 0b0000_0010 != 0,
            gc: value & 0b0000_0001 != 0,
        }
    }
}

impl From<OCI> for u8 {
    fn from(data: OCI) -> u8 {
        (u8::from(data.cl3) << 3)
            | (u8::from(data.cl2) << 2)
            | (u8::from(data.cl1) << 1)
            | u8::from(data.gc)
    }
}

/// Packed output circuit information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EP_TC_1 {
    /// Output circuit information
    pub oci: OCI,
    /// Quality descriptor of protection equipment
    pub qdp: QDP,
    /// Relay operation time
    pub relay_op_time: CP16Time2a,
    /// Time tag
    pub time: CP24Time2a,
}

impl From<DataBuffer> for M_EP_TC_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            oci: OCI::from(buf[0]),
            qdp: QDP::from(buf[1]),
            relay_op_time: CP16Time2a::from([buf[2], buf[3]]),
            time: CP24Time2a::from([buf[4], buf[5], buf[6]]),
        }
    }
}

impl From<M_EP_TC_1> for DataBuffer {
    fn from(data: M_EP_TC_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.oci);
        buf[1] = u8::from(data.qdp);
        buf[2..4].copy_from_slice(&<[u8; 2]>::from(data.relay_op_time));
        buf[4..7].copy_from_slice(&<[u8; 3]>::from(data.time));
        buf
    }
}

/// Status change detection
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SCD {
    /// Value
    pub value: u32,
}

impl From<[u8; 4]> for SCD {
    fn from(buf: [u8; 4]) -> Self {
        SCD {
            value: u32::from_le_bytes(buf),
        }
    }
}

impl From<SCD> for [u8; 4] {
    fn from(data: SCD) -> [u8; 4] {
        data.value.to_le_bytes()
    }
}

/// Packed single point information with status change detection
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_PS_NA_1 {
    /// Status change detection
    pub scd: SCD,
    /// Quality descriptor
    pub qds: QDS,
}

impl From<DataBuffer> for M_PS_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            scd: SCD::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
        }
    }
}

impl From<M_PS_NA_1> for DataBuffer {
    fn from(data: M_PS_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.scd));
        buf[4] = u8::from(data.qds);
        buf
    }
}

/// Measured value, normalized value without quality descriptor
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_ND_1 {
    /// Normalized value
    pub nva: NVA,
}

impl From<DataBuffer> for M_ME_ND_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
        }
    }
}

impl From<M_ME_ND_1> for DataBuffer {
    fn from(data: M_ME_ND_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf
    }
}

/// Single point information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_SP_TB_1 {
    /// Single point information with quality descriptor
    pub siq: SIQ,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_SP_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            siq: SIQ::from(buf[0]),
            time: CP56Time2a::from([buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<M_SP_TB_1> for DataBuffer {
    fn from(data: M_SP_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.siq);
        buf[1..8].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Double point information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_DP_TB_1 {
    /// Double point information with quality descriptor
    pub diq: DIQ,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_DP_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            diq: DIQ::from(buf[0]),
            time: CP56Time2a::from([buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<M_DP_TB_1> for DataBuffer {
    fn from(data: M_DP_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.diq);
        buf[1..8].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Step position information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ST_TB_1 {
    /// Value with transient state indication
    pub vti_value: u8,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_ST_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            vti_value: buf[0],
            qds: QDS::from(buf[1]),
            time: CP56Time2a::from([buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8]]),
        }
    }
}

impl From<M_ST_TB_1> for DataBuffer {
    fn from(data: M_ST_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = data.vti_value;
        buf[1] = u8::from(data.qds);
        buf[2..9].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Bit string of 32 bits with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_BO_TB_1 {
    /// Bit string of 32 bits
    pub bsi: BSI,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_BO_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bsi: BSI::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
            time: CP56Time2a::from([buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]]),
        }
    }
}

impl From<M_BO_TB_1> for DataBuffer {
    fn from(data: M_BO_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bsi));
        buf[4] = u8::from(data.qds);
        buf[5..12].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Normalized value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_TD_1 {
    /// Normalized value
    pub nva: NVA,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_ME_TD_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
            qds: QDS::from(buf[2]),
            time: CP56Time2a::from([buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]]),
        }
    }
}

impl From<M_ME_TD_1> for DataBuffer {
    fn from(data: M_ME_TD_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf[2] = u8::from(data.qds);
        buf[3..10].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Scaled value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_TE_1 {
    /// Scaled value
    pub sva: SVA,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_ME_TE_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sva: SVA::from([buf[0], buf[1]]),
            qds: QDS::from(buf[2]),
            time: CP56Time2a::from([buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]]),
        }
    }
}

impl From<M_ME_TE_1> for DataBuffer {
    fn from(data: M_ME_TE_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.sva));
        buf[2] = u8::from(data.qds);
        buf[3..10].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Short floating point with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_ME_TF_1 {
    /// Short floating point
    pub r32: R32,
    /// Quality descriptor
    pub qds: QDS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_ME_TF_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            r32: R32::from([buf[0], buf[1], buf[2], buf[3]]),
            qds: QDS::from(buf[4]),
            time: CP56Time2a::from([buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]]),
        }
    }
}

impl From<M_ME_TF_1> for DataBuffer {
    fn from(data: M_ME_TF_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.r32));
        buf[4] = u8::from(data.qds);
        buf[5..12].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Integrated total with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_IT_TB_1 {
    /// Binary counter reading
    pub bcr: BCR,
    /// Sequence quality descriptor
    pub seq_qd: SeqQD,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_IT_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bcr: BCR::from([buf[0], buf[1], buf[2], buf[3]]),
            seq_qd: SeqQD::from(buf[4]),
            time: CP56Time2a::from([buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]]),
        }
    }
}

impl From<M_IT_TB_1> for DataBuffer {
    fn from(data: M_IT_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bcr));
        buf[4] = u8::from(data.seq_qd);
        buf[5..12].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Single event of protection equipment with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EP_TD_1 {
    /// Single event of protection equipment
    pub sep: SEP,
    /// Elapsed time
    pub elapsed: CP16Time2a,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_EP_TD_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sep: SEP::from(buf[0]),
            elapsed: CP16Time2a::from([buf[1], buf[2]]),
            time: CP56Time2a::from([buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]]),
        }
    }
}

impl From<M_EP_TD_1> for DataBuffer {
    fn from(data: M_EP_TD_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.sep);
        buf[1..3].copy_from_slice(&<[u8; 2]>::from(data.elapsed));
        buf[3..10].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Packed start events of protection equipment with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EP_TE_1 {
    /// Start events of protection equipment
    pub start_ep: StartEP,
    /// Quality descriptor of protection equipment
    pub qdp: QDP,
    /// Relay duration time
    pub relay_duration: CP16Time2a,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_EP_TE_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            start_ep: StartEP::from(buf[0]),
            qdp: QDP::from(buf[1]),
            relay_duration: CP16Time2a::from([buf[2], buf[3]]),
            time: CP56Time2a::from([buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10]]),
        }
    }
}

impl From<M_EP_TE_1> for DataBuffer {
    fn from(data: M_EP_TE_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.start_ep);
        buf[1] = u8::from(data.qdp);
        buf[2..4].copy_from_slice(&<[u8; 2]>::from(data.relay_duration));
        buf[4..11].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Packed output circuit information with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EP_TF_1 {
    /// Output circuit information
    pub oci: OCI,
    /// Quality descriptor of protection equipment
    pub qdp: QDP,
    /// Relay operation time
    pub relay_op_time: CP16Time2a,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for M_EP_TF_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            oci: OCI::from(buf[0]),
            qdp: QDP::from(buf[1]),
            relay_op_time: CP16Time2a::from([buf[2], buf[3]]),
            time: CP56Time2a::from([buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10]]),
        }
    }
}

impl From<M_EP_TF_1> for DataBuffer {
    fn from(data: M_EP_TF_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.oci);
        buf[1] = u8::from(data.qdp);
        buf[2..4].copy_from_slice(&<[u8; 2]>::from(data.relay_op_time));
        buf[4..11].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Select/execute command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum SelectExecute {
    /// Execute
    Execute = 0,
    #[default]
    /// Select
    Select = 1,
}

impl From<u8> for SelectExecute {
    fn from(value: u8) -> Self {
        match value {
            0 => SelectExecute::Execute,
            _ => SelectExecute::Select,
        }
    }
}

/// Qualifer of command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum QU {
    #[default]
    /// Unspecified
    Unspecified,
    /// Short pulse
    ShortPulse,
    /// Long pulse
    LongPulse,
    /// Persistent
    Persistent,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for QU {
    fn from(value: u8) -> Self {
        match value {
            0 => QU::Unspecified,
            1 => QU::ShortPulse,
            2 => QU::LongPulse,
            3 => QU::Persistent,
            v => QU::Other(v),
        }
    }
}

impl From<QU> for u8 {
    fn from(data: QU) -> u8 {
        match data {
            QU::Unspecified => 0,
            QU::ShortPulse => 1,
            QU::LongPulse => 2,
            QU::Persistent => 3,
            QU::Other(v) => v,
        }
    }
}

/// Single command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct SCO {
    /// Select/execute
    pub se: SelectExecute,
    /// Qualifier of command
    pub qu: QU,
    /// Single command state information
    pub scs: SPI,
}

impl From<u8> for SCO {
    fn from(value: u8) -> Self {
        SCO {
            se: SelectExecute::from(value >> 7),
            qu: QU::from(value >> 2 & 0b0001_1111),
            scs: SPI::from(value & 0b0000_0001),
        }
    }
}

impl From<SCO> for u8 {
    fn from(data: SCO) -> u8 {
        (data.se as u8) << 7 | (u8::from(data.qu) & 0b0001_1111) << 2 | data.scs as u8
    }
}

/// Single command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SC_NA_1 {
    /// Single command
    pub sco: SCO,
}

impl From<DataBuffer> for C_SC_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sco: SCO::from(buf[0]),
        }
    }
}

impl From<C_SC_NA_1> for DataBuffer {
    fn from(data: C_SC_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.sco);
        buf
    }
}

/// Double command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct DCO {
    /// Select/execute
    pub se: SelectExecute,
    /// Qualifier of command
    pub qu: QU,
    /// Double command state information
    pub dcs: DPI,
}

impl From<u8> for DCO {
    fn from(value: u8) -> Self {
        DCO {
            se: SelectExecute::from(value >> 7),
            qu: QU::from(value >> 2 & 0b0001_1111),
            dcs: DPI::from(value & 0b0000_0001),
        }
    }
}

impl From<DCO> for u8 {
    fn from(data: DCO) -> u8 {
        (data.se as u8) << 7 | (u8::from(data.qu) & 0b0001_1111) << 2 | data.dcs as u8
    }
}

/// Double command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_DC_NA_1 {
    /// Double command
    pub dco: DCO,
}

impl From<DataBuffer> for C_DC_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            dco: DCO::from(buf[0]),
        }
    }
}

impl From<C_DC_NA_1> for DataBuffer {
    fn from(data: C_DC_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.dco);
        buf
    }
}

/// Status of regulating step
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum RCS {
    #[default]
    /// Not allowed
    NotAllowed0 = 0,
    /// Decrement
    Decrement = 1,
    /// Increment
    Increment = 2,
    /// Not allowed
    NotAllowed3 = 3,
}

impl From<u8> for RCS {
    fn from(value: u8) -> Self {
        match value {
            0 => RCS::NotAllowed0,
            1 => RCS::Decrement,
            2 => RCS::Increment,
            _ => RCS::NotAllowed3,
        }
    }
}

/// Regulating step command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct RCO {
    /// Select/execute
    pub se: SelectExecute,
    /// Qualifier of command
    pub qu: QU,
    /// Status of regulating step
    pub rcs: RCS,
}

impl From<u8> for RCO {
    fn from(value: u8) -> Self {
        RCO {
            se: SelectExecute::from(value >> 7),
            qu: QU::from(value >> 2 & 0b0001_1111),
            rcs: RCS::from(value & 0b0000_0011),
        }
    }
}

impl From<RCO> for u8 {
    fn from(data: RCO) -> u8 {
        (data.se as u8) << 7 | (u8::from(data.qu) & 0b0001_1111) << 2 | data.rcs as u8
    }
}

/// Regulating step command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_RC_NA_1 {
    /// Regulating step command
    pub rco: RCO,
}

impl From<DataBuffer> for C_RC_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            rco: RCO::from(buf[0]),
        }
    }
}

impl From<C_RC_NA_1> for DataBuffer {
    fn from(data: C_RC_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.rco);
        buf
    }
}

/// Qualifier of set point command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QL {
    /// 0 = default, 1.. - reserved
    pub ql: u8,
}

impl From<u8> for QL {
    fn from(value: u8) -> Self {
        QL {
            ql: value & 0b0111_1111,
        }
    }
}

impl From<QL> for u8 {
    fn from(data: QL) -> u8 {
        data.ql & 0b0111_1111
    }
}

/// Qualifier of set point command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QOS {
    /// Select/execute
    pub se: SelectExecute,
    /// Qualifier
    pub ql: QL,
}

impl From<u8> for QOS {
    fn from(value: u8) -> Self {
        QOS {
            se: SelectExecute::from(value >> 7),
            ql: QL::from(value),
        }
    }
}

impl From<QOS> for u8 {
    fn from(data: QOS) -> u8 {
        (data.se as u8) << 7 | u8::from(data.ql)
    }
}

/// Set point command, normalized value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SE_NA_1 {
    /// Normalized value
    pub nva: NVA,
    /// Quality descriptor
    pub qos: QOS,
}

impl From<DataBuffer> for C_SE_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
            qos: QOS::from(buf[2]),
        }
    }
}

impl From<C_SE_NA_1> for DataBuffer {
    fn from(data: C_SE_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf[2] = u8::from(data.qos);
        buf
    }
}

/// Set point command, scaled value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SE_NB_1 {
    /// Scaled value
    pub sva: SVA,
    /// Quality descriptor
    pub qos: QOS,
}

impl From<DataBuffer> for C_SE_NB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sva: SVA::from([buf[0], buf[1]]),
            qos: QOS::from(buf[2]),
        }
    }
}

impl From<C_SE_NB_1> for DataBuffer {
    fn from(data: C_SE_NB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.sva));
        buf[2] = u8::from(data.qos);
        buf
    }
}

/// Set point command, short floating point value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SE_NC_1 {
    /// Short floating point value
    pub r32: R32,
    /// Quality descriptor
    pub qos: QOS,
}

impl From<DataBuffer> for C_SE_NC_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            r32: R32::from([buf[0], buf[1], buf[2], buf[3]]),
            qos: QOS::from(buf[4]),
        }
    }
}

impl From<C_SE_NC_1> for DataBuffer {
    fn from(data: C_SE_NC_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.r32));
        buf[4] = u8::from(data.qos);
        buf
    }
}

/// Bit string of 32 bits command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_BO_NA_1 {
    /// Bit string of 32 bits
    pub bsi: BSI,
}

impl From<DataBuffer> for C_BO_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bsi: BSI::from([buf[0], buf[1], buf[2], buf[3]]),
        }
    }
}

impl From<C_BO_NA_1> for DataBuffer {
    fn from(data: C_BO_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bsi));
        buf
    }
}

/// Single command with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SC_TA_1 {
    /// Single command
    pub sco: SCO,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_SC_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sco: SCO::from(buf[0]),
            time: CP56Time2a::from([buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<C_SC_TA_1> for DataBuffer {
    fn from(data: C_SC_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.sco);
        buf[1..8].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Double command with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_DC_TA_1 {
    /// Double command
    pub dco: DCO,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_DC_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            dco: DCO::from(buf[0]),
            time: CP56Time2a::from([buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<C_DC_TA_1> for DataBuffer {
    fn from(data: C_DC_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.dco);
        buf[1..8].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Regulating step command with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_RC_TA_1 {
    /// Regulating step command
    pub rco: RCO,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_RC_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            rco: RCO::from(buf[0]),
            time: CP56Time2a::from([buf[1], buf[2], buf[3], buf[4], buf[5], buf[6], buf[7]]),
        }
    }
}

impl From<C_RC_TA_1> for DataBuffer {
    fn from(data: C_RC_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.rco);
        buf[1..8].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Set point command, normalized value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SE_TA_1 {
    /// Normalized value
    pub nva: NVA,
    /// Quality descriptor
    pub qos: QOS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_SE_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
            qos: QOS::from(buf[2]),
            time: CP56Time2a::from([buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]]),
        }
    }
}

impl From<C_SE_TA_1> for DataBuffer {
    fn from(data: C_SE_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf[2] = u8::from(data.qos);
        buf[3..10].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Set point command, scaled value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SE_TB_1 {
    /// Scaled value
    pub sva: SVA,
    /// Quality descriptor
    pub qos: QOS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_SE_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sva: SVA::from([buf[0], buf[1]]),
            qos: QOS::from(buf[2]),
            time: CP56Time2a::from([buf[3], buf[4], buf[5], buf[6], buf[7], buf[8], buf[9]]),
        }
    }
}

impl From<C_SE_TB_1> for DataBuffer {
    fn from(data: C_SE_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.sva));
        buf[2] = u8::from(data.qos);
        buf[3..10].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Set point command, short floating point value with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_SE_TC_1 {
    /// Short floating point value
    pub r32: R32,
    /// Quality descriptor
    pub qos: QOS,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_SE_TC_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            r32: R32::from([buf[0], buf[1], buf[2], buf[3]]),
            qos: QOS::from(buf[4]),
            time: CP56Time2a::from([buf[5], buf[6], buf[7], buf[8], buf[9], buf[10], buf[11]]),
        }
    }
}

impl From<C_SE_TC_1> for DataBuffer {
    fn from(data: C_SE_TC_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.r32));
        buf[4] = u8::from(data.qos);
        buf[5..12].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Bit string of 32 bits command with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_BO_TB_1 {
    /// Bit string of 32 bits
    pub bsi: BSI,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_BO_TB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            bsi: BSI::from([buf[0], buf[1], buf[2], buf[3]]),
            time: CP56Time2a::from([buf[4], buf[5], buf[6], buf[7], buf[8], buf[9], buf[10]]),
        }
    }
}

impl From<C_BO_TB_1> for DataBuffer {
    fn from(data: C_BO_TB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.bsi));
        buf[4..11].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Local parameter change
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum LPC {
    #[default]
    /// No change
    NoChange = 0,
    /// Changed
    Changed = 1,
}

impl From<u8> for LPC {
    fn from(value: u8) -> Self {
        match value {
            0 => LPC::NoChange,
            _ => LPC::Changed,
        }
    }
}

/// Cause of initialization
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum COI {
    #[default]
    /// Local power on
    LocalPowerOn,
    /// Local manual reset
    LocalManualReset,
    /// Remote reset
    RemoteReset,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for COI {
    fn from(value: u8) -> Self {
        match value {
            0 => COI::LocalPowerOn,
            1 => COI::LocalManualReset,
            2 => COI::RemoteReset,
            v => COI::Other(v & 0b0111_1111),
        }
    }
}

impl From<COI> for u8 {
    fn from(data: COI) -> u8 {
        match data {
            COI::LocalPowerOn => 0,
            COI::LocalManualReset => 1,
            COI::RemoteReset => 2,
            COI::Other(v) => v & 0b0111_1111,
        }
    }
}

/// End of initialization
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct M_EI_NA_1 {
    /// Local parameter change
    pub lpc: LPC,
    /// Cause of initialization
    pub coi: COI,
}

impl From<DataBuffer> for M_EI_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            lpc: LPC::from(buf[0] >> 7),
            coi: COI::from(buf[0]),
        }
    }
}

impl From<M_EI_NA_1> for DataBuffer {
    fn from(data: M_EI_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = (data.lpc as u8) << 7 | u8::from(data.coi);
        buf
    }
}

/// Qualifier of interrogation
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum QOI {
    #[default]
    /// Unused
    Unused,
    /// Interrogation
    Inrogen,
    /// Interrogation
    Inro1,
    /// Interrogation
    Inro2,
    /// Interrogation
    Inro3,
    /// Interrogation
    Inro4,
    /// Interrogation
    Inro5,
    /// Interrogation
    Inro6,
    /// Interrogation
    Inro7,
    /// Interrogation
    Inro8,
    /// Interrogation
    Inro9,
    /// Interrogation
    Inro10,
    /// Interrogation
    Inro11,
    /// Interrogation
    Inro12,
    /// Interrogation
    Inro13,
    /// Interrogation
    Inro14,
    /// Interrogation
    Inro15,
    /// Interrogation
    Inro16,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for QOI {
    fn from(value: u8) -> Self {
        match value {
            0 => QOI::Unused,
            20 => QOI::Inrogen,
            21 => QOI::Inro1,
            22 => QOI::Inro2,
            23 => QOI::Inro3,
            24 => QOI::Inro4,
            25 => QOI::Inro5,
            26 => QOI::Inro6,
            27 => QOI::Inro7,
            28 => QOI::Inro8,
            29 => QOI::Inro9,
            30 => QOI::Inro10,
            31 => QOI::Inro11,
            32 => QOI::Inro12,
            33 => QOI::Inro13,
            34 => QOI::Inro14,
            35 => QOI::Inro15,
            36 => QOI::Inro16,
            v => QOI::Other(v),
        }
    }
}

impl From<QOI> for u8 {
    fn from(data: QOI) -> u8 {
        match data {
            QOI::Unused => 0,
            QOI::Inrogen => 20,
            QOI::Inro1 => 21,
            QOI::Inro2 => 22,
            QOI::Inro3 => 23,
            QOI::Inro4 => 24,
            QOI::Inro5 => 25,
            QOI::Inro6 => 26,
            QOI::Inro7 => 27,
            QOI::Inro8 => 28,
            QOI::Inro9 => 29,
            QOI::Inro10 => 30,
            QOI::Inro11 => 31,
            QOI::Inro12 => 32,
            QOI::Inro13 => 33,
            QOI::Inro14 => 34,
            QOI::Inro15 => 35,
            QOI::Inro16 => 36,
            QOI::Other(v) => v,
        }
    }
}

/// Interrogation command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_IC_NA_1 {
    /// Qualifier of interrogation
    pub qoi: QOI,
}

impl From<DataBuffer> for C_IC_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            qoi: QOI::from(buf[0]),
        }
    }
}

impl From<C_IC_NA_1> for DataBuffer {
    fn from(data: C_IC_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.qoi);
        buf
    }
}

/// Freeze/reset qualifier of counter interrogation commands
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum FRZ {
    #[default]
    /// Read
    Read,
    /// Freeze
    Freeze,
    /// Freeze and reset
    FreezeAndReset,
    /// Reset
    Reset,
}

impl From<u8> for FRZ {
    fn from(value: u8) -> Self {
        match value {
            0 => FRZ::Read,
            1 => FRZ::Freeze,
            2 => FRZ::FreezeAndReset,
            _ => FRZ::Reset,
        }
    }
}

impl From<FRZ> for u8 {
    fn from(data: FRZ) -> u8 {
        match data {
            FRZ::Read => 0,
            FRZ::Freeze => 1,
            FRZ::FreezeAndReset => 2,
            FRZ::Reset => 3,
        }
    }
}

/// Request qualifier of counter interrogation commands
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum RQT {
    #[default]
    /// No counter read
    None,
    /// Group 1 counter interrogation
    ReqCo1,
    /// Group 2 counter interrogation
    ReqCo2,
    /// Group 3 counter interrogation
    ReqCo3,
    /// Group 4 counter interrogation
    ReqCo4,
    /// General counter interrogation
    ReqCoGen,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for RQT {
    fn from(value: u8) -> Self {
        match value {
            0 => RQT::None,
            1 => RQT::ReqCo1,
            2 => RQT::ReqCo2,
            3 => RQT::ReqCo3,
            4 => RQT::ReqCo4,
            5 => RQT::ReqCoGen,
            v => RQT::Other(v),
        }
    }
}

impl From<RQT> for u8 {
    fn from(data: RQT) -> u8 {
        match data {
            RQT::None => 0,
            RQT::ReqCo1 => 1,
            RQT::ReqCo2 => 2,
            RQT::ReqCo3 => 3,
            RQT::ReqCo4 => 4,
            RQT::ReqCoGen => 5,
            RQT::Other(v) => v,
        }
    }
}

/// Counter interrogation command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_CI_NA_1 {
    /// Freeze/reset qualifier
    pub frz: FRZ,
    /// Request qualifier
    pub rqt: RQT,
}

impl From<DataBuffer> for C_CI_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            frz: FRZ::from(buf[0] >> 6),
            rqt: RQT::from(buf[0] & 0b0011_1111),
        }
    }
}

impl From<C_CI_NA_1> for DataBuffer {
    fn from(data: C_CI_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.frz) << 6 | u8::from(data.rqt) & 0b0011_1111;
        buf
    }
}

/// Read command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_RD_NA_1 {}

impl From<DataBuffer> for C_RD_NA_1 {
    fn from(_buf: DataBuffer) -> Self {
        Self {}
    }
}

impl From<C_RD_NA_1> for DataBuffer {
    fn from(_data: C_RD_NA_1) -> DataBuffer {
        DataBuffer::default()
    }
}

/// Clock synchronization command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_CS_NA_1 {
    /// Time
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_CS_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            time: CP56Time2a::from([buf[0], buf[1], buf[2], buf[3], buf[4], buf[5], buf[6]]),
        }
    }
}

impl From<C_CS_NA_1> for DataBuffer {
    fn from(data: C_CS_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..7].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Test command (fixed pattern)
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct C_TS_NA_1 {
    fbp0: u8,
    fbp1: u8,
}

impl Default for C_TS_NA_1 {
    fn default() -> Self {
        Self {
            fbp0: 0xaa,
            fbp1: 0x55,
        }
    }
}

impl From<DataBuffer> for C_TS_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            fbp0: buf[0],
            fbp1: buf[1],
        }
    }
}

impl From<C_TS_NA_1> for DataBuffer {
    fn from(data: C_TS_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = data.fbp0;
        buf[1] = data.fbp1;
        buf
    }
}

/// Qualifier of reset process
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum QRP {
    #[default]
    /// Unused
    Unused,
    /// General process reset
    General,
    /// Reset pending events with time tag
    TtEvents,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for QRP {
    fn from(value: u8) -> Self {
        match value {
            0 => QRP::Unused,
            1 => QRP::General,
            2 => QRP::TtEvents,
            v => QRP::Other(v),
        }
    }
}

impl From<QRP> for u8 {
    fn from(data: QRP) -> u8 {
        match data {
            QRP::Unused => 0,
            QRP::General => 1,
            QRP::TtEvents => 2,
            QRP::Other(v) => v,
        }
    }
}

/// Reset process command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_RP_NA_1 {
    /// Qualifier of reset process
    pub qrp: QRP,
}

impl From<DataBuffer> for C_RP_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            qrp: QRP::from(buf[0]),
        }
    }
}

impl From<C_RP_NA_1> for DataBuffer {
    fn from(data: C_RP_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.qrp);
        buf
    }
}

/// Delay acquisition command
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_CD_NA_1 {
    /// Time
    pub time: CP16Time2a,
}

impl From<DataBuffer> for C_CD_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            time: CP16Time2a::from([buf[0], buf[1]]),
        }
    }
}

impl From<C_CD_NA_1> for DataBuffer {
    fn from(data: C_CD_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.time));
        buf
    }
}

/// Test command with time tag
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct C_TS_TA_1 {
    /// Test counter
    pub tsc: u16,
    /// Time tag
    pub time: CP56Time2a,
}

impl From<DataBuffer> for C_TS_TA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            tsc: u16::from_be_bytes([buf[0], buf[1]]),
            time: CP56Time2a::from([buf[2], buf[3], buf[4], buf[5], buf[6], buf[7], buf[8]]),
        }
    }
}

impl From<C_TS_TA_1> for DataBuffer {
    fn from(data: C_TS_TA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&data.tsc.to_be_bytes());
        buf[2..9].copy_from_slice(&<[u8; 7]>::from(data.time));
        buf
    }
}

/// Kind of parameter of measured value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum KPA {
    #[default]
    /// Unused
    Unused,
    /// Threshold
    Thresh,
    /// Filter
    Filter,
    /// Low limit
    LoLimit,
    /// High limit
    HiLimit,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for KPA {
    fn from(value: u8) -> Self {
        match value {
            0 => KPA::Unused,
            1 => KPA::Thresh,
            2 => KPA::Filter,
            3 => KPA::LoLimit,
            4 => KPA::HiLimit,
            v => KPA::Other(v),
        }
    }
}

impl From<KPA> for u8 {
    fn from(data: KPA) -> u8 {
        match data {
            KPA::Unused => 0,
            KPA::Thresh => 1,
            KPA::Filter => 2,
            KPA::LoLimit => 3,
            KPA::HiLimit => 4,
            KPA::Other(v) => v,
        }
    }
}

/// Qualifier of parameter of measured value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct QPM {
    /// Kind of parameter of measured value
    pub kpa: KPA,
    /// POP
    pub pop: bool,
    /// Local parameter change
    pub lpc: LPC,
}

impl From<u8> for QPM {
    fn from(value: u8) -> Self {
        QPM {
            kpa: KPA::from(value >> 5),
            pop: value & 0b0100_0000 != 0,
            lpc: LPC::from(value & 0b1000_0000 >> 7),
        }
    }
}

impl From<QPM> for u8 {
    fn from(data: QPM) -> u8 {
        (u8::from(data.kpa) << 5) | (u8::from(data.pop)) << 6 | (data.lpc as u8) << 7
    }
}

/// Parameter of measured value, normalized value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct P_ME_NA_1 {
    /// Normalized value
    pub nva: NVA,
    /// Qualifier of parameter of measured value
    pub qpm: QPM,
}

impl From<DataBuffer> for P_ME_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            nva: NVA::from([buf[0], buf[1]]),
            qpm: QPM::from(buf[2]),
        }
    }
}

impl From<P_ME_NA_1> for DataBuffer {
    fn from(data: P_ME_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.nva));
        buf[2] = u8::from(data.qpm);
        buf
    }
}

/// Parameter of scaled value, measured value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct P_ME_NB_1 {
    /// Scaled value
    pub sva: SVA,
    /// Qualifier of parameter of measured value
    pub qpm: QPM,
}

impl From<DataBuffer> for P_ME_NB_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            sva: SVA::from([buf[0], buf[1]]),
            qpm: QPM::from(buf[2]),
        }
    }
}

impl From<P_ME_NB_1> for DataBuffer {
    fn from(data: P_ME_NB_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..2].copy_from_slice(&<[u8; 2]>::from(data.sva));
        buf[2] = u8::from(data.qpm);
        buf
    }
}

/// Parameter of short floating point value, measured value
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct P_ME_NC_1 {
    /// Short floating point value
    pub r32: R32,
    /// Qualifier of parameter of measured value
    pub qpm: QPM,
}

impl From<DataBuffer> for P_ME_NC_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            r32: R32::from([buf[0], buf[1], buf[2], buf[3]]),
            qpm: QPM::from(buf[4]),
        }
    }
}

impl From<P_ME_NC_1> for DataBuffer {
    fn from(data: P_ME_NC_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0..4].copy_from_slice(&<[u8; 4]>::from(data.r32));
        buf[4] = u8::from(data.qpm);
        buf
    }
}

/// Qualifier of parameter activation
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum QPA {
    #[default]
    /// Unused
    Unused,
    /// General
    General,
    /// Object
    Object,
    /// Transmission
    Transmission,
    /// Other (custom)
    Other(u8),
}

impl From<u8> for QPA {
    fn from(value: u8) -> Self {
        match value {
            0 => QPA::Unused,
            1 => QPA::General,
            2 => QPA::Object,
            3 => QPA::Transmission,
            v => QPA::Other(v),
        }
    }
}

impl From<QPA> for u8 {
    fn from(data: QPA) -> u8 {
        match data {
            QPA::Unused => 0,
            QPA::General => 1,
            QPA::Object => 2,
            QPA::Transmission => 3,
            QPA::Other(v) => v,
        }
    }
}

/// Parameter activation
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct P_AC_NA_1 {
    /// Qualifier of parameter activation
    pub qpa: QPA,
}

impl From<DataBuffer> for P_AC_NA_1 {
    fn from(buf: DataBuffer) -> Self {
        Self {
            qpa: QPA::from(buf[0]),
        }
    }
}

impl From<P_AC_NA_1> for DataBuffer {
    fn from(data: P_AC_NA_1) -> DataBuffer {
        let mut buf = DataBuffer::default();
        buf[0] = u8::from(data.qpa);
        buf
    }
}

/// IEC 60870-5 101/104 data types
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub enum DataType {
    #[default]
    /// (000, 0x00) not allowed
    ASDU_TYPEUNDEF = 0,
    /// (001, 0x01) Single-point information
    M_SP_NA_1 = 1,
    /// (002, 0x02) Single-point information with time tag
    M_SP_TA_1 = 2,
    /// (003, 0x03) Double-point information
    M_DP_NA_1 = 3,
    /// (004, 0x04) Double-point information with time tag
    M_DP_TA_1 = 4,
    /// (005, 0x05) Step position information
    M_ST_NA_1 = 5,
    /// (006, 0x06) Step position information with time tag
    M_ST_TA_1 = 6,
    /// (007, 0x07) Bitstring of 32 bit
    M_BO_NA_1 = 7,
    /// (008, 0x08) Bitstring of 32 bit with time tag
    M_BO_TA_1 = 8,
    /// (009, 0x09) Measured value, normalised value
    M_ME_NA_1 = 9,
    /// (010, 0x0A) Measured value, normalized value with time tag
    M_ME_TA_1 = 10,
    /// (011, 0x0B) Measured value, scaled value
    M_ME_NB_1 = 11,
    /// (012, 0x0C) Measured value, scaled value with time tag
    M_ME_TB_1 = 12,
    /// (013, 0x0D) Measured value, short floating point number
    M_ME_NC_1 = 13,
    /// (014, 0x0E) Measured value, short floating point number with time tag
    M_ME_TC_1 = 14,
    /// (015, 0x0F) Integrated totals
    M_IT_NA_1 = 15,
    /// (016, 0x10) Integrated totals with time tag
    M_IT_TA_1 = 16,
    /// (017, 0x11) Event of protection equipment with time tag
    M_EP_TA_1 = 17,
    /// (018, 0x12) Packed start events of protection equipment with time tag
    M_EP_TB_1 = 18,
    /// (019, 0x13) Packed output circuit information of protection equipment with time tag
    M_EP_TC_1 = 19,
    /// (020, 0x14) Packed single point information with status change detection
    M_PS_NA_1 = 20,
    /// (021, 0x15) Measured value, normalized value without quality descriptor
    M_ME_ND_1 = 21,
    /// (030, 0x1E) Single-point information with time tag CP56Time2a
    M_SP_TB_1 = 30,
    /// (031, 0x1F) Double-point information with time tag CP56Time2a
    M_DP_TB_1 = 31,
    /// (032, 0x20) Step position information with time tag CP56Time2a
    M_ST_TB_1 = 32,
    /// (033, 0x21) Bitstring of 32 bit with time tag CP56Time2a
    M_BO_TB_1 = 33,
    /// (034, 0x22) Measured value, normalised value with time tag CP56Time2a
    M_ME_TD_1 = 34,
    /// (035, 0x23) Measured value, scaled value with time tag CP56Time2a
    M_ME_TE_1 = 35,
    /// (036, 0x24) Measured value, short floating point number with time tag CP56Time2a
    M_ME_TF_1 = 36,
    /// (037, 0x25) Integrated totals with time tag CP56Time2a
    M_IT_TB_1 = 37,
    /// (038, 0x26) Event of protection equipment with time tag CP56Time2a
    M_EP_TD_1 = 38,
    /// (039, 0x27) Packed start events of protection equipment with time tag CP56Time2a
    M_EP_TE_1 = 39,
    /// (040, 0x28) Packed output circuit information of protection equipment with time tag CP56Time2a
    M_EP_TF_1 = 40,
    /// (045, 0x2D) Single command
    C_SC_NA_1 = 45,
    /// (046, 0x2E) Double command
    C_DC_NA_1 = 46,
    /// (047, 0x2F) Regulating step command
    C_RC_NA_1 = 47,
    /// (048, 0x30) Set-point Command, normalised value
    C_SE_NA_1 = 48,
    /// (049, 0x31) Set-point Command, scaled value
    C_SE_NB_1 = 49,
    /// (050, 0x32) Set-point Command, short floating point number
    C_SE_NC_1 = 50,
    /// (051, 0x33) Bitstring 32 bit command
    C_BO_NA_1 = 51,
    /// (058, 0x3A) Single command with time tag CP56Time2a
    C_SC_TA_1 = 58,
    /// (059, 0x3B) Double command with time tag CP56Time2a
    C_DC_TA_1 = 59,
    /// (060, 0x3C) Regulating step command with time tag CP56Time2a
    C_RC_TA_1 = 60,
    /// (061, 0x3D) Measured value, normalised value command with time tag CP56Time2a
    C_SE_TA_1 = 61,
    /// (062, 0x3E) Measured value, scaled value command with time tag CP56Time2a
    C_SE_TB_1 = 62,
    /// (063, 0x3F) Measured value, short floating point number command with time tag CP56Time2a
    C_SE_TC_1 = 63,
    /// (064, 0x40) Bitstring of 32 bit command with time tag CP56Time2a
    C_BO_TA_1 = 64,
    /// (070, 0x46) End of Initialisation
    M_EI_NA_1 = 70,
    /// (100, 0x64) Interrogation command
    C_IC_NA_1 = 100,
    /// (101, 0x65) Counter interrogation command
    C_CI_NA_1 = 101,
    /// (102, 0x66) Read Command
    C_RD_NA_1 = 102,
    /// (103, 0x67) Clock synchronisation command
    C_CS_NA_1 = 103,
    /// (104, 0x68) Test command
    C_TS_NA_1 = 104,
    /// (105, 0x69) Reset process command
    C_RP_NA_1 = 105,
    /// (106, 0x6A) Delay acquisition command
    C_CD_NA_1 = 106,
    /// (107, 0x6B) Test command with time tag CP56Time2a
    C_TS_TA_1 = 107,
    /// (110, 0x6E) Parameter of measured values, normalized value
    P_ME_NA_1 = 110,
    /// (111, 0x6F) Parameter of measured values, scaled value
    P_ME_NB_1 = 111,
    /// (112, 0x70) Parameter of measured values, short floating point number
    P_ME_NC_1 = 112,
    /// (113, 0x71) Parameter activation
    P_AC_NA_1 = 113,
    //
    //F_FR_NA_1 = 120, // (120, 0x78) File ready
    //F_SR_NA_1 = 121, // (121, 0x79) Section ready
    //F_SC_NA_1 = 122, // (122, 0x7A) Call directory, select file, call file, call section
    //F_LS_NA_1 = 123, // (123, 0x7B) Last section, last segment
    //F_FA_NA_1 = 124, // (124, 0x7C) ACK file, ACK section
    //F_SG_NA_1 = 125, // (125, 0x7D) Segment
    //F_DR_TA_1 = 126, // (126, 0x7E) Directory
}

impl TryFrom<u8> for DataType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DataType::ASDU_TYPEUNDEF),
            1 => Ok(DataType::M_SP_NA_1),
            2 => Ok(DataType::M_SP_TA_1),
            3 => Ok(DataType::M_DP_NA_1),
            4 => Ok(DataType::M_DP_TA_1),
            5 => Ok(DataType::M_ST_NA_1),
            6 => Ok(DataType::M_ST_TA_1),
            7 => Ok(DataType::M_BO_NA_1),
            8 => Ok(DataType::M_BO_TA_1),
            9 => Ok(DataType::M_ME_NA_1),
            10 => Ok(DataType::M_ME_TA_1),
            11 => Ok(DataType::M_ME_NB_1),
            12 => Ok(DataType::M_ME_TB_1),
            13 => Ok(DataType::M_ME_NC_1),
            14 => Ok(DataType::M_ME_TC_1),
            15 => Ok(DataType::M_IT_NA_1),
            16 => Ok(DataType::M_IT_TA_1),
            17 => Ok(DataType::M_EP_TA_1),
            18 => Ok(DataType::M_EP_TB_1),
            19 => Ok(DataType::M_EP_TC_1),
            20 => Ok(DataType::M_PS_NA_1),
            21 => Ok(DataType::M_ME_ND_1),
            30 => Ok(DataType::M_SP_TB_1),
            31 => Ok(DataType::M_DP_TB_1),
            32 => Ok(DataType::M_ST_TB_1),
            33 => Ok(DataType::M_BO_TB_1),
            34 => Ok(DataType::M_ME_TD_1),
            35 => Ok(DataType::M_ME_TE_1),
            36 => Ok(DataType::M_ME_TF_1),
            37 => Ok(DataType::M_IT_TB_1),
            38 => Ok(DataType::M_EP_TD_1),
            39 => Ok(DataType::M_EP_TE_1),
            40 => Ok(DataType::M_EP_TF_1),
            45 => Ok(DataType::C_SC_NA_1),
            46 => Ok(DataType::C_DC_NA_1),
            47 => Ok(DataType::C_RC_NA_1),
            48 => Ok(DataType::C_SE_NA_1),
            49 => Ok(DataType::C_SE_NB_1),
            50 => Ok(DataType::C_SE_NC_1),
            51 => Ok(DataType::C_BO_NA_1),
            58 => Ok(DataType::C_SC_TA_1),
            59 => Ok(DataType::C_DC_TA_1),
            60 => Ok(DataType::C_RC_TA_1),
            61 => Ok(DataType::C_SE_TA_1),
            62 => Ok(DataType::C_SE_TB_1),
            63 => Ok(DataType::C_SE_TC_1),
            64 => Ok(DataType::C_BO_TA_1),
            70 => Ok(DataType::M_EI_NA_1),
            100 => Ok(DataType::C_IC_NA_1),
            101 => Ok(DataType::C_CI_NA_1),
            102 => Ok(DataType::C_RD_NA_1),
            103 => Ok(DataType::C_CS_NA_1),
            104 => Ok(DataType::C_TS_NA_1),
            105 => Ok(DataType::C_RP_NA_1),
            106 => Ok(DataType::C_CD_NA_1),
            107 => Ok(DataType::C_TS_TA_1),
            110 => Ok(DataType::P_ME_NA_1),
            111 => Ok(DataType::P_ME_NB_1),
            112 => Ok(DataType::P_ME_NC_1),
            113 => Ok(DataType::P_AC_NA_1),
            //120 => Ok(IEC60870_Type::F_FR_NA_1),
            //121 => Ok(IEC60870_Type::F_SR_NA_1),
            //122 => Ok(IEC60870_Type::F_SC_NA_1),
            //123 => Ok(IEC60870_Type::F_LS_NA_1),
            //124 => Ok(IEC60870_Type::F_FA_NA_1),
            //125 => Ok(IEC60870_Type::F_SG_NA_1),
            //126 => Ok(IEC60870_Type::F_DR_TA_1),
            v => Err(Error::DataType(v)),
        }
    }
}

impl DataType {
    /// Get the size of the data type in bytes
    #[allow(clippy::match_same_arms)]
    pub fn size(self) -> usize {
        match self {
            DataType::ASDU_TYPEUNDEF => 0,
            DataType::M_SP_NA_1 => 1,
            DataType::M_SP_TA_1 => 4,
            DataType::M_DP_NA_1 => 1,
            DataType::M_DP_TA_1 => 4,
            DataType::M_ST_NA_1 => 2,
            DataType::M_ST_TA_1 => 5,
            DataType::M_BO_NA_1 => 5,
            DataType::M_BO_TA_1 => 8,
            DataType::M_ME_NA_1 => 3,
            DataType::M_ME_TA_1 => 6,
            DataType::M_ME_NB_1 => 3,
            DataType::M_ME_TB_1 => 6,
            DataType::M_ME_NC_1 => 5,
            DataType::M_ME_TC_1 => 8,
            DataType::M_IT_NA_1 => 5,
            DataType::M_IT_TA_1 => 8,
            DataType::M_EP_TA_1 => 6,
            DataType::M_EP_TB_1 => 7,
            DataType::M_EP_TC_1 => 7,
            DataType::M_PS_NA_1 => 5,
            DataType::M_ME_ND_1 => 2,
            DataType::M_SP_TB_1 => 8,
            DataType::M_DP_TB_1 => 8,
            DataType::M_ST_TB_1 => 9,
            DataType::M_BO_TB_1 => 12,
            DataType::M_ME_TD_1 => 10,
            DataType::M_ME_TE_1 => 10,
            DataType::M_ME_TF_1 => 12,
            DataType::M_IT_TB_1 => 12,
            DataType::M_EP_TD_1 => 10,
            DataType::M_EP_TE_1 => 11,
            DataType::M_EP_TF_1 => 11,

            DataType::C_SC_NA_1 => 1,
            DataType::C_DC_NA_1 => 1,
            DataType::C_RC_NA_1 => 1,
            DataType::C_SE_NA_1 => 3,
            DataType::C_SE_NB_1 => 3,
            DataType::C_SE_NC_1 => 5,
            DataType::C_BO_NA_1 => 4,
            DataType::C_SC_TA_1 => 8,
            DataType::C_DC_TA_1 => 8,
            DataType::C_RC_TA_1 => 8,
            DataType::C_SE_TA_1 => 10,
            DataType::C_SE_TB_1 => 10,
            DataType::C_SE_TC_1 => 12,
            DataType::C_BO_TA_1 => 11,

            DataType::M_EI_NA_1 => 1,

            DataType::C_IC_NA_1 => 1,
            DataType::C_CI_NA_1 => 1,
            DataType::C_RD_NA_1 => 0,
            DataType::C_CS_NA_1 => 7,
            DataType::C_TS_NA_1 => 2,
            DataType::C_RP_NA_1 => 1,
            DataType::C_CD_NA_1 => 2,
            DataType::C_TS_TA_1 => 9,

            DataType::P_ME_NA_1 => 3,
            DataType::P_ME_NB_1 => 3,
            DataType::P_ME_NC_1 => 5,
            DataType::P_AC_NA_1 => 1,
            //IEC60870_Type::F_FR_NA_1 => 1,
            //IEC60870_Type::F_SR_NA_1 => 1,
            //IEC60870_Type::F_SC_NA_1 => 1,
            //IEC60870_Type::F_LS_NA_1 => 1,
            //IEC60870_Type::F_FA_NA_1 => 1,
            //IEC60870_Type::F_SG_NA_1 => 1,
            //IEC60870_Type::F_DR_TA_1 => 1,
        }
    }
}
