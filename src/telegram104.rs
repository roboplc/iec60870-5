use std::{
    io::{Cursor, Read, Write},
    sync::{
        atomic::{AtomicU16, Ordering},
        Arc,
    },
};

use crate::{
    types::{datatype::DataType, DataBuffer, Iou, COT, MAX_IEC_DATA_LEN},
    Error,
};

const IEC_HEADER: u8 = 0x68;
const FRAME_COUNTER_MAX: u16 = 32767;

/// Chat sequence counter. Important: despite of providing all methods in a thread-safe manner by
/// default, the counter must be still covered with a mutex or a similar synchronization primitive
/// in a multi-threaded environment.
#[derive(Default, Debug, Clone)]
pub struct ChatSequenceCounter {
    inner: Arc<ChatSequenceCounterInner>,
}

#[derive(Default, Debug)]
struct ChatSequenceCounterInner {
    tx: AtomicU16,
    rx: AtomicU16,
}

impl ChatSequenceCounter {
    /// Creates a new TX/RX chat sequence
    pub fn new() -> Self {
        Self::default()
    }
}

impl ChatSequenceCounter {
    /// Manually increments the RX counter
    pub fn increment_rx(&self) -> u16 {
        let rx = self.inner.rx.fetch_add(1, Ordering::Relaxed);
        if rx >= FRAME_COUNTER_MAX {
            self.inner.rx.store(0, Ordering::Relaxed);
        }
        rx + 1
    }
    /// Manually increments the TX counter
    pub fn increment_tx(&self) -> u16 {
        let tx = self.inner.tx.fetch_add(1, Ordering::Relaxed);
        if tx >= FRAME_COUNTER_MAX {
            self.inner.tx.store(0, Ordering::Relaxed);
        } else {
            self.inner.tx.store(tx + 1, Ordering::Relaxed);
        }
        tx + 1
    }
    /// Returns the current TX counter value
    pub fn current_tx(&self) -> u16 {
        self.inner.tx.load(Ordering::Relaxed)
    }
    /// Returns the current RX counter value
    pub fn current_rx(&self) -> u16 {
        self.inner.rx.load(Ordering::Relaxed)
    }
    /// Resets the TX and RX counters to 0
    pub fn reset(&self) {
        self.inner.tx.store(0, Ordering::Relaxed);
        self.inner.rx.store(0, Ordering::Relaxed);
    }
}

/// IEC 60870-5-104 telegram
#[derive(Debug, Clone)]
pub enum Telegram104 {
    /// U-frame
    U(Telegram104_U),
    /// S-frame
    S(Telegram104_S),
    /// I-frame
    I(Telegram104_I),
}

