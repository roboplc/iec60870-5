use std::convert::TryFrom;

use crate::Error;

/// Cause of transmission
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum COT {
    /// Unused
    Unused = 0,
    /// Cyclic
    Cyclic = 1,
    /// Background
    Background = 2,
    /// Spontaneous
    Spontan = 3,
    /// Initiated
    Init = 4,
    /// Request
    Req = 5,
    /// Activation
    Act = 6,
    /// Activation confirmation
    ActCon = 7,
    /// Deactivation
    Deact = 8,
    /// Deactivation confirmation
    DeactCon = 9,
    /// Activation termination
    ActTerm = 10,
    /// Return remote
    RetRem = 11,
    /// Return local
    RetLoc = 12,
    /// File transfer
    File = 13,
    /// Reserved
    COT14 = 14,
    /// Reserved
    COT15 = 15,
    /// Reserved
    COT16 = 16,
    /// Reserved
    COT17 = 17,
    /// Reserved
    COT18 = 18,
    /// Reserved
    COT19 = 19,
    /// Interrogation
    Inrogen = 20,
    /// Interrogation
    Inro1 = 21,
    /// Interrogation
    Inro2 = 22,
    /// Interrogation
    Inro3 = 23,
    /// Interrogation
    Inro4 = 24,
    /// Interrogation
    Inro5 = 25,
    /// Interrogation
    Inro6 = 26,
    /// Interrogation
    Inro7 = 27,
    /// Interrogation
    Inro8 = 28,
    /// Interrogation
    Inro9 = 29,
    /// Interrogation
    Inro10 = 30,
    /// Interrogation
    Inro11 = 31,
    /// Interrogation
    Inro12 = 32,
    /// Interrogation
    Inro13 = 33,
    /// Interrogation
    Inro14 = 34,
    /// Interrogation
    Inro15 = 35,
    /// Interrogation
    Inro16 = 36,
    /// Request for counter interrogation
    ReqCoGen = 37,
    /// Request for counter interrogation
    ReqCo1 = 38,
    /// Request for counter interrogation
    ReqCo2 = 39,
    /// Request for counter interrogation
    ReqCo3 = 40,
    /// Request for counter interrogation
    ReqCo4 = 41,
    /// Reserved
    COT42 = 42,
    /// Reserved
    COT43 = 43,
    /// Unknown type error
    UnknownType = 44,
    /// Unknown cause error
    UnknownCause = 45,
    /// Unknown ASDU address error
    UnknownAsduAddress = 46,
    /// Unknown object address error
    UnknownObjectAddress = 47,
    /// Reserved
    COT48 = 48,
    /// Reserved
    COT49 = 49,
    /// Reserved
    COT50 = 50,
    /// Reserved
    COT51 = 51,
    /// Reserved
    COT52 = 52,
    /// Reserved
    COT53 = 53,
    /// Reserved
    COT54 = 54,
    /// Reserved
    COT55 = 55,
    /// Reserved
    COT56 = 56,
    /// Reserved
    COT57 = 57,
    /// Reserved
    COT58 = 58,
    /// Reserved
    COT59 = 59,
    /// Reserved
    COT60 = 60,
    /// Reserved
    COT61 = 61,
    /// Reserved
    COT62 = 62,
    /// Reserved
    COT63 = 63,
}

impl TryFrom<u8> for COT {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(COT::Unused),
            1 => Ok(COT::Cyclic),
            2 => Ok(COT::Background),
            3 => Ok(COT::Spontan),
            4 => Ok(COT::Init),
            5 => Ok(COT::Req),
            6 => Ok(COT::Act),
            7 => Ok(COT::ActCon),
            8 => Ok(COT::Deact),
            9 => Ok(COT::DeactCon),
            10 => Ok(COT::ActTerm),
            11 => Ok(COT::RetRem),
            12 => Ok(COT::RetLoc),
            13 => Ok(COT::File),
            14 => Ok(COT::COT14),
            15 => Ok(COT::COT15),
            16 => Ok(COT::COT16),
            17 => Ok(COT::COT17),
            18 => Ok(COT::COT18),
            19 => Ok(COT::COT19),
            20 => Ok(COT::Inrogen),
            21 => Ok(COT::Inro1),
            22 => Ok(COT::Inro2),
            23 => Ok(COT::Inro3),
            24 => Ok(COT::Inro4),
            25 => Ok(COT::Inro5),
            26 => Ok(COT::Inro6),
            27 => Ok(COT::Inro7),
            28 => Ok(COT::Inro8),
            29 => Ok(COT::Inro9),
            30 => Ok(COT::Inro10),
            31 => Ok(COT::Inro11),
            32 => Ok(COT::Inro12),
            33 => Ok(COT::Inro13),
            34 => Ok(COT::Inro14),
            35 => Ok(COT::Inro15),
            36 => Ok(COT::Inro16),
            37 => Ok(COT::ReqCoGen),
            38 => Ok(COT::ReqCo1),
            39 => Ok(COT::ReqCo2),
            40 => Ok(COT::ReqCo3),
            41 => Ok(COT::ReqCo4),
            42 => Ok(COT::COT42),
            43 => Ok(COT::COT43),
            44 => Ok(COT::UnknownType),
            45 => Ok(COT::UnknownCause),
            46 => Ok(COT::UnknownAsduAddress),
            47 => Ok(COT::UnknownObjectAddress),
            48 => Ok(COT::COT48),
            49 => Ok(COT::COT49),
            50 => Ok(COT::COT50),
            51 => Ok(COT::COT51),
            52 => Ok(COT::COT52),
            53 => Ok(COT::COT53),
            54 => Ok(COT::COT54),
            55 => Ok(COT::COT55),
            56 => Ok(COT::COT56),
            57 => Ok(COT::COT57),
            58 => Ok(COT::COT58),
            59 => Ok(COT::COT59),
            60 => Ok(COT::COT60),
            61 => Ok(COT::COT61),
            62 => Ok(COT::COT62),
            63 => Ok(COT::COT63),
            v => Err(Error::COT(v)),
        }
    }
}
