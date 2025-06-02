#[derive(Debug,PartialEq)]
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

#[derive(Debug,PartialEq, Eq)]
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

#[derive(Debug,PartialEq, Eq)]
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

#[derive(Debug,PartialEq, Eq)]
pub enum EnumN {
    Enum8(u8),
    Enum16(u16),
}

#[derive(Debug,PartialEq)]
pub enum FloatN {
    Semi(f32),   // Could represent custom 16-bit float as needed
    Single(f32),
    Double(f64),
}

#[derive(Debug,PartialEq, Eq)]
pub enum ZclString<'a> {
    OctetString(&'a [u8]),
    CharString(&'a str),
    LongOctetString(&'a [u8]),
    LongCharString(&'a str),
}

#[derive(Debug,PartialEq, Eq)]
pub enum TimeType {
    TimeOfDay(u32),
    Date(u32),
    UTCTime(u32),
}

#[derive(Debug,PartialEq, Eq)]
pub enum IdentifierType {
    ClusterId(u16),
    AttributeId(u16),
    BACnetOid(u32),
}

#[derive(Debug,PartialEq, Eq)]
pub enum MiscType<'a> {
    IeeeAddress(u64),
    SecurityKey(&'a [u8; 16]),
}