impl Telegram104 {
    /// Applies the counter values to the outgoing telegram send/receive sequence numbers and
    /// increments the send sequence number if required
    pub fn chat_sequence_apply_outgoing(&mut self, counter: &ChatSequenceCounter) {
        match self {
            Self::U(_) => {}
            Self::S(s) => s.chat_sequence_apply_outgoing(counter),
            Self::I(i) => i.chat_sequence_apply_outgoing(counter),
        }
    }
    /// Validates the received telegram against the counter values and increments the receive
    /// sequence number if required
    pub fn chat_sequence_validate_incoming(
        &self,
        counter: &ChatSequenceCounter,
    ) -> Result<(), Error> {
        match self {
            Self::U(_) => Ok(()),
            Self::S(s) => s.chat_sequence_validate_incoming(counter),
            Self::I(i) => i.chat_sequence_validate_incoming(counter),
        }
    }
    /// New U-frame with start data transfer
    pub fn new_start_dt() -> Self {
        Self::U(Telegram104_U::new_start_dt())
    }
    /// New U-frame with stop data transfer
    pub fn new_stop_dt() -> Self {
        Self::U(Telegram104_U::new_stop_dt())
    }
    /// New U-frame with test
    pub fn new_test() -> Self {
        Self::U(Telegram104_U::new_test())
    }
    /// Write the telegram to a writer
    pub fn write(&self, mut writer: impl Write) -> Result<(), Error> {
        let mut buf = Cursor::new(Vec::with_capacity(256));
        buf.write_all(&[IEC_HEADER])?;
        match self {
            Self::U(u) => u.write(&mut buf)?,
            Self::S(s) => s.write(&mut buf)?,
            Self::I(i) => i.write(&mut buf)?,
        }
        writer.write_all(&buf.into_inner()).map_err(Into::into)
    }
    /// Read the telegram from a reader
    ///
    /// # Panics
    ///
    /// Should not panic
    pub fn read(mut reader: impl Read) -> Result<Self, Error> {
        let mut header_buf = [0u8, 2];
        reader.read_exact(&mut header_buf)?;
        if header_buf[0] != IEC_HEADER {
            return Err(Error::invalid_data("Invalid header"));
        }
        let length = usize::from(header_buf[1]);
        if length > 253 {
            return Err(Error::invalid_data("Telegram too long"));
        }
        let mut buf = [0u8; 253];
        reader.read_exact(&mut buf[..length])?;
        if length < 4 {
            return Err(Error::invalid_data("Telegram too short"));
        }
        Ok(if length == 4 {
            let control_buf: [u8; 4] = buf[..4].try_into().unwrap();
            match control_buf[0] & 0b11 {
                0b01 => Telegram104::S(Telegram104_S::try_from_control_buf(control_buf)?),
                0b11 => Telegram104::U(Telegram104_U::from_control_buf(control_buf)),
                _ => return Err(Error::invalid_data("Invalid control field")),
            }
        } else {
            Telegram104::I(Telegram104_I::read(Cursor::new(buf))?)
        })
    }
}

/// S-frame telegram
#[derive(Debug, Clone, Default)]
#[allow(non_camel_case_types, clippy::module_name_repetitions)]
pub struct Telegram104_S {
    recv_sn: u16,
}

impl From<Telegram104_S> for Telegram104 {
    fn from(s: Telegram104_S) -> Self {
        Self::S(s)
    }
}

impl Telegram104_S {
    /// Creates a new S-frame telegram
    pub fn new() -> Self {
        Self::default()
    }
    /// Applies the counter values to the outgoing telegram send/receive sequence numbers and
    /// increments the send sequence number
    pub fn chat_sequence_apply_outgoing(&mut self, counter: &ChatSequenceCounter) {
        self.recv_sn = counter.current_rx();
        //counter.increment_tx(); // TODO - check
    }
    /// Validates the received telegram against the counter values and increments the receive
    /// sequence number
    pub fn chat_sequence_validate_incoming(
        &self,
        counter: &ChatSequenceCounter,
    ) -> Result<(), Error> {
        let current_tx = counter.current_tx();
        if self.recv_sn != current_tx {
            return Err(Error::ChatSequence(self.recv_sn, current_tx));
        }
        //counter.increment_rx(); // TODO - check
        Ok(())
    }
    /// Get the receive sequence number
    pub fn recv_sn(&self) -> u16 {
        self.recv_sn
    }
    fn try_from_control_buf(control_buf: [u8; 4]) -> Result<Self, Error> {
        if control_buf[2] & 1 != 0 {
            return Err(Error::invalid_data("Invalid S-frame control field"));
        }
        Ok(Self {
            recv_sn: u16::from(control_buf[2] >> 1) | (u16::from(control_buf[3]) << 7),
        })
    }
    fn write(&self, mut writer: impl Write) -> Result<(), Error> {
        let buf = [
            0x4,
            0b01,
            0,
            u8::try_from((self.recv_sn & 0b0111_1111) << 1).unwrap(),
            u8::try_from(self.recv_sn >> 7).unwrap(),
        ];
        writer.write_all(&buf)?;
        Ok(())
    }
    /// Convert to Telegram104
    pub fn into_telegram104(self) -> Telegram104 {
        self.into()
    }
}

/// U-frame telegram
#[derive(Debug, Clone)]
#[allow(
    non_camel_case_types,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools
)]
pub struct Telegram104_U {
    test: bool,
    start_dt: bool,
    stop_dt: bool,
    con: bool,
}

impl From<Telegram104_U> for Telegram104 {
    fn from(u: Telegram104_U) -> Self {
        Self::U(u)
    }
}

