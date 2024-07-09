use std::io::{Cursor, Read, Write};

use crate::{
    types::{datatype::DataType, DataBuffer, Iou, COT, MAX_IEC_DATA_LEN},
    Error,
};

const IEC_HEADER: u8 = 0x68;
const IEC_HEADER_FIXED: u8 = 0x10;
const IEC_STOP: u8 = 0x16;

/// IEC 60870-5-101 telegram configuration (used with each telegram)
/// Defaults: link_address_len = 1, originator_address_len = 1, adsu_address_len = 2,
/// iou_address_len = 3
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    link_address_len: u8,
    originator_address_len: u8,
    adsu_address_len: u8,
    iou_address_len: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            link_address_len: 1,
            originator_address_len: 1,
            adsu_address_len: 2,
            iou_address_len: 3,
        }
    }
}

impl Config {
    /// Create a new configuration
    pub fn new() -> Self {
        Self::default()
    }
    /// # Panics
    ///
    /// Panics if `link_address_len` is greater than 2.
    pub fn with_link_address_len(mut self, link_address_len: u8) -> Self {
        assert!(link_address_len <= 2);
        self.link_address_len = link_address_len;
        self
    }
    /// # Panics
    ///
    /// Panics if `originator_address_len` is greater than 1.
    pub fn with_originator_address_len(mut self, originator_address_len: u8) -> Self {
        assert!(originator_address_len <= 1);
        self.originator_address_len = originator_address_len;
        self
    }
    /// # Panics
    ///
    /// Panics if `adsu_address_len` is greater than 2 or less than 1.
    pub fn with_adsu_address_len(mut self, adsu_address_len: u8) -> Self {
        assert!((1..=2).contains(&adsu_address_len));
        self.adsu_address_len = adsu_address_len;
        self
    }
    /// # Panics
    ///
    /// Panics if `iou_address_len` is greater than 3 or less than 1.
    pub fn with_iou_address_len(mut self, iou_address_len: u8) -> Self {
        assert!((1..=3).contains(&iou_address_len));
        self.iou_address_len = iou_address_len;
        self
    }
}

fn buf_checksum(buf: &[u8]) -> u8 {
    buf.iter().fold(0, |acc, &x| acc.wrapping_add(x))
}

/// IEC 60870-5-101 telegram
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct Telegram101 {
    prm: bool,
    fcb_acd: bool,
    fcv_dfc: bool,
    function_code: u8,
    link_address: u32,
    data_type: DataType,
    test: bool,
    negative: bool,
    cot: Option<COT>, // None for fixed length
    originator: u16,
    adsu: u16,
    iou: Vec<Iou>,
    sequental: bool,
    config: Config,
}

