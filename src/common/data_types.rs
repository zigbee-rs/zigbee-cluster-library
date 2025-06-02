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

impl ZclDataType<'_> {
    pub fn length(&self) -> usize {
        match self {
            ZclDataType::NoData => 0,
            ZclDataType::Data(data) => data.length(),
            ZclDataType::Bool(_) => 1,
            ZclDataType::Bitmap(bitmap) => bitmap.length(),
            ZclDataType::UnsignedInt(unsigned) => unsigned.length(),
            ZclDataType::SignedInt(signed) => signed.length(),
            ZclDataType::Enum(enum_type) => enum_type.length(),
            ZclDataType::Float(float) => float.length(),
            ZclDataType::String(string) => string.length(),
            ZclDataType::Array(array) => array.len(),
            ZclDataType::Structure(structure) => structure.len(),
            ZclDataType::Set(set) => set.len(),
            ZclDataType::Bag(bag) => bag.len(),
            ZclDataType::Time(time) => time.length(),
            ZclDataType::Identifier(identifier) => identifier.length(),
            ZclDataType::Misc(misc) => misc.length(),
            ZclDataType::Unknown => 0,
        }
    }
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

impl DataN {
    pub fn length(&self) -> usize {
        match self {
            DataN::Data8(_) => 1,
            DataN::Data16(_) => 2,
            DataN::Data24(_) => 3,
            DataN::Data32(_) => 4,
            DataN::Data40(_) => 5,
            DataN::Data48(_) => 6,
            DataN::Data56(_) => 7,
            DataN::Data64(_) => 8,
        }
    }
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

impl BitmapN {
    pub fn length(&self) -> usize {
        match self {
            BitmapN::Bitmap8(_) => 1,
            BitmapN::Bitmap16(_) => 2,
            BitmapN::Bitmap24(_) => 3,
            BitmapN::Bitmap32(_) => 4,
            BitmapN::Bitmap40(_) => 5,
            BitmapN::Bitmap48(_) => 6,
            BitmapN::Bitmap56(_) => 7,
            BitmapN::Bitmap64(_) => 8,
        }
    }
}

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

impl UnsignedN {
    pub fn length(&self) -> usize {
        match self {
            UnsignedN::Uint8(_) => 1,
            UnsignedN::Uint16(_) => 2,
            UnsignedN::Uint24(_) => 3,
            UnsignedN::Uint32(_) => 4,
            UnsignedN::Uint40(_) => 5,
            UnsignedN::Uint48(_) => 6,
            UnsignedN::Uint56(_) => 7,
            UnsignedN::Uint64(_) => 8,
        }
    }
}

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

impl SignedN {
    pub fn length(&self) -> usize {
        match self {
            SignedN::Int8(_) => 1,
            SignedN::Int16(_) => 2,
            SignedN::Int24(_) => 3,
            SignedN::Int32(_) => 4,
            SignedN::Int40(_) => 5,
            SignedN::Int48(_) => 6,
            SignedN::Int56(_) => 7,
            SignedN::Int64(_) => 8,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum EnumN {
    Enum8(u8),
    Enum16(u16),
}

impl EnumN {
    pub fn length(&self) -> usize {
        match self {
            EnumN::Enum8(_) => 1,
            EnumN::Enum16(_) => 2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FloatN {
    Semi(u16), // Could represent custom 16-bit float as needed
    Single(f32),
    Double(f64),
}

impl FloatN {
    pub fn length(&self) -> usize {
        match self {
            FloatN::Semi(_) => 2,
            FloatN::Single(_) => 4,
            FloatN::Double(_) => 8,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ZclString<'a> {
    OctetString(&'a [u8]),
    CharString(&'a str),
    LongOctetString(&'a [u8]),
    LongCharString(&'a str),
}

impl ZclString<'_> {
    pub fn length(&self) -> usize {
        match self {
            ZclString::OctetString(s) => s.len(),
            ZclString::CharString(s) => s.len(),
            ZclString::LongOctetString(s) => s.len(),
            ZclString::LongCharString(s) => s.len(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimeType {
    TimeOfDay(u32),
    Date(u32),
    UTCTime(u32),
}

impl TimeType {
    pub fn length(&self) -> usize {
        4
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum IdentifierType {
    ClusterId(u16),
    AttributeId(u16),
    BACnetOid(u32),
}

impl IdentifierType {
    pub fn length(&self) -> usize {
        match self {
            IdentifierType::ClusterId(_) => 2,
            IdentifierType::AttributeId(_) => 2,
            IdentifierType::BACnetOid(_) => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum MiscType<'a> {
    IeeeAddress(u64),
    SecurityKey(&'a [u8; 16]),
}

impl MiscType<'_> {
    pub fn length(&self) -> usize {
        match self {
            MiscType::IeeeAddress(_) => 8,
            MiscType::SecurityKey(_) => 16,
        }
    }
}
