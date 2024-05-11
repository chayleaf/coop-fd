//! A library for working with Russian fiscal data formats.
use std::{
    collections::BTreeMap,
    fmt,
    io::{self, Read},
    mem,
    str::FromStr,
};

use encoding_rs::IBM866;
#[doc(hidden)]
pub use internal::{FieldInternal, Padding, Repr};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use thiserror::Error;

pub use fiscal_data_derive::Ffd;
pub mod enums;
pub mod fields;
pub mod json;
pub mod structs;

#[derive(Debug, Error)]
pub enum Error {
    #[error("number out of range")]
    NumberOutOfRange,
    #[error("field too big")]
    FieldTooBig,
    #[error("invalid cp866 string")]
    InvalidString,
    #[error("eof")]
    Eof,
    #[error("invalid format")]
    InvalidFormat,
    #[error("invalid length")]
    InvalidLength,
    #[error("io error: {0}")]
    Io(
        #[from]
        #[source]
        io::Error,
    ),
}

impl From<std::num::TryFromIntError> for Error {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::NumberOutOfRange
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Self::InvalidFormat
    }
}
impl From<std::convert::Infallible> for Error {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}
// for [u8; N]: TryFrom<Vec<u8>>
impl From<Vec<u8>> for Error {
    fn from(_: Vec<u8>) -> Self {
        Self::InvalidLength
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Pseudo-unix timestamp.
///
/// This shows the number of seconds past 1970-01-01 **in the local timezone**.
#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
#[doc(hidden)]
pub struct LocalTime(u32);

impl<Tz: chrono::TimeZone> TryFrom<chrono::DateTime<Tz>> for LocalTime {
    type Error = Error;
    fn try_from(value: chrono::DateTime<Tz>) -> Result<Self, Self::Error> {
        value.naive_local().try_into()
    }
}

impl TryFrom<chrono::NaiveDateTime> for LocalTime {
    type Error = Error;
    fn try_from(value: chrono::NaiveDateTime) -> Result<Self, Self::Error> {
        value
            .and_utc()
            .timestamp()
            .try_into()
            .map(Self)
            .map_err(|_| Error::NumberOutOfRange)
    }
}

impl TryFrom<chrono::NaiveDate> for LocalTime {
    type Error = Error;
    fn try_from(value: chrono::NaiveDate) -> Result<Self, Self::Error> {
        value.and_time(chrono::NaiveTime::default()).try_into()
    }
}

impl From<LocalTime> for chrono::NaiveDateTime {
    fn from(value: LocalTime) -> Self {
        chrono::DateTime::<chrono::Utc>::from_timestamp(value.0.into(), 0)
            .unwrap()
            .naive_utc()
    }
}

impl From<LocalTime> for chrono::NaiveDate {
    fn from(value: LocalTime) -> Self {
        chrono::NaiveDateTime::from(value).date()
    }
}

struct LocalTimeVis;

impl<'de> Visitor<'de> for LocalTimeVis {
    type Value = LocalTime;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a local timestamp")
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(LocalTime(v.into()))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(LocalTime(v))
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(LocalTime(v.into()))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u32::try_from(v)
            .map(LocalTime)
            .map_err(|_| E::invalid_type(Unexpected::Unsigned(v), &self))
    }
}

impl Serialize for LocalTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

impl<'de> Deserialize<'de> for LocalTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_u32(LocalTimeVis)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VarFloat {
    pub mantissa: u64,
    pub dot_offset: u8,
}

impl VarFloat {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn serialize_into(&self, out: &mut impl io::Write) -> Result<()> {
        out.write_all(&[self.dot_offset])?;
        out.write_all(
            &self.mantissa.to_le_bytes()
                [..mem::size_of::<u64>() - self.mantissa.leading_zeros() as usize / 8],
        )?;
        Ok(())
    }
    pub fn f64_approximation(&self) -> f64 {
        self.mantissa as f64 / 10.0f64.powi(self.dot_offset.into())
    }
}

impl From<VarFloat> for f64 {
    fn from(value: VarFloat) -> Self {
        value.f64_approximation()
    }
}

impl FromStr for VarFloat {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().filter(|&x| x == '.').count() == 1 {
            let s = s.trim_end_matches('0').trim_end_matches('.');
            let mantissa = s.replace('.', "").parse()?;
            Ok(Self {
                mantissa,
                dot_offset: s
                    .find('.')
                    .map(|i| s.len() - i - 1)
                    .unwrap_or_default()
                    .try_into()?,
            })
        } else {
            Ok(Self::from(s.parse::<u64>()?))
        }
    }
}

