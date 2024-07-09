#![allow(non_camel_case_types)]

use std::time::Duration;

use bma_ts::Timestamp;
use chrono::{Datelike, Timelike};

use crate::Error;

const ERR_TIME_CONVERSION_FAILED: &str = "Time conversion failed";

/// CP16Time2a time type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct CP16Time2a {
    /// Milliseconds (0-59999)
    pub ms: u16,
}

impl TryFrom<Duration> for CP16Time2a {
    type Error = Error;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let ms = value.as_millis();
        if ms > 59999 {
            return Err(Error::Overflow);
        }
        Ok(CP16Time2a {
            ms: u16::try_from(ms).unwrap(),
        })
    }
}

impl From<CP16Time2a> for Duration {
    fn from(value: CP16Time2a) -> Duration {
        Duration::from_millis(u64::from(value.ms))
    }
}

impl From<[u8; 2]> for CP16Time2a {
    fn from(buf: [u8; 2]) -> Self {
        let ms = u16::from_le_bytes(buf);
        CP16Time2a { ms }
    }
}

impl From<CP16Time2a> for [u8; 2] {
    fn from(data: CP16Time2a) -> [u8; 2] {
        data.ms.to_le_bytes()
    }
}

/// CP24Time2a time type
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
pub struct CP24Time2a {
    /// Milliseconds (0-59999)
    pub ms: u16,
    /// Minutes (0-59)
    pub min: u8,
    /// Invalid flag
    pub iv: bool,
}

impl TryFrom<Duration> for CP24Time2a {
    type Error = Error;

    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        let ms = value.as_millis();
        if ms >= 60 * 60000 {
            return Err(Error::Overflow);
        }
        Ok(CP24Time2a {
            ms: u16::try_from(ms % (60 * 1000)).unwrap(),
            min: u8::try_from(ms / (60 * 1000)).unwrap(),
            iv: false,
        })
    }
}

impl From<CP24Time2a> for Duration {
    fn from(value: CP24Time2a) -> Duration {
        Duration::from_millis(u64::from(value.min) * 60_000 + u64::from(value.ms))
    }
}

impl TryFrom<Timestamp> for CP24Time2a {
    type Error = Error;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let t = value.try_into_datetime_local().map_err(Error::conversion)?;
        let mut ms = t.nanosecond() / 1_000_000 + t.second() * 1000;
        if ms > 59_999 {
            ms = 59_999;
        }
        let mut min = t.minute();
        if min > 59 {
            min = 59;
        }
        Ok(CP24Time2a {
            ms: u16::try_from(ms).unwrap(),
            min: u8::try_from(min).unwrap(),
            iv: false,
        })
    }
}

impl TryFrom<CP24Time2a> for Timestamp {
    type Error = Error;

    fn try_from(value: CP24Time2a) -> Result<Self, Self::Error> {
        let seconds = value.ms / 1000;
        let ms = value.ms % 1000;
        let t = chrono::Local::now()
            .with_minute(u32::from(value.min))
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_second(seconds.into())
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_nanosecond(u32::from(ms) * 1_000_000)
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?;
        t.try_into().map_err(Error::conversion)
    }
}

impl From<[u8; 3]> for CP24Time2a {
    fn from(buf: [u8; 3]) -> Self {
        let ms = u16::from_le_bytes([buf[0], buf[1]]);
        let min = buf[2] & 0b0011_1111;
        let iv = min & 0b1000_0000 != 0;
        CP24Time2a { ms, min, iv }
    }
}

impl From<CP24Time2a> for [u8; 3] {
    fn from(data: CP24Time2a) -> [u8; 3] {
        let mut buf = [0; 3];
        buf[0..2].copy_from_slice(&data.ms.to_le_bytes());
        buf[2] = data.min | (u8::from(data.iv) << 7);
        buf
    }
}

/// Day of week
#[derive(Debug, Clone, Eq, PartialEq, Default)]
#[repr(u8)]
pub enum DayOfWeek {
    /// Monday
    Monday = 1,
    /// Tuesday
    Tuesday = 2,
    /// Wednesday
    Wednesday = 3,
    /// Thursday
    Thursday = 4,
    /// Friday
    Friday = 5,
    /// Saturday
    Saturday = 6,
    /// Sunday
    #[default]
    Sunday = 7,
}

impl From<u8> for DayOfWeek {
    fn from(value: u8) -> Self {
        match value {
            1 => DayOfWeek::Monday,
            2 => DayOfWeek::Tuesday,
            3 => DayOfWeek::Wednesday,
            4 => DayOfWeek::Thursday,
            5 => DayOfWeek::Friday,
            6 => DayOfWeek::Saturday,
            _ => DayOfWeek::Sunday,
        }
    }
}