impl Telegram104_U {
    /// Is this a start data transfer frame
    pub fn is_start_dt(&self) -> bool {
        self.start_dt
    }
    /// Is this a stop data transfer frame
    pub fn is_stop_dt(&self) -> bool {
        self.stop_dt
    }
    /// Is this a test frame
    pub fn is_test(&self) -> bool {
        self.test
    }
    /// Is this a confirmation frame
    pub fn is_con(&self) -> bool {
        self.con
    }
    /// Creates a new U-frame telegram for starting data transfer
    pub fn new_start_dt() -> Self {
        Self {
            test: false,
            start_dt: true,
            stop_dt: false,
            con: false,
        }
    }
    /// Creates a new U-frame telegram for stopping data transfer
    pub fn new_stop_dt() -> Self {
        Self {
            test: false,
            start_dt: false,
            stop_dt: true,
            con: false,
        }
    }
    /// Creates a new U-frame telegram for testing
    pub fn new_test() -> Self {
        Self {
            test: true,
            start_dt: false,
            stop_dt: false,
            con: false,
        }
    }
    fn from_control_buf(control_buf: [u8; 4]) -> Self {
        let control = control_buf[0];
        let mut con = false;
        let test = control & 0b1100_0000 != 0;
        let stop_dt = control & 0b0011_0000 != 0;
        let start_dt = control & 0b0000_1100 != 0;
        if test && control & 0b1000_0000 != 0 {
            con = true;
        }
        if stop_dt && control & 0b0010_0000 != 0 {
            con = true;
        }
        if start_dt && control & 0b0000_1000 != 0 {
            con = true;
        }
        Self {
            test,
            start_dt,
            stop_dt,
            con,
        }
    }
    fn write(&self, mut writer: impl Write) -> Result<(), Error> {
        let mut control = 0b11;
        if self.con {
            if self.test {
                control |= 0b1000_0000;
            }
            if self.stop_dt {
                control |= 0b0010_0000;
            }
            if self.start_dt {
                control |= 0b0000_1000;
            }
        } else {
            if self.test {
                control |= 0b0100_0000;
            }
            if self.stop_dt {
                control |= 0b0001_0000;
            }
            if self.start_dt {
                control |= 0b0000_0100;
            }
        }
        writer.write_all(&([4, control, 0, 0, 0]))?;
        Ok(())
    }
    /// Convert to Telegram104
    pub fn into_telegram104(self) -> Telegram104 {
        self.into()
    }
}

/// I-frame telegram
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Telegram104_I {
    send_sn: u16,
    recv_sn: u16,
    data_type: DataType,
    test: bool,
    negative: bool,
    cot: COT,
    originator: u8,
    adsu: u16,
    iou: Vec<Iou>,
    sequental: bool,
}

impl From<Telegram104_I> for Telegram104 {
    fn from(i: Telegram104_I) -> Self {
        Self::I(i)
    }
}