impl TryFrom<f64> for VarFloat {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        value.to_string().parse()
    }
}

impl TryFrom<f32> for VarFloat {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        value.to_string().parse()
    }
}

impl From<u64> for VarFloat {
    fn from(value: u64) -> Self {
        Self {
            mantissa: value,
            dot_offset: 0,
        }
    }
}

impl From<u32> for VarFloat {
    fn from(value: u32) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<u16> for VarFloat {
    fn from(value: u16) -> Self {
        Self::from(u64::from(value))
    }
}

impl From<u8> for VarFloat {
    fn from(value: u8) -> Self {
        Self::from(u64::from(value))
    }
}

struct VarFloatVisitor;

impl<'de> Visitor<'de> for VarFloatVisitor {
    type Value = VarFloat;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a floating-point number")
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VarFloat::from(v))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VarFloat::from(v))
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VarFloat::from(v))
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(VarFloat::from(v))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        VarFloat::try_from(v).map_err(E::custom)
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        VarFloat::try_from(v as f64).map_err(E::custom)
    }
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u8::try_from(v)
            .map(VarFloat::from)
            .map_err(|_| E::invalid_value(Unexpected::Signed(v.into()), &self))
    }
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u16::try_from(v)
            .map(VarFloat::from)
            .map_err(|_| E::invalid_value(Unexpected::Signed(v.into()), &self))
    }
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u32::try_from(v)
            .map(VarFloat::from)
            .map_err(|_| E::invalid_value(Unexpected::Signed(v.into()), &self))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u64::try_from(v)
            .map(VarFloat::from)
            .map_err(|_| E::invalid_value(Unexpected::Signed(v), &self))
    }
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u64::try_from(v)
            .map(VarFloat::from)
            .map_err(|_| E::invalid_value(Unexpected::Other("a 128-bit integer"), &self))
    }
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u64::try_from(v)
            .map(VarFloat::from)
            .map_err(|_| E::invalid_value(Unexpected::Other("a 128-bit integer"), &self))
    }
}

impl Serialize for VarFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.dot_offset == 0 {
            serializer.serialize_u64(self.mantissa)
        } else {
            serializer.serialize_f64(self.f64_approximation())
        }
    }
}

impl<'de> Deserialize<'de> for VarFloat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(VarFloatVisitor)
    }
}

type Data = Vec<u8>;

#[derive(Clone, Default)]
pub struct Object(BTreeMap<u16, Vec<Data>>, Vec<u8>);

impl Object {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn trailer(&self) -> &[u8] {
        &self.1
    }
    pub fn set_trailer(&mut self, trailer: &[u8]) {
        self.1 = trailer.to_vec();
    }
    fn serialize_into(&self, out: &mut impl io::Write) -> Result<()> {
        for (k, v) in &self.0 {
            for x in v {
                out.write_all(&k.to_le_bytes())?;
                out.write_all(
                    &u16::try_from(x.len())
                        .map_err(|_| Error::FieldTooBig)?
                        .to_le_bytes(),
                )?;
                out.write_all(x)?;
            }
        }
        out.write_all(&self.1)?;
        Ok(())
    }
    pub fn remove<F: Field>(&mut self) -> bool {
        self.0.remove(&F::TAG).is_some()
    }
    pub fn set<F: Field>(&mut self, x: F::Type) -> Result<()> {
        self.0.insert(F::TAG, vec![internal::into_data::<F>(x)?]);
        Ok(())
    }
    pub fn push<F: MultiField>(&mut self, x: F::Type) -> Result<()> {
        self.0
            .entry(F::TAG)
            .or_default()
            .push(internal::into_data::<F>(x)?);
        Ok(())
    }
    pub fn first_obj(&self) -> Result<Option<Object>> {
        for (k, v) in self.0.iter() {
            if matches!(fields::all_reprs().get(k), Some(Repr::Object)) {
                if let Some(x) = v.first() {
                    return Some(Object::from_bytes(x.clone())).transpose();
                }
            }
        }
        Ok(None)
    }
    pub fn contains<F: Field>(&self) -> bool {
        self.0
            .get(&F::TAG)
            .map(|x| !x.is_empty())
            .unwrap_or_default()
    }
    pub fn get<F: Field>(&self) -> Result<Option<F::Type>> {
        let Some(val) = self.0.get(&F::TAG) else {
            return Ok(None);
        };
        let Some(val) = val.first() else {
            return Ok(None);
        };
        internal::from_data::<F>(val.clone()).map(Some)
    }
    pub fn get_all<F: Field>(&self) -> Result<Vec<F::Type>> {
        let Some(val) = self.0.get(&F::TAG) else {
            return Ok(vec![]);
        };
        val.iter()
            .map(|x| internal::from_data::<F>(x.clone()))
            .collect()
    }
    pub fn contains_raw(&self, k: u16) -> bool {
        self.0.get(&k).map(|x| !x.is_empty()).unwrap_or_default()
    }
    pub fn set_raw(&mut self, k: u16, data: &[Data]) {
        self.0.insert(k, data.to_owned());
    }
    pub fn iter_raw(&self) -> impl Iterator<Item = (u16, &[Data])> {
        self.0.iter().map(|(k, v)| (*k, &v[..]))
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        format!("{self:?}") == format!("{other:?}")
    }
}