/// CP56Time2a time type
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct CP56Time2a {
    /// Milliseconds (0-59999)
    pub ms: u16,
    /// Invalid flag
    pub iv: bool,
    /// Minutes (0-59)
    pub min: u8,
    /// Summer time flag
    pub su: bool,
    /// Hours (0-23)
    pub hour: u8,
    /// Day of week
    pub dow: DayOfWeek,
    /// Day of month (1-31)
    pub day: u8,
    /// Month (1-12)
    pub month: u8,
    /// Year (0-99)
    pub year: u8,
}

impl From<[u8; 7]> for CP56Time2a {
    fn from(buf: [u8; 7]) -> Self {
        let ms = u16::from_le_bytes([buf[0], buf[1]]);
        let iv = buf[2] & 0b1000_0000 != 0;
        let min = buf[2] & 0b0011_1111;
        let summer_time = buf[3] & 0b1000_0000 != 0;
        let hour = buf[3] & 0b0001_1111;
        let dow = (buf[4] & 0b1110_0000) >> 5;
        let day = buf[4] & 0b0001_1111;
        let month = buf[5] & 0b0000_1111;
        let year = buf[6] & 0b0011_1111;
        CP56Time2a {
            ms,
            iv,
            min,
            su: summer_time,
            hour,
            dow: dow.into(),
            day,
            month,
            year,
        }
    }
}

impl From<CP56Time2a> for [u8; 7] {
    fn from(data: CP56Time2a) -> [u8; 7] {
        let mut buf = [0; 7];
        buf[0..2].copy_from_slice(&data.ms.to_le_bytes());
        buf[2] = data.min | (u8::from(data.iv) << 7);
        buf[3] = data.hour | (u8::from(data.su) << 7);
        buf[4] = (data.dow as u8) << 5 | data.day;
        buf[5] = data.month;
        buf[6] = data.year;
        buf
    }
}

impl TryFrom<Timestamp> for CP56Time2a {
    type Error = Error;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let t = value.try_into_datetime_local().map_err(Error::conversion)?;
        let mut ms = t.nanosecond() / 1_000_000 + t.second() * 1000;
        if ms > 59_999 {
            ms = 59_999;
        }
        let mut min = t.minute();
        if min > 59 {
            min = 59;
        }
        let mut hour = t.hour();
        if hour > 23 {
            hour = 23;
        }
        let offset = t.offset().local_minus_utc();
        let standard_offset = t
            .with_month(1)
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_day(1)
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .offset()
            .local_minus_utc();
        let summer_time = offset != standard_offset;
        Ok(CP56Time2a {
            ms: u16::try_from(ms).unwrap(),
            iv: false,
            min: u8::try_from(min).unwrap(),
            su: summer_time,
            hour: u8::try_from(hour).unwrap(),
            dow: u8::try_from(t.weekday().number_from_monday())
                .map_err(Error::conversion)?
                .into(),
            day: u8::try_from(t.day()).map_err(Error::conversion)?,
            month: u8::try_from(t.month()).map_err(Error::conversion)?,
            year: u8::try_from(t.year() % 100).map_err(Error::conversion)?,
        })
    }
}

impl TryFrom<CP56Time2a> for Timestamp {
    type Error = Error;

    fn try_from(value: CP56Time2a) -> Result<Self, Self::Error> {
        let seconds = value.ms / 1000;
        let ms = value.ms % 1000;
        let t = chrono::Local::now()
            .with_minute(u32::from(value.min))
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_second(seconds.into())
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_nanosecond(u32::from(ms) * 1_000_000)
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_hour(u32::from(value.hour))
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_day(u32::from(value.day))
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_month(u32::from(value.month))
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?
            .with_year(2000 + i32::from(value.year))
            .ok_or_else(|| Error::conversion(ERR_TIME_CONVERSION_FAILED))?;
        t.try_into().map_err(Error::conversion)
    }
}

#[cfg(test)]
mod tests {
    use bma_ts::Timestamp;
    use chrono::{Local, TimeZone, Timelike};
    use std::time::Duration;

    use super::{CP16Time2a, CP24Time2a, CP56Time2a};

    #[test]
    fn test_cp16time2a_from_duration() {
        let duration = Duration::from_millis(12345);
        let cp16time2a = CP16Time2a::try_from(duration).unwrap();
        assert_eq!(cp16time2a.ms, 12345);
    }

    #[test]
    fn test_duration_from_cp16time2a() {
        let cp16time2a = CP16Time2a { ms: 12345 };
        let duration: Duration = cp16time2a.into();
        assert_eq!(duration.as_millis(), 12345);
    }