impl Telegram104_I {
    /// Creates a new I-frame telegram
    pub fn new(data_type: DataType, cot: COT, adsu: u16) -> Self {
        Self {
            send_sn: 0,
            recv_sn: 0,
            data_type,
            test: false,
            negative: false,
            cot,
            originator: 0,
            adsu,
            iou: <_>::default(),
            sequental: false,
        }
    }
    /// Applies the counter values to the outgoing telegram send/receive sequence numbers and
    /// increments the send sequence number
    pub fn chat_sequence_apply_outgoing(&mut self, counter: &ChatSequenceCounter) {
        self.send_sn = counter.current_tx();
        self.recv_sn = counter.current_rx();
        counter.increment_tx();
    }
    /// Validates the received telegram against the counter values and increments the receive
    /// sequence number
    pub fn chat_sequence_validate_incoming(
        &self,
        counter: &ChatSequenceCounter,
    ) -> Result<(), Error> {
        let current_rx = counter.current_rx();
        if self.send_sn != current_rx {
            return Err(Error::ChatSequence(self.send_sn, current_rx));
        }
        counter.increment_rx();
        Ok(())
    }
    /// Manually sets the TX (send) sequence number
    pub fn with_send_sn(mut self, send_sn: u16) -> Self {
        self.send_sn = send_sn;
        self
    }
    /// Manually sets the RX (receive) sequence number
    pub fn with_recv_sn(mut self, recv_sn: u16) -> Self {
        self.recv_sn = recv_sn;
        self
    }
    /// Manually sets data to sequental
    pub fn with_seq(mut self) -> Self {
        self.sequental = true;
        self
    }
    /// Sets the COT
    pub fn with_cot(mut self, cot: COT) -> Self {
        self.cot = cot;
        self
    }
    /// Sets the data type
    pub fn with_data_type(mut self, data_type: DataType) -> Self {
        self.data_type = data_type;
        self
    }
    /// Sets the ADSU
    pub fn with_adsu(mut self, adsu: u16) -> Self {
        self.adsu = adsu;
        self
    }
    /// Get the send sequence number
    pub fn send_sn(&self) -> u16 {
        self.send_sn
    }
    /// Get the receive sequence number
    pub fn recv_sn(&self) -> u16 {
        self.recv_sn
    }
    /// Get the data type
    pub fn data_type(&self) -> DataType {
        self.data_type
    }
    /// Get the COT
    pub fn cot(&self) -> COT {
        self.cot
    }
    /// Get the ADSU
    pub fn adsu(&self) -> u16 {
        self.adsu
    }
    /// Is this a test frame
    pub fn is_test(&self) -> bool {
        self.test
    }
    /// Is this a negative frame
    pub fn is_negative(&self) -> bool {
        self.negative
    }
    /// is this a sequental frame
    pub fn is_sequental(&self) -> bool {
        self.sequental
    }
    /// Get the originator
    pub fn originator(&self) -> u8 {
        self.originator
    }
    /// Get the information objects
    pub fn iou(&self) -> &[Iou] {
        &self.iou
    }
    /// Get the information objects as mutable
    pub fn iou_mut(&mut self) -> &mut [Iou] {
        &mut self.iou
    }
    /// Set the test flag
    pub fn with_test(mut self) -> Self {
        self.test = true;
        self
    }
    /// Set the negative flag
    pub fn with_negative(mut self) -> Self {
        self.negative = true;
        self
    }
    /// Set the originator address
    pub fn with_originator(mut self, originator: u8) -> Self {
        self.originator = originator;
        self
    }
    /// Set the information objects from a vector
    pub fn with_iou(mut self, iou: Vec<Iou>) -> Self {
        self.iou = iou;
        self
    }
    /// Clear the information objects
    pub fn clear_iou(&mut self) {
        self.iou.clear();
    }
    /// Append a single IOU
    pub fn append_iou(&mut self, address: u32, value: impl Into<DataBuffer>) {
        self.iou.push(Iou::new(address, value));
    }
    /// Append next IOU in sequence (marks the telegram data as sequental)
    pub fn append_iou_seq(&mut self, value: impl Into<DataBuffer>) {
        self.sequental = true;
        self.iou.push(Iou::new(0, value));
    }
    fn read<R>(mut reader: R) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut control = [0u8; 4];
        reader.read_exact(&mut control)?;
        if control[0] & 0b0000_0001 != 0 {
            return Err(Error::invalid_data("Invalid I-frame control field"));
        }
        let send_sn = u16::from(control[0] >> 1) | (u16::from(control[1]) << 7);
        let recv_sn = u16::from(control[2] >> 1) | (u16::from(control[3]) << 7);

        if send_sn > 32768 {
            return Err(Error::invalid_data("Send sequence number too large"));
        }
        if recv_sn > 32768 {
            return Err(Error::invalid_data("Receive sequence number too large"));
        }

        let mut data_type_buf = [0u8; 1];
        reader.read_exact(&mut data_type_buf)?;
        let data_type: DataType = data_type_buf[0]
            .try_into()
            .map_err(|_| Error::invalid_data("Invalid type identifier"))?;

        let mut iou_len_buf = [0u8; 1];
        reader.read_exact(&mut iou_len_buf)?;
        let sequental = iou_len_buf[0] & 0b1000_0000 != 0;
        let iou_len = usize::from(iou_len_buf[0] & 0b0111_1111);