struct DebugHelper<'a>(&'a [u8], Repr);
impl<'a> fmt::Debug for DebugHelper<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.1 {
            Repr::Int => match u64::from_bytes(self.0.to_vec()) {
                Ok(x) => write!(f, "{x:?}"),
                Err(_) => f.write_str("<invalid int>"),
            },
            Repr::Bytes => write!(f, "{:?}", self.0),
            Repr::Float => match VarFloat::from_bytes(self.0.to_vec()) {
                Ok(x) => write!(f, "{:?}", x.f64_approximation()),
                Err(_) => f.write_str("<invalid float>"),
            },
            Repr::String => write!(f, "{:?}", IBM866.decode(self.0).0),
            Repr::Object => {
                let len = self.0.len();
                let mut r = self.0;
                let mut r = io::Cursor::new(&mut r);
                let mut map = f.debug_map();
                let mut data = vec![];
                while r.position() != len as u64 {
                    let mut tag = [0u8, 0u8];
                    if r.read_exact(&mut tag).is_err() {
                        return f.write_str("<missing tag>");
                    }
                    let tag = u16::from_le_bytes(tag);
                    let mut len = [0u8, 0u8];
                    if r.read_exact(&mut len).is_err() {
                        map.key(&tag);
                        return f.write_str("<missing length>");
                    }
                    let len = u16::from_le_bytes(len);
                    let mut buf = vec![0u8; len.into()];
                    if r.read_exact(&mut buf).is_err() {
                        map.key(&tag);
                        return f.write_str("<missing value>");
                    }
                    data.push((tag, buf));
                }
                data.sort();
                for (tag, buf) in data {
                    map.key(&tag);
                    map.value(&DebugHelper(
                        &buf,
                        fields::all_reprs().get(&tag).copied().unwrap_or_default(),
                    ));
                }
                map.finish()
            }
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = f.debug_map();
        for (k, vs) in &self.0 {
            for v in vs {
                map.entry(
                    k,
                    &DebugHelper(v, fields::all_reprs().get(k).copied().unwrap_or_default()),
                );
            }
        }
        map.finish()
    }
}