impl Telegram101 {
    /// Create a new telegram
    pub fn new(data_type: DataType, cot: COT, adsu: u16, config: Config) -> Self {
        Self {
            prm: false,
            fcb_acd: false,
            fcv_dfc: false,
            function_code: 0,
            link_address: 0,
            data_type,
            test: false,
            negative: false,
            cot: Some(cot),
            originator: 0,
            adsu,
            iou: <_>::default(),
            sequental: false,
            config,
        }
    }
    /// Create a new fixed length telegram
    pub fn new_fixed(config: Config) -> Self {
        Self {
            prm: false,
            fcb_acd: false,
            fcv_dfc: false,
            function_code: 0,
            link_address: 0,
            data_type: DataType::ASDU_TYPEUNDEF,
            test: false,
            negative: false,
            cot: None,
            originator: 0,
            adsu: 0,
            iou: <_>::default(),
            sequental: false,
            config,
        }
    }
    /// Is this a fixed length telegram
    pub fn is_fixed(&self) -> bool {
        self.cot.is_none()
    }
    /// PRM=true if master, PRM=false if slave
    pub fn with_prm(mut self, value: bool) -> Self {
        self.prm = value;
        self
    }
    /// FBC/ACD bit
    pub fn with_fcb_acd(mut self, value: bool) -> Self {
        self.fcb_acd = value;
        self
    }
    /// FCV/DFC bit
    pub fn with_fcv_dfc(mut self, value: bool) -> Self {
        self.fcv_dfc = value;
        self
    }
    /// Function code
    pub fn with_function_code(mut self, value: u8) -> Self {
        self.function_code = value;
        self
    }
    /// Link address
    pub fn with_link_address(mut self, value: u32) -> Self {
        self.link_address = value;
        self
    }
    /// Forcibly set sequential
    pub fn with_seq(mut self) -> Self {
        self.sequental = true;
        self
    }
    /// Type identifier
    pub fn data_type(&self) -> DataType {
        self.data_type
    }
    /// COT
    pub fn cot(&self) -> COT {
        self.cot.unwrap_or(COT::Unused)
    }
    /// ADSU
    pub fn adsu(&self) -> u16 {
        self.adsu
    }
    /// Is this a test telegram
    pub fn is_test(&self) -> bool {
        self.test
    }
    /// Negative=True if negative confirmation
    pub fn is_negative(&self) -> bool {
        self.negative
    }
    /// Is the data sequental
    pub fn is_sequental(&self) -> bool {
        self.sequental
    }
    /// Link address
    pub fn link_address(&self) -> u32 {
        self.link_address
    }
    /// Originator address
    pub fn originator(&self) -> u16 {
        self.originator
    }
    /// Information objects
    pub fn iou(&self) -> &[Iou] {
        &self.iou
    }
    /// Mutable information objects
    pub fn iou_mut(&mut self) -> &mut [Iou] {
        &mut self.iou
    }
    /// Clear information objects
    pub fn clear_iou(&mut self) {
        self.iou.clear();
    }
    /// Set test bit
    pub fn with_test(mut self) -> Self {
        self.test = true;
        self
    }
    /// Set negative bit
    pub fn with_negative(mut self) -> Self {
        self.negative = true;
        self
    }
    /// Set originator address
    pub fn with_originator(mut self, originator: u16) -> Self {
        self.originator = originator;
        self
    }
    /// Set IOU from Vec
    pub fn with_iou(mut self, iou: Vec<Iou>) -> Self {
        self.iou = iou;
        self
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
    #[allow(clippy::too_many_lines)]
    fn read_variable_length<R>(mut reader: R, config: Config) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buf = [0; 4];
        reader.read_exact(&mut buf)?;
        let length = buf[0];
        if length != buf[1] {
            return Err(Error::invalid_data("invalid length"));
        }
        if length > 253 {
            return Err(Error::invalid_data("telegram too long"));
        }
        if buf[2] != IEC_HEADER {
            return Err(Error::invalid_data("invalid header"));
        }
        let mut buf = vec![0u8; usize::from(length)];
        reader.read_exact(&mut buf)?;
        let mut tail = [0; 2];
        reader.read_exact(&mut tail)?;
        if tail[0] != buf_checksum(&buf) {
            return Err(Error::invalid_data("invalid checksum"));
        }
        if tail[1] != IEC_STOP {
            return Err(Error::invalid_data("invalid stop"));
        }
        let mut frame = Cursor::new(buf);
        let mut control_buf = [0; 1];
        frame.read_exact(&mut control_buf)?;
        let mut link_address_buf = vec![0; usize::from(config.link_address_len)];
        if link_address_buf.capacity() > 0 {
            frame.read_exact(&mut link_address_buf)?;
        }
        link_address_buf.resize(4, 0);
        let link_address = u32::from_le_bytes(link_address_buf.try_into().unwrap());
        let mut buf = [0u8; 3];
        frame.read_exact(&mut buf)?;
        let data_type = DataType::try_from(buf[0])
            .map_err(|_| Error::invalid_data(format!("invalid data_type {}", buf[0])))?;
        let iou_len = usize::from(buf[1] & 0b0111_1111);
        let sequental = buf[1] & 0b1000_0000 != 0;
        let cot = COT::try_from(buf[2] & 0b0011_1111)
            .map_err(|_| Error::invalid_data(format!("invalid cot {}", buf[2])))?;
        let test = buf[2] & 0b1000_0000 != 0;
        let negative = buf[2] & 0b0100_0000 != 0;
        let mut originator_buf = vec![0; usize::from(config.originator_address_len)];
        if originator_buf.capacity() > 0 {
            frame.read_exact(&mut originator_buf)?;
        }
        originator_buf.resize(2, 0);
        let originator = u16::from_le_bytes(originator_buf.try_into().unwrap());
        let mut adsu_buf = vec![0; usize::from(config.adsu_address_len)];
        frame.read_exact(&mut adsu_buf)?;
        adsu_buf.resize(2, 0);
        let adsu = u16::from_le_bytes(adsu_buf.try_into().unwrap());
        let mut iou = Vec::with_capacity(iou_len);
        let mut first_address = 0;
        for i in 0..iou_len {
            let address = if i == 0 || !sequental {
                let mut address_buf = vec![0; usize::from(config.iou_address_len)];
                reader.read_exact(&mut address_buf)?;
                address_buf.resize(4, 0);
                first_address = u32::from_le_bytes(address_buf.try_into().unwrap());
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
            prm: control_buf[0] & 0b0100_0000 != 0,
            fcb_acd: control_buf[0] & 0b0010_0000 != 0,
            fcv_dfc: control_buf[0] & 0b0001_0000 != 0,
            function_code: control_buf[0] & 0b0000_1111,
            link_address,
            data_type: data_type,
            test,
            negative,
            cot: Some(cot),
            originator,
            adsu,
            iou,
            sequental,
            config,
        })
    }
    fn read_fixed_length<R>(mut reader: R, config: Config) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut buf = vec![0u8; 1 + usize::from(config.link_address_len) + 2];
        reader.read_exact(&mut buf)?;
        let control = buf[0];
        let mut link_address_buf = buf[1..=usize::from(config.link_address_len)].to_vec();
        link_address_buf.resize(4, 0);
        let link_address = u32::from_le_bytes(link_address_buf.try_into().unwrap());
        let checksum = buf_checksum(&buf[..=usize::from(config.link_address_len)]);
        if checksum != buf[1 + usize::from(config.link_address_len)] {
            return Err(Error::invalid_data("invalid checksum"));
        }
        let stop = buf[1 + usize::from(config.link_address_len) + 1];
        if stop != IEC_STOP {
            return Err(Error::invalid_data("invalid stop"));
        }
        Ok(Self {
            prm: control & 0b0100_0000 != 0,
            fcb_acd: control & 0b0010_0000 != 0,
            fcv_dfc: control & 0b0001_0000 != 0,
            function_code: control & 0b0000_1111,
            link_address,
            data_type: DataType::ASDU_TYPEUNDEF,
            test: false,
            negative: false,
            cot: None,
            originator: 0,
            adsu: 0,
            iou: <_>::default(),
            sequental: false,
            config,
        })
    }
    /// Read a telegram from a reader
    pub fn read<R>(mut reader: R, config: Config) -> Result<Self, Error>
    where
        R: Read,
    {
        let mut frame_header = [0; 1];
        reader.read_exact(&mut frame_header)?;
        match frame_header[0] {
            IEC_HEADER => Self::read_variable_length(reader, config),
            IEC_HEADER_FIXED => Self::read_fixed_length(reader, config),
            _ => Err(Error::invalid_data("invalid header")),
        }
    }

    fn control_field(&self) -> u8 {
        let mut control = 0;
        if self.prm {
            control |= 0b0100_0000;
        }
        if self.fcb_acd {
            control |= 0b0010_0000;
        }
        if self.fcv_dfc {
            control |= 0b0001_0000;
        }
        control |= self.function_code & 0b0000_1111;
        control
    }

    /// Write the telegram to a writer
    ///
    /// # Panics
    ///
    /// Should not panic
    pub fn write<W>(&self, mut writer: W) -> Result<(), Error>
    where
        W: Write,
    {
        if let Some(cot) = self.cot {
            // variable length
            if self.iou.len() > usize::from(u8::MAX) {
                return Err(Error::invalid_data("too many information objects"));
            }
            let mut capacity = 1 // control field
            + usize::from(self.config.link_address_len) // link address
            + 1 // data_type
            + 1 // iou length
            + 1 // cot
            + usize::from(self.config.originator_address_len) // originator
            + usize::from(self.config.adsu_address_len) // adsu
            ;
            let kind_size = self.data_type.size();
            if self.sequental {
                capacity += usize::from(self.config.iou_address_len) + kind_size * self.iou.len();
            } else {
                capacity += (usize::from(self.config.iou_address_len) + kind_size) * self.iou.len();
            }
            let mut buf = Vec::<u8>::with_capacity(capacity);
            buf.push(self.control_field());
            buf.extend(
                &self.link_address.to_le_bytes()[..usize::from(self.config.link_address_len)],
            );
            buf.push(self.data_type as u8);
            let mut iou_len = u8::try_from(self.iou.len()).unwrap();
            if self.sequental {
                iou_len |= 0b1000_0000;
            }
            buf.push(iou_len);
            let cot_byte = (cot as u8)
                | (if self.negative { 0b0100_0000 } else { 0 })
                | (if self.test { 0b1000_0000 } else { 0 });
            buf.push(cot_byte);
            buf.extend(
                &self.originator.to_le_bytes()[..usize::from(self.config.originator_address_len)],
            );
            buf.extend(&self.adsu.to_le_bytes()[..usize::from(self.config.adsu_address_len)]);
            for (n, iou) in self.iou.iter().enumerate() {
                if n == 0 || !self.sequental {
                    buf.extend(
                        &iou.address.to_le_bytes()[..usize::from(self.config.iou_address_len)],
                    );
                }
                buf.extend(&iou.value[..kind_size]);
            }
            let length = buf.len();
            if length > 253 {
                return Err(Error::invalid_data("telegram too long"));
            }
            writer.write_all(&[
                IEC_HEADER,
                u8::try_from(length).unwrap(),
                u8::try_from(length).unwrap(),
                IEC_HEADER,
            ])?;
            writer.write_all(&buf)?;
            writer.write_all(&[buf_checksum(&buf)])?;
        } else {
            // fixed length
            let mut buf = Vec::<u8>::with_capacity(1 + usize::from(self.config.link_address_len));
            buf.push(self.control_field());
            buf.extend(
                &self.link_address.to_le_bytes()[..usize::from(self.config.link_address_len)],
            );
            writer.write_all(&[IEC_HEADER_FIXED])?;
            writer.write_all(&buf)?;
            writer.write_all(&[buf_checksum(&buf)])?;
        }
        writer.write_all(&[IEC_STOP])?;
        Ok(())
    }
}
