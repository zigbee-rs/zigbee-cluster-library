//! Data types
//!
//! See section 2.6.2
use byte::{BytesExt, TryRead, TryWrite};

#[derive(Debug, PartialEq)]
pub enum ZclDataType<'a> {
    NoData,
    Data(DataN),
    Bool(bool),
    Bitmap(BitmapN),
    UnsignedInt(UnsignedN),
    SignedInt(SignedN),
    Enum(EnumN),
    Float(FloatN),
    String(ZclString<'a>),
    Array(&'a [ZclDataType<'a>]),
    Structure(&'a [ZclDataType<'a>]),
    Set(&'a [ZclDataType<'a>]),
    Bag(&'a [ZclDataType<'a>]),
    Time(TimeType),
    Identifier(IdentifierType),
    Misc(MiscType<'a>),
    Unknown,
}

impl<'a> TryRead<'a, u8> for ZclDataType<'a> {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x00 => ZclDataType::NoData,
            0x08 | 0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x0E | 0x0F => {
                ZclDataType::Data(bytes.read_with(offset, identifier)?)
            }
            0x10 => ZclDataType::Bool(bytes.read(offset)?),
            0x18 | 0x19 | 0x1A | 0x1B | 0x1C | 0x1D | 0x1E | 0x1F => {
                ZclDataType::Bitmap(bytes.read_with(offset, identifier)?)
            }
            0x20 | 0x21 | 0x22 | 0x23 | 0x24 | 0x25 | 0x26 | 0x27 => {
                ZclDataType::UnsignedInt(bytes.read_with(offset, identifier)?)
            }
            0x28 | 0x29 | 0x2A | 0x2B | 0x2C | 0x2D | 0x2E | 0x2F => {
                ZclDataType::SignedInt(bytes.read_with(offset, identifier)?)
            }
            0x30 | 0x31 => ZclDataType::Enum(bytes.read_with(offset, identifier)?),
            0x38 | 0x39 | 0x3A => ZclDataType::Float(bytes.read_with(offset, identifier)?),
            0x41 | 0x42 | 0x43 | 0x44 => ZclDataType::String(bytes.read_with(offset, identifier)?),
            //0x48 => ZclDataType::Array(bytes.read_with(offset, identifier)?),
            //0x4C => ZclDataType::Structure(bytes.read_with(offset, identifier)?),
            //0x50 => ZclDataType::Set(bytes.read_with(offset, identifier)?),
            //0x51 => ZclDataType::Bag(bytes.read_with(offset, identifier)?),
            0xE0 | 0xE1 | 0xE2 => ZclDataType::Time(bytes.read_with(offset, identifier)?),
            0xE8 | 0xE9 | 0xEA => ZclDataType::Identifier(bytes.read_with(offset, identifier)?),
            0xF0 | 0xF1 => ZclDataType::Misc(bytes.read_with(offset, identifier)?),

            _ => ZclDataType::Unknown,
        };

        Ok((v, *offset))
    }
}

impl TryWrite<u8> for ZclDataType<'_> {
    fn try_write(self, _bytes: &mut [u8], _identifier: u8) -> Result<usize, ::byte::Error> {
        unimplemented!()
    }
}

/// 2.6.2.2 General Data
#[derive(Debug, PartialEq, Eq)]
pub enum DataN {
    Data8(u8),
    Data16(u16),
    Data24(u32),
    Data32(u32),
    Data40(u64),
    Data48(u64),
    Data56(u64),
    Data64(u64),
}

impl<'a> TryRead<'a, u8> for DataN {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x08 => DataN::Data8(bytes.read(offset)?),
            0x09 => DataN::Data16(bytes.read(offset)?),
            0x0A => DataN::Data24(bytes.read(offset)?),
            0x0B => DataN::Data32(bytes.read(offset)?),
            0x0C => DataN::Data40(bytes.read(offset)?),
            0x0D => DataN::Data48(bytes.read(offset)?),
            0x0E => DataN::Data56(bytes.read(offset)?),
            0x0F => DataN::Data64(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid DataN",
                })
            }
        };

        Ok((v, *offset))
    }
}