impl TlvType for Vec<u8> {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(bytes)
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self)
    }
    const REPR: Repr = Repr::Bytes;
}
impl TlvType for VarFloat {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut ret = VarFloat::new();
        eprintln!("reading float");
        ret.dot_offset = bytes.first().copied().ok_or(Error::Eof)?;
        let mut x = [0u8; mem::size_of::<u64>()];
        if bytes.len() > mem::size_of::<u64>() + 1 {
            return Err(Error::NumberOutOfRange);
        }
        x[..bytes.len() - 1].copy_from_slice(&bytes[1..]);
        ret.mantissa = u64::from_le_bytes(x);
        Ok(ret)
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        let mut ret = vec![];
        self.serialize_into(&mut ret)?;
        Ok(ret)
    }
    const REPR: Repr = Repr::Float;
}
impl TlvType for u64 {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut x = [0u8; mem::size_of::<Self>()];
        if bytes.len() > mem::size_of::<Self>() {
            return Err(Error::NumberOutOfRange);
        }
        x[..bytes.len()].copy_from_slice(&bytes);
        Ok(Self::from_le_bytes(x))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.to_le_bytes()[..mem::size_of::<u64>() - self.leading_zeros() as usize / 8].into())
    }
}
impl TlvType for u32 {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut x = [0u8; mem::size_of::<Self>()];
        if bytes.len() > mem::size_of::<Self>() {
            return Err(Error::NumberOutOfRange);
        }
        x[..bytes.len()].copy_from_slice(&bytes);
        Ok(Self::from_le_bytes(x))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u64).into_bytes()
    }
}
impl TlvType for u16 {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut x = [0u8; mem::size_of::<Self>()];
        if bytes.len() > mem::size_of::<Self>() {
            return Err(Error::NumberOutOfRange);
        }
        x[..bytes.len()].copy_from_slice(&bytes);
        Ok(Self::from_le_bytes(x))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u64).into_bytes()
    }
}
impl TlvType for u8 {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut x = [0u8; mem::size_of::<Self>()];
        if bytes.len() > mem::size_of::<Self>() {
            return Err(Error::NumberOutOfRange);
        }
        x[..bytes.len()].copy_from_slice(&bytes);
        Ok(Self::from_le_bytes(x))
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u64).into_bytes()
    }
}
impl TlvType for bool {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        match u8::from_bytes(bytes)? {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::NumberOutOfRange),
        }
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self as u64).into_bytes()
    }
}
impl TlvType for String {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let (s, _, invalid) = IBM866.decode(&bytes);
        (!invalid)
            .then(|| s.into_owned())
            .ok_or(Error::InvalidString)
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        match IBM866.encode(&self) {
            (s, _, false) => Ok(s.into()),
            _ => Err(Error::InvalidString),
        }
    }
    const REPR: Repr = Repr::String;
}
impl TlvType for LocalTime {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        u32::from_bytes(bytes).map(Self)
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        (self.0 as u64).into_bytes()
    }
}
impl TlvType for chrono::NaiveDateTime {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(LocalTime::from_bytes(bytes)?.into())
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        LocalTime::try_from(self)?.into_bytes()
    }
}
impl TlvType for chrono::NaiveDate {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        Ok(LocalTime::from_bytes(bytes)?.into())
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        LocalTime::try_from(self)?.into_bytes()
    }
}
impl TlvType for Object {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let mut ret = Object::new();
        let len = bytes.len();
        let mut cursor = io::Cursor::new(bytes);
        while cursor.position() != len as u64 {
            eprintln!("{len} reading tag");
            let mut tag = [0u8, 0u8];
            cursor.read_exact(&mut tag).map_err(|_| Error::Eof)?;
            let tag = u16::from_le_bytes(tag);
            eprintln!("{len} reading len (tag={tag})");
            let mut len = [0u8, 0u8];
            cursor.read_exact(&mut len).map_err(|_| Error::Eof)?;
            let len = u16::from_le_bytes(len);
            let mut buf = vec![0u8; len.into()];
            eprintln!("reading buf (len={len}, pos={})", cursor.position());
            cursor.read_exact(&mut buf).map_err(|_| Error::Eof)?;
            ret.0.entry(tag).or_default().push(buf);
            // HACK: if a tag is small, this is probably a document
            // if there's any more data, it's probably the message fiscal sign
            if tag < 100 {
                break;
            }
        }
        cursor.read_to_end(&mut ret.1).map_err(|_| Error::Eof)?;
        Ok(ret)
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        let mut ret = vec![];
        self.serialize_into(&mut ret)?;
        Ok(ret)
    }
    const REPR: Repr = Repr::Object;
}
impl TlvType for [u8; 6] {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self> {
        bytes.try_into().map_err(|_| Error::InvalidLength)
    }
    fn into_bytes(self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }
    const REPR: Repr = Repr::Bytes;
}

pub trait TlvType: Sized {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self>;
    fn into_bytes(self) -> Result<Vec<u8>>;
    // Default to int because most values are integers
    const REPR: Repr = Repr::Int;
}

#[doc(hidden)]
pub mod internal {
    use crate::{Data, Error, Result, TlvType};