        if iou_len > usize::from(u8::MAX) {
            return Err(Error::invalid_data("Too many information objects"));
        }

        let mut cot_buf = [0u8; 1];
        reader.read_exact(&mut cot_buf)?;
        let cot_byte = cot_buf[0];

        let negative = cot_byte & 0b1000_0000 != 0;
        let test = cot_byte & 0b0100_0000 != 0;

        let cot = COT::try_from(cot_byte & 0b0011_1111)
            .map_err(|_| Error::invalid_data("Invalid COT"))?;

        let mut originator = [0u8; 1];
        reader.read_exact(&mut originator)?;
        let originator = originator[0];

        let mut adsu = [0u8; 2];
        reader.read_exact(&mut adsu)?;
        let adsu = u16::from_le_bytes(adsu);

        let mut iou = Vec::with_capacity(iou_len);
        let mut first_address = 0;
        for i in 0..iou_len {
            let address = if i == 0 || !sequental {
                let mut address_buf = [0u8; 3];
                reader.read_exact(&mut address_buf)?;
                first_address =
                    u32::from_le_bytes([address_buf[0], address_buf[1], address_buf[2], 0]);
                first_address
            } else {
                first_address + u32::try_from(i).unwrap()
            };
            let mut value = vec![0u8; data_type.size()];
            reader.read_exact(&mut value)?;

            value.resize(MAX_IEC_DATA_LEN, 0);

            iou.push(Iou {
                address,
                value: value.try_into().unwrap(),
            });
        }

        Ok(Self {
            send_sn,
            recv_sn,
            data_type,
            test,
            negative,
            cot,
            originator,
            adsu,
            iou,
            sequental,
        })
    }

    fn write<W>(&self, mut writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        if self.send_sn > 32768 {
            return Err(Error::invalid_data("send sequence number too large"));
        }
        if self.recv_sn > 32768 {
            return Err(Error::invalid_data("receive sequence number too large"));
        }
        if self.iou.len() > usize::from(u8::MAX) {
            return Err(Error::invalid_data("too many information objects"));
        }
        let kind_size = self.data_type.size();
        let mut length = 4; // control fields
        if !self.iou.is_empty() {
            length += 1 // type identifier
            + 1 // number of objects,
            + 1 // T, P/N, COT
            + 1 // originator
            + 2; // ADSU
        }
        if self.sequental {
            length += 3 + kind_size * self.iou.len();
        } else {
            length += (3 + kind_size) * self.iou.len();
        }
        if length > 253 {
            return Err(Error::invalid_data("telegram too long"));
        }
        writer.write_all(&(u8::try_from(length).unwrap()).to_be_bytes())?;
        let control: [u8; 4] = [
            u8::try_from((self.send_sn & 0b0111_1111) << 1).unwrap(),
            u8::try_from(self.send_sn >> 7).unwrap(),
            u8::try_from((self.recv_sn & 0b0111_1111) << 1).unwrap(),
            u8::try_from(self.recv_sn >> 7).unwrap(),
        ];
        writer.write_all(&control)?;
        if self.iou.is_empty() {
            return Ok(());
        }
        writer.write_all(&[self.data_type as u8])?;
        let mut iou_len = u8::try_from(self.iou.len()).unwrap();
        if self.sequental {
            iou_len |= 0b1000_0000;
        }
        writer.write_all(&[iou_len])?;
        let cot_byte = (self.cot as u8)
            | (if self.negative { 0b0100_0000 } else { 0 })
            | (if self.test { 0b1000_0000 } else { 0 });
        writer.write_all(&[cot_byte])?;
        writer.write_all(&[self.originator])?;
        writer.write_all(&self.adsu.to_le_bytes())?;
        for (n, iou) in self.iou.iter().enumerate() {
            if n == 0 || !self.sequental {
                writer.write_all(&iou.address.to_le_bytes()[..3])?;
            }
            writer.write_all(&iou.value[..kind_size])?;
        }
        Ok(())
    }
    /// Convert to Telegram104
    pub fn into_telegram104(self) -> Telegram104 {
        self.into()
    }
}