/// 2.6.2.4 Bitmap
#[derive(Debug, PartialEq, Eq)]
pub enum BitmapN {
    Bitmap8(u8),
    Bitmap16(u16),
    Bitmap24(u32),
    Bitmap32(u32),
    Bitmap40(u64),
    Bitmap48(u64),
    Bitmap56(u64),
    Bitmap64(u64),
}

impl<'a> TryRead<'a, u8> for BitmapN {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x08 => BitmapN::Bitmap8(bytes.read(offset)?),
            0x09 => BitmapN::Bitmap16(bytes.read(offset)?),
            0x0A => BitmapN::Bitmap24(bytes.read(offset)?),
            0x0B => BitmapN::Bitmap32(bytes.read(offset)?),
            0x0C => BitmapN::Bitmap40(bytes.read(offset)?),
            0x0D => BitmapN::Bitmap48(bytes.read(offset)?),
            0x0E => BitmapN::Bitmap56(bytes.read(offset)?),
            0x0F => BitmapN::Bitmap64(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid BitmapN",
                })
            }
        };

        Ok((v, *offset))
    }
}

/// 2.6.2.5 Unsigned Integer
#[derive(Debug, PartialEq, Eq)]
pub enum UnsignedN {
    Uint8(u8),
    Uint16(u16),
    Uint24(u32),
    Uint32(u32),
    Uint40(u64),
    Uint48(u64),
    Uint56(u64),
    Uint64(u64),
}

impl<'a> TryRead<'a, u8> for UnsignedN {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x20 => UnsignedN::Uint8(bytes.read(offset)?),
            0x21 => UnsignedN::Uint16(bytes.read(offset)?),
            0x22 => UnsignedN::Uint24(bytes.read(offset)?),
            0x23 => UnsignedN::Uint32(bytes.read(offset)?),
            0x24 => UnsignedN::Uint40(bytes.read(offset)?),
            0x25 => UnsignedN::Uint48(bytes.read(offset)?),
            0x26 => UnsignedN::Uint56(bytes.read(offset)?),
            0x27 => UnsignedN::Uint64(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid UnsignedN",
                })
            }
        };

        Ok((v, *offset))
    }
}

/// 2.6.2.6 Signed Integer
#[derive(Debug, PartialEq, Eq)]
pub enum SignedN {
    Int8(i8),
    Int16(i16),
    Int24(i32),
    Int32(i32),
    Int40(i64),
    Int48(i64),
    Int56(i64),
    Int64(i64),
}

impl<'a> TryRead<'a, u8> for SignedN {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x28 => SignedN::Int8(bytes.read(offset)?),
            0x29 => SignedN::Int16(bytes.read(offset)?),
            0x2A => SignedN::Int24(bytes.read(offset)?),
            0x2B => SignedN::Int32(bytes.read(offset)?),
            0x2C => SignedN::Int40(bytes.read(offset)?),
            0x2D => SignedN::Int48(bytes.read(offset)?),
            0x2E => SignedN::Int56(bytes.read(offset)?),
            0x2F => SignedN::Int64(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid SignedN",
                })
            }
        };

        Ok((v, *offset))
    }
}

//// 2.6.2.7 Enumeration
#[derive(Debug, PartialEq, Eq)]
pub enum EnumN {
    Enum8(u8),
    Enum16(u16),
}

impl<'a> TryRead<'a, u8> for EnumN {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x30 => EnumN::Enum8(bytes.read(offset)?),
            0x31 => EnumN::Enum16(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid EnumN",
                })
            }
        };

        Ok((v, *offset))
    }
}