    pub fn from_data<F: FieldInternal>(mut data: Data) -> Result<F::Type> {
        if match F::PADDING {
            Padding::None { length: None } => true,
            Padding::None {
                length: Some(length),
            } => data.len() <= length.into(),
            Padding::Fixed { length } => data.len() == usize::from(length),
            Padding::Right { length, padding } => {
                data.len() == usize::from(length) && {
                    while matches!(data.last(), Some(x) if *x == padding) {
                        data.pop();
                    }
                    true
                }
            }
        } {
            Ok(F::Type::from_bytes(data)?)
        } else {
            Err(Error::InvalidLength)
        }
    }

    pub fn into_data<F: FieldInternal>(t: F::Type) -> Result<Data> {
        let mut ret = t.into_bytes()?;
        if match F::PADDING {
            Padding::None { length: None } => true,
            Padding::None {
                length: Some(length),
            } => ret.len() <= length.into(),
            Padding::Fixed { length } => ret.len() == usize::from(length),
            Padding::Right { length, padding } => {
                ret.len() <= length.into() && {
                    ret.resize(length.into(), padding);
                    true
                }
            }
        } {
            Ok(ret)
        } else {
            Err(Error::InvalidLength)
        }
    }

    #[derive(Copy, Clone, Debug, Default)]
    pub enum Repr {
        // Default to bytes so unknown tags are displayed as bytes
        #[default]
        Bytes,
        Float,
        Int,
        String,
        Object,
    }

    pub enum Padding {
        /// No padding
        None {
            /// Max length, if any
            length: Option<u16>,
        },
        /// Pad right with a certain byte
        Right {
            /// String length
            length: u16,
            padding: u8,
        },
        /// Fixed width, no padding because length must match
        Fixed { length: u16 },
    }

    #[derive(Copy, Clone, Debug)]
    pub struct JsonName<'a> {
        pub name: &'a str,
        pub enclosure_tag_overrides: &'a [(u16, &'a str)],
    }

    pub trait FieldInternal {
        const PADDING: Padding = Padding::None { length: None };
        const TAG: u16;
        const JSON_NAME: Option<JsonName<'static>> = None;
        type Type: TlvType;
    }
}