    #[test]
    fn test_cp24time2a_from_duration() {
        let duration = Duration::from_millis(1_234_567);
        let cp24time2a = CP24Time2a::try_from(duration).unwrap();
        assert_eq!(cp24time2a.ms, 34567);
        assert_eq!(cp24time2a.min, 20);
        assert!(!cp24time2a.iv);
    }

    #[test]
    fn test_duration_from_cp24time2a() {
        let cp24time2a = CP24Time2a {
            ms: 34567,
            min: 20,
            iv: false,
        };
        let duration: Duration = cp24time2a.into();
        assert_eq!(duration.as_millis(), 1_234_567);
    }

    #[test]
    fn test_cp56time2a_from_timestamp() {
        let t: Timestamp = Local
            .with_ymd_and_hms(2024, 7, 1, 12, 34, 56)
            .unwrap()
            .with_nanosecond(789_000_000)
            .unwrap()
            .try_into()
            .unwrap();
        let cp56time2a = CP56Time2a::try_from(t).unwrap();
        assert_eq!(cp56time2a.ms, 56789);
        assert_eq!(cp56time2a.min, 34);
        assert_eq!(cp56time2a.hour, 12);
        assert_eq!(cp56time2a.day, 1);
        assert_eq!(cp56time2a.month, 7);
        assert_eq!(cp56time2a.year, 24);
        assert_eq!(cp56time2a.dow, 1.into());
        assert!(!cp56time2a.iv);
        //assert!(cp56time2a.su); // removed from CI-tests due to CI-server time zones
    }

    #[test]
    fn test_timestamp_from_cp56time2a() {
        let cp56time2a = CP56Time2a {
            ms: 56789,
            min: 34,
            iv: false,
            hour: 12,
            day: 1,
            month: 7,
            year: 24,
            su: false,
            dow: 1.into(),
        };
        let timestamp = Timestamp::try_from(cp56time2a).unwrap();
        let t: Timestamp = Local
            .with_ymd_and_hms(2024, 7, 1, 12, 34, 56)
            .unwrap()
            .with_nanosecond(789_000_000)
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(timestamp, t);
    }
    #[test]
    fn test_cp16time2a_from_bytes() {
        let bytes: [u8; 2] = [0x39, 0x30];
        let cp16time2a: CP16Time2a = bytes.into();
        assert_eq!(cp16time2a.ms, 12345);
    }

    #[test]
    fn test_bytes_from_cp16time2a() {
        let cp16time2a = CP16Time2a { ms: 12345 };
        let bytes: [u8; 2] = cp16time2a.into();
        assert_eq!(bytes, [0x39, 0x30]);
    }

    #[test]
    fn test_cp24time2a_from_bytes() {
        let bytes: [u8; 3] = [0x07, 0x87, 0x14];
        let cp24time2a: CP24Time2a = bytes.into();
        assert_eq!(cp24time2a.ms, 34567);
        assert_eq!(cp24time2a.min, 20);
        assert!(!cp24time2a.iv);
    }

    #[test]
    fn test_bytes_from_cp24time2a() {
        let cp24time2a = CP24Time2a {
            ms: 34567,
            min: 20,
            iv: false,
        };
        let bytes: [u8; 3] = cp24time2a.into();
        assert_eq!(bytes, [0x07, 0x87, 0x14]);
    }

    #[test]
    fn test_cp56time2a_from_bytes() {
        let bytes: [u8; 7] = [0xD5, 0xDD, 0x22, 0x92, 0b0101_1110, 0x07, 0x18];
        let cp56time2a: CP56Time2a = bytes.into();
        assert_eq!(cp56time2a.ms, 56789);
        assert_eq!(cp56time2a.min, 34);
        assert_eq!(cp56time2a.hour, 18);
        assert_eq!(cp56time2a.day, 30);
        assert_eq!(cp56time2a.month, 7);
        assert_eq!(cp56time2a.year, 24);
        assert_eq!(cp56time2a.dow, 2.into());
        assert!(!cp56time2a.iv);
        assert!(cp56time2a.su);
    }

    #[test]
    fn test_bytes_from_cp56time2a() {
        let cp56time2a = CP56Time2a {
            ms: 56789,
            min: 34,
            iv: false,
            hour: 18,
            day: 30,
            month: 7,
            year: 24,
            su: true,
            dow: 2.into(),
        };
        let bytes: [u8; 7] = cp56time2a.into();
        assert_eq!(bytes, [0xD5, 0xDD, 0x22, 0x92, 0b0101_1110, 0x07, 0x18]);
    }
}