#[derive(Debug, PartialEq)]
pub enum FloatN {
    /// 2.6.2.8 Semi-precision based on IEEE-754
    Semi(u16),
    /// 2.6.2.9 Single precision
    Single(f32),
    /// 2.6.2.10 Double precision
    Double(f64),
}

impl<'a> TryRead<'a, u8> for FloatN {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0x38 => FloatN::Semi(bytes.read(offset)?),
            0x39 => FloatN::Single(bytes.read(offset)?),
            0x3A => FloatN::Double(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid FloatN",
                })
            }
        };

        Ok((v, *offset))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ZclString<'a> {
    /// 2.6.2.12 Octet String
    OctetString(&'a [u8]),
    /// 2.6.2.13 Character String
    CharString(&'a str),
    /// 2.6.2.14 Long Octet String
    LongOctetString(&'a [u8]),
    /// 2.6.2.15 Long Character String
    LongCharString(&'a str),
}

impl<'a> TryRead<'a, u8> for ZclString<'a> {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            // 0x41 => ZclString::OctetString(bytes.read::<&'a [u8]>(offset)?),
            0x42 => ZclString::CharString(bytes.read::<&'a str>(offset)?),
            // 0x43 => ZclString::LongOctetString(bytes.read::<&'a [u8]>(offset)?),
            0x44 => ZclString::LongCharString(bytes.read::<&'a str>(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid ZclString",
                })
            }
        };

        Ok((v, *offset))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimeType {
    /// 2.6.2.19 Time of day
    TimeOfDay(u32),
    /// 2.6.2.20 Date
    Date(u32),
    /// 2.6.2.21 UTC Time
    UTCTime(u32),
}

impl<'a> TryRead<'a, u8> for TimeType {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0xE0 => TimeType::TimeOfDay(bytes.read(offset)?),
            0xE1 => TimeType::Date(bytes.read(offset)?),
            0xE2 => TimeType::UTCTime(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid TimeType",
                })
            }
        };

        Ok((v, *offset))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierType {
    /// 2.6.2.22 Cluster ID
    ClusterId(u16),
    /// 2.6.2.23 Attribute ID
    AttributeId(u16),
    /// 2.6.2.24 BACnet OID (Object Identifier)
    BACnetOid(u32),
}

impl<'a> TryRead<'a, u8> for IdentifierType {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0xE8 => IdentifierType::ClusterId(bytes.read(offset)?),
            0xE9 => IdentifierType::AttributeId(bytes.read(offset)?),
            0xEA => IdentifierType::BACnetOid(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid IdentifierType",
                })
            }
        };

        Ok((v, *offset))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MiscType<'a> {
    /// 2.6.2.25 IEEE Address
    IeeeAddress(u64),
    /// 128-bit Security Key
    SecurityKey(&'a [u8; 16]),
}

impl<'a> TryRead<'a, u8> for MiscType<'a> {
    fn try_read(bytes: &'a [u8], identifier: u8) -> Result<(Self, usize), ::byte::Error> {
        let offset = &mut 0;
        let v = match identifier {
            0xF0 => MiscType::IeeeAddress(bytes.read(offset)?),
            // 0xF1 => MiscType::SecurityKey(bytes.read(offset)?),
            _ => {
                return Err(byte::Error::BadInput {
                    err: "invalid MiscType",
                })
            }
        };

        Ok((v, *offset))
    }
}

#[cfg(test)]
mod tests {
    use byte::TryRead;

    use crate::common::data_types::SignedN;

    use super::ZclDataType;

    #[test]
    fn parse_nodata() {
        // given
        let input: &[u8] = &[0x3f];

        // when
        let (data, len) = ZclDataType::try_read(input, 0x00).unwrap();

        // then
        assert_eq!(len, 1);
        assert!(matches!(data, ZclDataType::SignedInt(_)));
        if let ZclDataType::SignedInt(value) = data {
            assert_eq!(value, SignedN::Int16(2623));
        } else {
            panic!("GeneralCommand expected!");
        }
    }
}