pub trait Field: FieldInternal {}
impl<T: FieldInternal> Field for T {}
pub trait MultiField: FieldInternal {}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use chrono::TimeZone;

    #[allow(unused)]
    use super::*;

    fn round_trip<F: FieldInternal>(x: F::Type, b: &[u8])
    where
        F::Type: Clone + Debug + PartialEq,
    {
        assert_eq!(internal::into_data::<F>(x.clone()).unwrap(), b.to_vec());
        assert_eq!(internal::from_data::<F>(b.into()).unwrap(), x);
    }

    #[test]
    fn test_u32() {
        round_trip::<fields::ShiftNum>(0x12345678, b"\x78\x56\x34\x12");
        round_trip::<fields::ShiftNum>(0x00123456, b"\x56\x34\x12\x00");
        round_trip::<fields::ShiftNum>(0x00000000, b"\x00\x00\x00\x00");
    }

    #[test]
    fn test_vln() {
        round_trip::<fields::TotalSum>(0x12345678, b"\x78\x56\x34\x12");
        round_trip::<fields::TotalSum>(0x00123456, b"\x56\x34\x12");
        round_trip::<fields::TotalSum>(0x00000000, b"");
    }

    #[test]
    fn test_fvln() {
        round_trip::<fields::ItemQuantity>(
            VarFloat {
                mantissa: 0x123456,
                dot_offset: 5,
            },
            b"\x05\x56\x34\x12",
        );
        assert_eq!(
            VarFloat::try_from(12345678.9012345).unwrap(),
            VarFloat {
                mantissa: 123456789012345,
                dot_offset: 7,
            }
        );
        assert_eq!(
            VarFloat::try_from(12345678.0).unwrap(),
            VarFloat {
                mantissa: 12345678,
                dot_offset: 0,
            }
        );
    }

    #[test]
    fn test_string() {
        round_trip::<fields::DocName>(
            "Привет, мир!".to_owned(),
            b"\x8f\xe0\xa8\xa2\xa5\xe2, \xac\xa8\xe0!",
        );
    }

    #[test]
    fn test_stlv() {
        let mut x = Object::new();
        x.set::<fields::TotalSum>(0x12345678).unwrap();
        x.set::<fields::DocName>("Привет, мир!".to_owned()).unwrap();
        round_trip::<fields::ReceiptItem>(
            x,
            b"\xe8\x03\x0c\x00\x8f\xe0\xa8\xa2\xa5\xe2, \xac\xa8\xe0!\xfc\x03\x04\x00xV4\x12",
        );
    }

    #[test]
    fn test_complex_stlv() {
        // test data from https://github.com/yandex/ofd
        let mut report = Object::new();
        report.set::<fields::AutoModeFlag>(false).unwrap();
        report.set::<fields::OfflineModeFlag>(false).unwrap();
        report
            .set::<fields::RetailPlaceAddress>(
                "111141 г.Москва, ул. Кусковская д.20А офис В-202".to_owned(),
            )
            .unwrap();
        report
            .set::<fields::DateTime>(LocalTime(1488541080).into())
            .unwrap();
        report
            .set::<fields::KktSerial>("000000000002".to_owned())
            .unwrap();
        report
            .set::<fields::OfdInn>("7704358518".to_owned())
            .unwrap();
        report
            .set::<fields::UserInn>("7702203276".to_owned())
            .unwrap();
        report
            .set::<fields::Operator>("Шеннон К. ".to_owned())
            .unwrap();
        report
            .set::<fields::KktRegNum>("0000000005008570".to_owned())
            .unwrap();
        report.set::<fields::DocNum>(1).unwrap();
        report
            .set::<fields::DriveNum>("9999078900005488".to_owned())
            .unwrap();
        report
            .set::<fields::OfdName>("OOO TAXCOM".to_owned())
            .unwrap();
        report
            .set::<fields::User>("ООО РАПКАТ-центр ".to_owned())
            .unwrap();
        report.set::<fields::EncryptionFlag>(false).unwrap();
        report.set::<fields::FnsUrl>("nalog.ru".to_owned()).unwrap();
        report
            .set::<fields::TaxationTypes>(
                enums::TaxationTypes::SIMPLIFIED_NET | enums::TaxationTypes::SIMPLIFIED_GROSS,
            )
            .unwrap();
        report
            .set::<fields::DocFiscalSign>([33, 4, 170, 16, 117, 65])
            .unwrap();
        report
            .set::<fields::ReregistrationReason>(enums::ReregistrationReason::FnReplacement)
            .unwrap();
        report.set::<fields::OnlineKktFlag>(false).unwrap();
        report.set::<fields::ServiceFlag>(true).unwrap();
        report.set::<fields::BsoFlag>(false).unwrap();
        report
            .set::<fields::ReceiptSenderEmail>("example@example.com".to_owned())
            .unwrap();
        report
            .set::<fields::RetailPlace>(
                "111141 г.Москва, ул. Кусковская\r\nд.20А офис В-202".to_owned(),
            )
            .unwrap();
        report.set::<fields::KktVer>("2.0".to_owned()).unwrap();
        report
            .set::<fields::KktFfdVer>(enums::FfdVersion::V1_05)
            .unwrap();
        report
            .set::<fields::OperatorInn>("771234567890".to_owned())
            .unwrap();
        report.set::<fields::ExciseFlag>(true).unwrap();
        report
            .set::<fields::FfdVer>(enums::FfdVersion::V1_05)
            .unwrap();
        report.set::<fields::PrinterFlag>(false).unwrap();
        let mut x = Object::new();
        x.set::<fields::RegistrationParamUpdateReport>(report)
            .unwrap();
        // different order
        let data = b"\x0b\x00\x86\x01\x11\x04\x10\x009999078900005488\r\x04\x14\x000000000005008570    \xfa\x03\x0c\x007702203276  \x10\x04\x04\x00\x01\x00\x00\x00\xf4\x03\x04\x00\x98U\xb9X5\x04\x06\x00!\x04\xaa\x10uA \x04\x01\x00\x00\xea\x03\x01\x00\x00\xe9\x03\x01\x00\x00U\x04\x01\x00\x01V\x04\x01\x00\x00T\x04\x01\x00\x00&\x04\x01\x00\x06M\x04\x01\x00\x01\xf5\x03\x0c\x00000000000002\x18\x04\x11\x00\x8e\x8e\x8e \x90\x80\x8f\x8a\x80\x92-\xe6\xa5\xad\xe2\xe0 \xf1\x030\x00111141 \xa3.\x8c\xae\xe1\xaa\xa2\xa0, \xe3\xab. \x8a\xe3\xe1\xaa\xae\xa2\xe1\xaa\xa0\xef \xa4.20\x80 \xae\xe4\xa8\xe1 \x82-202\xf9\x03\x0c\x007704358518  $\x04\x08\x00nalog.ru]\x04\x13\x00example@example.com\xfd\x03\n\x00\x98\xa5\xad\xad\xae\xad \x8a. \x16\x04\n\x00OOO TAXCOM\xa5\x04\x01\x00\x02\xb9\x04\x01\x00\x02\xa4\x04\x03\x002.0\xa3\x041\x00111141 \xa3.\x8c\xae\xe1\xaa\xa2\xa0, \xe3\xab. \x8a\xe3\xe1\xaa\xae\xa2\xe1\xaa\xa0\xef\r\n\xa4.20\x80 \xae\xe4\xa8\xe1 \x82-202\xb3\x04\x0c\x00771234567890\xc5\x04\x01\x00\x00\xb7\x04\x01\x00\x01";
        let obj = Object::from_bytes(data.into()).unwrap();
        assert_eq!(obj, x);
        round_trip::<fields::ReceiptItem>(x, b"\x0b\x00\x86\x01\xe9\x03\x01\x00\x00\xea\x03\x01\x00\x00\xf1\x030\x00111141 \xa3.\x8c\xae\xe1\xaa\xa2\xa0, \xe3\xab. \x8a\xe3\xe1\xaa\xae\xa2\xe1\xaa\xa0\xef \xa4.20\x80 \xae\xe4\xa8\xe1 \x82-202\xf4\x03\x04\x00\x98U\xb9X\xf5\x03\x0c\x00000000000002\xf9\x03\x0c\x007704358518  \xfa\x03\x0c\x007702203276  \xfd\x03\n\x00\x98\xa5\xad\xad\xae\xad \x8a. \r\x04\x14\x000000000005008570    \x10\x04\x04\x00\x01\x00\x00\x00\x11\x04\x10\x009999078900005488\x16\x04\n\x00OOO TAXCOM\x18\x04\x11\x00\x8e\x8e\x8e \x90\x80\x8f\x8a\x80\x92-\xe6\xa5\xad\xe2\xe0  \x04\x01\x00\x00$\x04\x08\x00nalog.ru&\x04\x01\x00\x065\x04\x06\x00!\x04\xaa\x10uAM\x04\x01\x00\x01T\x04\x01\x00\x00U\x04\x01\x00\x01V\x04\x01\x00\x00]\x04\x13\x00example@example.com\xa3\x041\x00111141 \xa3.\x8c\xae\xe1\xaa\xa2\xa0, \xe3\xab. \x8a\xe3\xe1\xaa\xae\xa2\xe1\xaa\xa0\xef\r\n\xa4.20\x80 \xae\xe4\xa8\xe1 \x82-202\xa4\x04\x03\x002.0\xa5\x04\x01\x00\x02\xb3\x04\x0c\x00771234567890\xb7\x04\x01\x00\x01\xb9\x04\x01\x00\x02\xc5\x04\x01\x00\x00");
    }

    #[test]
    fn test_multikey() {
        let mut x = Object::new();
        x.push::<fields::TransferOperatorAddress>("A".to_owned())
            .unwrap();
        x.push::<fields::TransferOperatorAddress>("B".to_owned())
            .unwrap();
        x.push::<fields::TransferOperatorAddress>("C".to_owned())
            .unwrap();
        round_trip::<fields::ReceiptItem>(
            x,
            b"\xed\x03\x01\x00A\xed\x03\x01\x00B\xed\x03\x01\x00C",
        );
    }

    #[test]
    fn test_timestamp() {
        let date1 = chrono::Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let date2 = chrono::FixedOffset::east_opt(600)
            .unwrap()
            .with_ymd_and_hms(2020, 1, 1, 0, 0, 0)
            .unwrap();
        assert_eq!(
            LocalTime::try_from(date1).unwrap(),
            LocalTime::try_from(date2).unwrap()
        );
        assert_eq!(
            chrono::NaiveDateTime::from(LocalTime::try_from(date1).unwrap()),
            date2.naive_local()
        );
        assert_eq!(
            chrono::NaiveDateTime::from(LocalTime::try_from(date2).unwrap()),
            date1.naive_local()
        );
    }
}
