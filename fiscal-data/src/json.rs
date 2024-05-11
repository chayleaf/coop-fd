use std::fmt;

use fiscal_data_derive::Ffd;
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{self as fiscal_data, fields, Error, FieldInternal, Object, TlvType};

pub mod one_or_many {
    use std::{fmt, marker::PhantomData};

    use serde::{
        de::{MapAccess, SeqAccess, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    #[derive(Copy, Clone)]
    struct SingleOrMultiVis<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for SingleOrMultiVis<T> {
        type Value = Vec<T>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("one or multiple objects")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![T::deserialize(
                serde::de::value::StrDeserializer::new(v),
            )?])
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![T::deserialize(
                serde::de::value::StringDeserializer::new(v),
            )?])
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![T::deserialize(
                serde::de::value::BorrowedStrDeserializer::new(v),
            )?])
        }
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Ok(vec![T::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?])
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut ret = seq
                .size_hint()
                .map(Vec::with_capacity)
                .unwrap_or_else(Vec::new);
            while let Some(x) = seq.next_element()? {
                ret.push(x);
            }
            Ok(ret)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserialize(deserializer)
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![])
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![])
        }
    }

    pub fn serialize<S, T: Serialize>(x: &[T], ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let &[x] = &x {
            x.serialize(ser)
        } else {
            x.serialize(ser)
        }
    }
    pub fn deserialize<'de, D, T>(de: D) -> Result<Vec<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        de.deserialize_any(SingleOrMultiVis::<T>(PhantomData))
    }
}

pub mod one_or_singleton {
    use std::{fmt, marker::PhantomData};

    use serde::{
        de::{Error, MapAccess, SeqAccess, Visitor},
        Deserialize, Deserializer,
    };

    #[derive(Copy, Clone)]
    struct SingleOrTon<T>(PhantomData<T>);

    impl<'de, T: Deserialize<'de>> Visitor<'de> for SingleOrTon<T> {
        type Value = Option<T>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a single object or none")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(T::deserialize(
                serde::de::value::StrDeserializer::new(v),
            )?))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(T::deserialize(
                serde::de::value::StringDeserializer::new(v),
            )?))
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(T::deserialize(
                serde::de::value::BorrowedStrDeserializer::new(v),
            )?))
        }
        fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            Ok(Some(T::deserialize(
                serde::de::value::MapAccessDeserializer::new(map),
            )?))
        }
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let Some(x) = seq.next_element()? else {
                return Ok(None);
            };
            if seq.next_element::<T>()?.is_some() {
                return Err(A::Error::invalid_length(1, &self));
            }
            Ok(Some(x))
        }
    }

    pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        de.deserialize_any(SingleOrTon::<T>(PhantomData))
    }
}

pub mod as_localtime {
    use serde::{ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};
    use std::fmt;

    use crate::LocalTime;

    pub fn deserialize<'de, D, T: From<LocalTime>>(de: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(LocalTime::deserialize(de)?.into())
    }
    pub fn serialize<S, E: fmt::Display, T: Clone + TryInto<LocalTime, Error = E>>(
        x: &T,
        ser: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match x.clone().try_into() {
            Ok(val) => val.serialize(ser),
            Err(err) => Err(S::Error::custom(err)),
        }
    }
}
pub mod as_localtime_opt {
    use std::fmt;

    use serde::{de::Visitor, Deserialize, Deserializer, Serializer};

    use crate::LocalTime;

    struct OptionVisitor;
    impl<'de> Visitor<'de> for OptionVisitor {
        type Value = Option<LocalTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a local timestamp or none")
        }

        fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            LocalTime::deserialize(d).map(Some)
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }

    pub fn deserialize<'de, D, T: From<LocalTime>>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(de.deserialize_option(OptionVisitor)?.map(From::from))
    }
    pub fn serialize<S, E: fmt::Display, T: Clone + TryInto<LocalTime, Error = E>>(
        x: &Option<T>,
        ser: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(x) = x {
            super::as_localtime::serialize(x, ser)
        } else {
            ser.serialize_none()
        }
    }
}
pub mod fiscal_sign_6 {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(de: D) -> Result<[u8; 6], D::Error>
    where
        D: Deserializer<'de>,
    {
        let [_, _, a, b, c, d, e, f] = u64::deserialize(de)?.to_be_bytes();
        Ok([a, b, c, d, e, f])
    }
    pub fn serialize<S>(x: &[u8; 6], ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let [a, b, c, d, e, f] = *x;
        ser.serialize_u64(u64::from_be_bytes([0, 0, a, b, c, d, e, f]))
    }
}
pub mod fiscal_sign_8_opt {
    use serde::{de::Visitor, Deserialize, Deserializer, Serializer};
    use std::fmt;

    struct Vis;
    impl<'de> Visitor<'de> for Vis {
        type Value = Option<u64>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("u64")
        }
        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.into()))
        }
        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.into()))
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.into()))
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v))
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            Wrapper::deserialize(deserializer).map(|x| x.0)
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        // Many services love destroying the data FNS gives them by using floats instead of
        // integers for representing all numbers, including ФПС. Oh well, it's fine, I guess I
        // don't need it for anything anyway... This makes sure such data still parses.
        fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
    }
    struct Wrapper(Option<u64>);
    impl<'de> Deserialize<'de> for Wrapper {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_u64(Vis).map(Self)
        }
    }
    pub fn deserialize<'de, D>(de: D) -> Result<Option<[u8; 8]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Wrapper::deserialize(de)?.0.map(u64::to_be_bytes))
    }
    pub fn serialize<S>(x: &Option<[u8; 8]>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(x) = x {
            ser.serialize_some(&u64::from_be_bytes(*x))
        } else {
            ser.serialize_none()
        }
    }
}
pub mod bitvec1_opt {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: From<u32>,
    {
        let Some(bits) = Option::<Vec<u8>>::deserialize(de)? else {
            return Ok(None);
        };
        let mut ret = 0u32;
        for bit in bits {
            ret |= 1u32 << bit >> 1;
        }
        Ok(Some(ret.into()))
    }
    pub fn serialize<S, T>(x: &Option<T>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Clone + Into<u32>,
    {
        let Some(x) = x.clone().map(Into::into) else {
            return None::<Vec<u8>>.serialize(ser);
        };
        let mut bits = Vec::new();
        for i in 0u8..32 {
            if x & (1 << i) != 0 {
                bits.push(i + 1);
            }
        }
        Some(bits).serialize(ser)
    }
}

pub mod marking_code_opt {
    use std::fmt;

    use serde::{
        de::{Error, IgnoredAny, MapAccess, Visitor},
        Deserializer,
    };

    struct Vis;
    impl<'de> Visitor<'de> for Vis {
        type Value = Option<String>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a marking code or none")
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.to_owned()))
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v))
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(v.to_owned()))
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            while let Some(k) = map.next_key::<&str>()? {
                if k == "rawProductCode" {
                    return Ok(Some(map.next_value()?));
                }
                map.next_value::<IgnoredAny>()?;
            }
            Err(A::Error::missing_field("rawProductCode"))
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(Vis)
    }
}
pub mod bool_num {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };

    struct Vis;
    impl<'de> Visitor<'de> for Vis {
        type Value = bool;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean")
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }
        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
    }
    pub fn deserialize<'de, D>(de: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(Vis)
    }
    pub fn serialize<S>(x: &bool, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_u8(u8::from(*x))
    }
}
pub mod bool_num_opt {
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };
    struct Vis;
    impl<'de> Visitor<'de> for Vis {
        type Value = Option<bool>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a boolean or none")
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(self)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v))
        }
        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(v != 0))
        }
    }
    pub fn deserialize<'de, D>(de: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(Vis)
    }
    pub fn serialize<S>(x: &Option<bool>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(x) = x {
            ser.serialize_some(&u8::from(*x))
        } else {
            ser.serialize_none()
        }
    }
}
pub mod base64_vec_opt {
    use base64::prelude::*;
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };
    struct Vis;
    impl<'de> Visitor<'de> for Vis {
        type Value = Option<Vec<u8>>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a base64 string or none")
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(self)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(v)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(
                BASE64_STANDARD.decode(v).map_err(|err| E::custom(err))?,
            ))
        }
    }
    pub fn deserialize<'de, D>(de: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(Vis)
    }
    pub fn serialize<S>(x: &Option<Vec<u8>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(x) = x {
            ser.serialize_some(&BASE64_STANDARD.encode(x))
        } else {
            ser.serialize_none()
        }
    }
}
pub mod base64_array_opt {
    use base64::prelude::*;
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };
    struct Vis<const N: usize>;
    impl<'de, const N: usize> Visitor<'de> for Vis<N> {
        type Value = Option<[u8; N]>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a base64 string containing {N} bytes or none")
        }
        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(self)
        }
        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(v)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Some(
                BASE64_STANDARD
                    .decode(v)
                    .map_err(|err| E::custom(err))?
                    .try_into()
                    .map_err(|x: Vec<u8>| E::invalid_length(x.len(), &self))?,
            ))
        }
    }
    pub fn deserialize<'de, D, const N: usize>(de: D) -> Result<Option<[u8; N]>, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(Vis)
    }
    pub fn serialize<S, const N: usize>(x: &Option<[u8; N]>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(x) = x {
            ser.serialize_some(&BASE64_STANDARD.encode(x))
        } else {
            ser.serialize_none()
        }
    }
}
pub mod base64_array {
    use base64::prelude::*;
    use std::fmt;

    use serde::{
        de::{Error, Visitor},
        Deserializer, Serializer,
    };
    struct Vis<const N: usize>;
    impl<'de, const N: usize> Visitor<'de> for Vis<N> {
        type Value = [u8; N];
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a base64 string containing {N} bytes")
        }
        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(&v)
        }
        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            self.visit_str(v)
        }
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            BASE64_STANDARD
                .decode(v)
                .map_err(|err| E::custom(err))?
                .try_into()
                .map_err(|x: Vec<u8>| E::invalid_length(x.len(), &self))
        }
    }
    pub fn deserialize<'de, D, const N: usize>(de: D) -> Result<[u8; N], D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(Vis)
    }
    pub fn serialize<S>(x: &[u8], ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ser.serialize_some(&BASE64_STANDARD.encode(x))
    }
}

impl serde::de::Error for Error {
    fn custom<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Self::InvalidFormat
    }
}

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constant<const N: u16>;

struct ConstantVis<const N: u16>;

impl<const N: u16> From<Constant<N>> for u16 {
    fn from(_: Constant<N>) -> Self {
        N
    }
}

impl<const N: u16> TryFrom<u16> for Constant<N> {
    type Error = Error;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == N {
            Ok(Self)
        } else {
            Err(Error::NumberOutOfRange)
        }
    }
}

impl<'de, const N: u16> Visitor<'de> for ConstantVis<N> {
    type Value = Constant<N>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "the number {N}")
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        self.visit_u16(v.into())
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == N {
            Ok(Constant)
        } else {
            Err(E::invalid_value(
                serde::de::Unexpected::Unsigned(v.into()),
                &self,
            ))
        }
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u16::try_from(v)
            .map_err(|_| E::invalid_type(Unexpected::Unsigned(v.into()), &self))
            .and_then(|v| self.visit_u16(v))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        u16::try_from(v)
            .map_err(|_| E::invalid_type(Unexpected::Unsigned(v), &self))
            .and_then(|v| self.visit_u16(v))
    }
}

impl<const N: u16> Serialize for Constant<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(N)
    }
}

impl<'de, const N: u16> Deserialize<'de> for Constant<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u16(ConstantVis::<N>)
    }
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct FiscalReport {
    #[ffd(special = "tag")]
    pub code: Constant<1>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::UserInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userInn")]
    pub user_inn: Option<<fields::UserInn as FieldInternal>::Type>,
    #[ffd(tag = fields::DateTime)]
    #[serde(rename = "dateTime", with = "as_localtime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::OfflineModeFlag)]
    #[serde(with = "bool_num")]
    #[serde(rename = "offlineMode")]
    pub offline_mode: <fields::OfflineModeFlag as FieldInternal>::Type,
    #[ffd(tag = fields::PrinterFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "printInMachineSign")]
    pub print_in_machine_sign: Option<<fields::PrinterFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::BsoFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "bsoSign")]
    pub bso_sign: Option<<fields::BsoFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::EncryptionFlag)]
    #[serde(with = "bool_num")]
    #[serde(rename = "encryptionSign")]
    pub encryption_sign: <fields::EncryptionFlag as FieldInternal>::Type,
    #[ffd(tag = fields::AutoModeFlag)]
    #[serde(with = "bool_num")]
    #[serde(rename = "autoMode")]
    pub auto_mode: <fields::AutoModeFlag as FieldInternal>::Type,
    #[ffd(tag = fields::KktUsageFlags)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usageConditionSigns")]
    pub usage_condition_signs: Option<<fields::KktUsageFlags as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: <fields::RetailPlaceAddress as FieldInternal>::Type,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(rename = "retailPlace")]
    pub retail_place: <fields::RetailPlace as FieldInternal>::Type,
    #[ffd(tag = fields::OnlineKktFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "internetSign")]
    pub internet_sign: Option<<fields::OnlineKktFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::KktSerial)]
    #[serde(rename = "kktNumber")]
    pub kkt_number: <fields::KktSerial as FieldInternal>::Type,
    #[ffd(tag = fields::Operator)]
    pub operator: <fields::Operator as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(rename = "fiscalSign", with = "fiscal_sign_6")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::KktVer)]
    #[serde(rename = "kktVersion")]
    pub kkt_version: <fields::KktVer as FieldInternal>::Type,
    #[ffd(tag = fields::KktFfdVer)]
    #[serde(rename = "documentKktVersion")]
    pub document_kkt_version: <fields::KktFfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::ExciseFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "exciseDutyProductSign")]
    pub excise_duty_product_sign: Option<<fields::ExciseFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::ServiceFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "serviceSign")]
    pub service_sign: Option<<fields::ServiceFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::GamblingFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "gamblingSign")]
    pub gambling_sign: Option<<fields::GamblingFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::LotteryFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "lotterySign")]
    pub lottery_sign: Option<<fields::LotteryFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paymentAgentType")]
    pub payment_agent_type: Option<<fields::PaymentAgentTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveFfdVer)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentFdVersion")]
    pub document_fd_version: Option<<fields::DriveFfdVer as FieldInternal>::Type>,
    #[ffd(tag = fields::FiscalSignValidityPeriod)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fdKeyResource")]
    pub fd_key_resource: Option<<fields::FiscalSignValidityPeriod as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::TaxationTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taxationType")]
    pub taxation_type: Option<<fields::TaxationTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::MachineNumber)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "machineNumber")]
    pub machine_number: Option<<fields::MachineNumber as FieldInternal>::Type>,
    #[ffd(tag = fields::OfdName)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ofdName")]
    pub ofd_name: Option<<fields::OfdName as FieldInternal>::Type>,
    #[ffd(tag = fields::ReceiptSenderEmail)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sellerAddress")]
    pub seller_address: Option<<fields::ReceiptSenderEmail as FieldInternal>::Type>,
    #[ffd(tag = fields::FnsUrl)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fnsUrl")]
    pub fns_url: Option<<fields::FnsUrl as FieldInternal>::Type>,
    #[ffd(tag = fields::OfdInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ofdInn")]
    pub ofd_inn: Option<<fields::OfdInn as FieldInternal>::Type>,
    #[ffd(tag = fields::FiscalReportAdditionalProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropsFRC")]
    pub additional_props_frc: Option<<fields::FiscalReportAdditionalProp as FieldInternal>::Type>,
    #[ffd(tag = fields::FiscalReportAdditionalData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalDataFRC")]
    pub additional_data_frc: Option<<fields::FiscalReportAdditionalData as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct FiscalReportCorrection {
    #[ffd(special = "tag")]
    pub code: Constant<11>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    pub user: <fields::User as FieldInternal>::Type,
    #[ffd(tag = fields::UserInn)]
    #[serde(rename = "userInn")]
    pub user_inn: <fields::UserInn as FieldInternal>::Type,
    #[ffd(tag = fields::DateTime)]
    #[serde(rename = "dateTime", with = "as_localtime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::OfflineModeFlag)]
    #[serde(with = "bool_num")]
    #[serde(rename = "offlineMode")]
    pub offline_mode: <fields::OfflineModeFlag as FieldInternal>::Type,
    #[ffd(tag = fields::PrinterFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "printInMachineSign")]
    pub print_in_machine_sign: Option<<fields::PrinterFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::BsoFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "bsoSign")]
    pub bso_sign: Option<<fields::BsoFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::EncryptionFlag)]
    #[serde(with = "bool_num")]
    #[serde(rename = "encryptionSign")]
    pub encryption_sign: <fields::EncryptionFlag as FieldInternal>::Type,
    #[ffd(tag = fields::AutoModeFlag)]
    #[serde(with = "bool_num")]
    #[serde(rename = "autoMode")]
    pub auto_mode: <fields::AutoModeFlag as FieldInternal>::Type,
    #[ffd(tag = fields::KktUsageFlags)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usageConditionSigns")]
    pub usage_condition_signs: Option<<fields::KktUsageFlags as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: <fields::RetailPlaceAddress as FieldInternal>::Type,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(rename = "retailPlace")]
    pub retail_place: <fields::RetailPlace as FieldInternal>::Type,
    #[ffd(tag = fields::OnlineKktFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "internetSign")]
    pub internet_sign: Option<<fields::OnlineKktFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::KktSerial)]
    #[serde(rename = "kktNumber")]
    pub kkt_number: <fields::KktSerial as FieldInternal>::Type,
    #[ffd(tag = fields::Operator)]
    pub operator: <fields::Operator as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(rename = "fiscalSign", with = "fiscal_sign_6")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::KktVer)]
    #[serde(rename = "kktVersion")]
    pub kkt_version: <fields::KktVer as FieldInternal>::Type,
    #[ffd(tag = fields::KktFfdVer)]
    #[serde(rename = "documentKktVersion")]
    pub document_kkt_version: <fields::KktFfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::DriveFfdVer)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "documentFdVersion")]
    pub document_fd_version: Option<<fields::DriveFfdVer as FieldInternal>::Type>,
    #[ffd(tag = fields::FiscalSignValidityPeriod)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fdKeyResource")]
    pub fd_key_resource: Option<<fields::FiscalSignValidityPeriod as FieldInternal>::Type>,
    #[ffd(tag = fields::ExciseFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "exciseDutyProductSign")]
    pub excise_duty_product_sign: Option<<fields::ExciseFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::ServiceFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "serviceSign")]
    pub service_sign: Option<<fields::ServiceFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::GamblingFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "gamblingSign")]
    pub gambling_sign: Option<<fields::GamblingFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::LotteryFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "lotterySign")]
    pub lottery_sign: Option<<fields::LotteryFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paymentAgentType")]
    pub payment_agent_type: Option<<fields::PaymentAgentTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::KktInfoUpdateReason)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "correctionKktReasonCode")]
    pub correction_kkt_reason_code: Option<<fields::KktInfoUpdateReason as FieldInternal>::Type>,
    #[ffd(tag = fields::KktInfoUpdateReason)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "correctionReasonCode", with = "bitvec1_opt")]
    pub correction_reason_code: Option<<fields::KktInfoUpdateReason as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::TaxationTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taxationType")]
    pub taxation_type: Option<<fields::TaxationTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::MachineNumber)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "machineNumber")]
    pub machine_number: Option<<fields::MachineNumber as FieldInternal>::Type>,
    #[ffd(tag = fields::OfdName)]
    #[serde(rename = "ofdName")]
    pub ofd_name: <fields::OfdName as FieldInternal>::Type,
    #[ffd(tag = fields::ReceiptSenderEmail)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sellerAddress")]
    pub seller_address: Option<<fields::ReceiptSenderEmail as FieldInternal>::Type>,
    #[ffd(tag = fields::FnsUrl)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fnsUrl")]
    pub fns_url: Option<<fields::FnsUrl as FieldInternal>::Type>,
    #[ffd(tag = fields::OfdInn)]
    #[serde(rename = "ofdInn")]
    pub ofd_inn: <fields::OfdInn as FieldInternal>::Type,
    #[ffd(tag = fields::FiscalReportAdditionalProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropsFRC")]
    pub additional_props_frc: Option<<fields::FiscalReportAdditionalProp as FieldInternal>::Type>,
    #[ffd(tag = fields::FiscalReportAdditionalData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalDataFRC")]
    pub additional_data_frc: Option<<fields::FiscalReportAdditionalData as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fiscal_drive_sum_reports: Option<DriveStats>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveStats {
    #[ffd(tag = fields::TotalReceiptAndCorrectionCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_receipt_bso_count:
        Option<<fields::TotalReceiptAndCorrectionCount as FieldInternal>::Type>,
    #[ffd(tag = fields::SaleStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_oper: Option<CounterByPaymentType>,
    #[ffd(tag = fields::SaleReturnStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_return_oper: Option<CounterByPaymentType>,
    #[ffd(tag = fields::PurchaseStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_oper: Option<CounterByPaymentType>,
    #[ffd(tag = fields::PurchaseReturnStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_return_oper: Option<CounterByPaymentType>,
    #[ffd(tag = fields::CorrectionStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_correction: Option<CorrectionCounters>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterByPaymentType {
    #[ffd(tag = fields::AggregatedReceiptCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_bso_count: Option<<fields::AggregatedReceiptCount as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedCashSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_sum: Option<<fields::AggregatedCashSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedEcashSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecash_sum: Option<<fields::AggregatedEcashSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedPrepaidSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_sum: Option<<fields::AggregatedPrepaidSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedCreditSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_sum: Option<<fields::AggregatedCreditSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedProvisionSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision_sum: Option<<fields::AggregatedProvisionSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_sum: Option<<fields::AggregatedSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedVat20Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_18_sum: Option<<fields::AggregatedVat20Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedVat10Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_10_sum: Option<<fields::AggregatedVat10Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedVat20_120Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_18_118_sum: Option<<fields::AggregatedVat20_120Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedVat10_110Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_10_110_sum: Option<<fields::AggregatedVat10_110Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedSumWithVat0)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_0_sum: Option<<fields::AggregatedSumWithVat0 as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedSumWithNoVat)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_free_sum: Option<<fields::AggregatedSumWithNoVat as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionCounter {
    #[ffd(tag = fields::AggregatedReceiptCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_bso_count: Option<<fields::AggregatedReceiptCount as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedCashSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cash_sum: Option<<fields::AggregatedCashSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedEcashSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ecash_sum: Option<<fields::AggregatedEcashSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedPrepaidSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepaid_sum: Option<<fields::AggregatedPrepaidSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedCreditSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credit_sum: Option<<fields::AggregatedCreditSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedProvisionSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision_sum: Option<<fields::AggregatedProvisionSum as FieldInternal>::Type>,
    #[ffd(tag = fields::AggregatedSum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_sum: Option<<fields::AggregatedSum as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionCounters {
    #[ffd(tag = fields::CorrectionAndUntransmittedCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_correction_count:
        Option<<fields::CorrectionAndUntransmittedCount as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedSaleStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_correction: Option<CorrectionCounter>,
    #[ffd(tag = fields::UntransmittedPurchaseStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_correction: Option<CorrectionCounter>,
    #[ffd(tag = fields::UntransmittedSaleReturnStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_return_correction: Option<CorrectionCounter>,
    #[ffd(tag = fields::UntransmittedPurchaseReturnStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_return_correction: Option<CorrectionCounter>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct OpenShift {
    #[ffd(special = "tag")]
    pub code: Constant<2>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::UserInn)]
    #[serde(rename = "userInn")]
    pub user_inn: <fields::UserInn as FieldInternal>::Type,
    #[ffd(tag = fields::DateTime)]
    #[serde(rename = "dateTime", with = "as_localtime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::ShiftNum)]
    #[serde(rename = "shiftNumber")]
    pub shift_number: <fields::ShiftNum as FieldInternal>::Type,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(rename = "fiscalSign", with = "fiscal_sign_6")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::KktVer)]
    #[serde(rename = "kktVersion")]
    pub kkt_version: <fields::KktVer as FieldInternal>::Type,
    #[ffd(tag = fields::KktFfdVer)]
    #[serde(rename = "documentKktVersion")]
    pub document_kkt_version: <fields::KktFfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: Option<<fields::RetailPlaceAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlace")]
    pub retail_place: Option<<fields::RetailPlace as FieldInternal>::Type>,
    #[ffd(tag = fields::Operator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<<fields::Operator as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::OfdResponseTimeoutFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "ofdResponseTimeoutSign")]
    pub ofd_response_timeout_sign: Option<<fields::OfdResponseTimeoutFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveReplacementRequiredFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "fiscalDriveReplaceRequiredSign")]
    pub fiscal_drive_replace_required_sign:
        Option<<fields::DriveReplacementRequiredFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveMemoryFullFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "fiscalDriveMemoryExceededSign")]
    pub fiscal_drive_memory_exceeded_sign:
        Option<<fields::DriveMemoryFullFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveResourceExhaustionFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "fiscalDriveExhaustionSign")]
    pub fiscal_drive_exhaustion_sign:
        Option<<fields::DriveResourceExhaustionFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorMessage)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorMessage")]
    pub operator_message: Option<<fields::OperatorMessage as FieldInternal>::Type>,
    #[ffd(tag = fields::OpenShiftAdditionalProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropsOS")]
    pub additional_props_os: Option<<fields::OpenShiftAdditionalProp as FieldInternal>::Type>,
    #[ffd(tag = fields::OpenShiftAdditionalData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalDataOS")]
    pub additional_data_os: Option<<fields::OpenShiftAdditionalData as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct CurrentStateReport {
    #[ffd(special = "tag")]
    pub code: Constant<21>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::UserInn)]
    #[serde(rename = "userInn")]
    pub user_inn: <fields::UserInn as FieldInternal>::Type,
    #[ffd(tag = fields::DateTime)]
    #[serde(rename = "dateTime", with = "as_localtime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::FiscalSignValidityPeriod)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyResource")]
    pub key_resource: Option<<fields::FiscalSignValidityPeriod as FieldInternal>::Type>,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(rename = "fiscalSign", with = "fiscal_sign_6")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: Option<<fields::RetailPlaceAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlace")]
    pub retail_place: Option<<fields::RetailPlace as FieldInternal>::Type>,
    #[ffd(tag = fields::ShiftNum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shiftNumber")]
    pub shift_number: Option<<fields::ShiftNum as FieldInternal>::Type>,
    #[ffd(tag = fields::OfflineModeFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "offlineMode")]
    pub offline_mode: Option<<fields::OfflineModeFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedDocNum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notTransmittedDocumentNumber")]
    pub not_transmitted_document_number:
        Option<<fields::UntransmittedDocNum as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedDocCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notTransmittedDocumentsQuantity")]
    pub not_transmitted_documents_quantity:
        Option<<fields::UntransmittedDocCount as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedNotificationCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "undeliveredNotificationsNumber")]
    pub undelivered_notifications_number:
        Option<<fields::UntransmittedNotificationCount as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedDocDateTime)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notTransmittedDocumentsDateTime", with = "as_localtime_opt")]
    pub not_transmitted_documents_date_time:
        Option<<fields::UntransmittedDocDateTime as FieldInternal>::Type>,
    #[ffd(tag = fields::CurrentStateAdditionalAttribute)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropsCSR")]
    pub additional_props_csr:
        Option<<fields::CurrentStateAdditionalAttribute as FieldInternal>::Type>,
    #[ffd(tag = fields::CurrentStateAdditionalData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalDataCSR")]
    pub additional_data_csr: Option<<fields::CurrentStateAdditionalData as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fiscalDriveSumReports")]
    pub fiscal_drive_sum_reports: Option<DriveStats>,
    #[ffd(tag = fields::DriveUntransmittedStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notTransmittedDocumentsSumReports")]
    pub not_transmitted_documents_sum_reports: Option<CorrectionCounters>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct ReceiptBso<const T: u16> {
    #[ffd(special = "tag")]
    pub code: Constant<T>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::ReceiptNum)]
    #[serde(rename = "requestNumber")]
    pub request_number: <fields::ReceiptNum as FieldInternal>::Type,
    #[ffd(tag = fields::DateTime)]
    #[serde(with = "as_localtime")]
    #[serde(rename = "dateTime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::ShiftNum)]
    #[serde(rename = "shiftNumber")]
    pub shift_number: <fields::ShiftNum as FieldInternal>::Type,
    #[ffd(tag = fields::PaymentType)]
    #[serde(rename = "operationType")]
    pub operation_type: <fields::PaymentType as FieldInternal>::Type,
    #[ffd(tag = fields::TaxType)]
    #[serde(rename = "appliedTaxationType")]
    pub applied_taxation_type: <fields::TaxType as FieldInternal>::Type,
    /// Not documented, probably an alias for appliedTaxationType for misbehaving software
    #[ffd(tag = fields::TaxType)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taxationType")]
    pub taxation_type: Option<<fields::TaxType as FieldInternal>::Type>,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: Option<<fields::RetailPlaceAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlace")]
    pub retail_place: Option<<fields::RetailPlace as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalSum)]
    #[serde(rename = "totalSum")]
    pub total_sum: <fields::TotalSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalCashSum)]
    #[serde(rename = "cashTotalSum")]
    pub cash_total_sum: <fields::TotalCashSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalEcashSum)]
    #[serde(rename = "ecashTotalSum")]
    pub ecash_total_sum: <fields::TotalEcashSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalPrepaidSum)]
    #[serde(rename = "prepaidSum")]
    pub prepaid_sum: <fields::TotalPrepaidSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalCreditSum)]
    #[serde(rename = "creditSum")]
    pub credit_sum: <fields::TotalCreditSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalProvisionSum)]
    #[serde(rename = "provisionSum")]
    pub provision_sum: <fields::TotalProvisionSum as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(with = "fiscal_sign_6")]
    #[serde(rename = "fiscalSign")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::UserInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userInn")]
    pub user_inn: Option<<fields::UserInn as FieldInternal>::Type>,
    #[ffd(tag = fields::MachineNumber)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "machineNumber")]
    pub machine_number: Option<<fields::MachineNumber as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerPhoneOrEmail)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buyerPhoneOrAddress")]
    pub buyer_phone_or_address: Option<<fields::BuyerPhoneOrEmail as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat20Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds18")]
    pub nds_18: Option<<fields::TotalVat20Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat10Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds10")]
    pub nds_10: Option<<fields::TotalVat10Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalSumWithVat0)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds0")]
    pub nds_0: Option<<fields::TotalSumWithVat0 as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalSumWithNoVat)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ndsNo")]
    pub nds_no: Option<<fields::TotalSumWithNoVat as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat20_120Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds18118")]
    pub nds_18_118: Option<<fields::TotalVat20_120Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat10_110Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds10110")]
    pub nds_10_110: Option<<fields::TotalVat10_110Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::OnlineKktFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "internetSign")]
    pub internet_sign: Option<<fields::OnlineKktFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::ReceiptSenderEmail)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sellerAddress")]
    pub seller_address: Option<<fields::ReceiptSenderEmail as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paymentAgentType")]
    pub payment_agent_type: Option<<fields::PaymentAgentTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorPhone")]
    pub transfer_operator_phone: Vec<<fields::TransferOperatorPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentOperation)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "paymentAgentOperation")]
    pub payment_agent_operation: Vec<<fields::PaymentAgentOperation as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "paymentAgentPhone")]
    pub payment_agent_phone: Vec<<fields::PaymentAgentPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentOperatorPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "paymentOperatorPhone")]
    pub payment_operator_phone: Vec<<fields::PaymentOperatorPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorName)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorName")]
    pub transfer_operator_name: Vec<<fields::TransferOperatorName as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorAddress)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorAddress")]
    pub transfer_operator_address: Vec<<fields::TransferOperatorAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorInn)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorInn")]
    pub transfer_operator_inn: Vec<<fields::TransferOperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::FnsUrl)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fnsUrl")]
    pub fns_url: Option<<fields::FnsUrl as FieldInternal>::Type>,
    #[ffd(tag = fields::AdditionalReceiptProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propertiesData")]
    pub properties_data: Option<<fields::AdditionalReceiptProp as FieldInternal>::Type>,
    #[ffd(tag = fields::MarkedProductCheckResults)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "checkingLabeledProdResult")]
    pub checking_labeled_prod_result:
        Option<<fields::MarkedProductCheckResults as FieldInternal>::Type>,
    #[ffd(tag = fields::Operator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<<fields::Operator as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::ReceiptItem)]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub items: Vec<Item>,
    #[ffd(tag = fields::AdditionalUserProp)]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "one_or_singleton::deserialize"
    )]
    pub properties: Option<Property>,
    #[ffd(tag = fields::IndustryReceiptProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "industryReceiptDetails")]
    pub industry_receipt_details: Option<IndustryDetails>,
    #[ffd(tag = fields::BuyerInfo)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buyerInformation")]
    pub buyer_information: Option<BuyerInfo>,
    #[ffd(tag = fields::OperationalProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operationalDetails")]
    pub operational_details: Option<OperationalDetails>,
}

pub type Receipt = ReceiptBso<3>;
pub type Bso = ReceiptBso<4>;

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[ffd(tag = fields::ItemName)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<<fields::ItemName as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentMethod)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<<fields::PaymentMethod as FieldInternal>::Type>,
    #[ffd(tag = fields::ItemType)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<<fields::ItemType as FieldInternal>::Type>,
    #[ffd(tag = fields::ItemTotalPrice)]
    pub sum: <fields::ItemTotalPrice as FieldInternal>::Type,
    #[ffd(tag = fields::ItemUnitPrice)]
    pub price: <fields::ItemUnitPrice as FieldInternal>::Type,
    #[ffd(tag = fields::ItemQuantity)]
    pub quantity: <fields::ItemQuantity as FieldInternal>::Type,
    #[ffd(tag = fields::AdditionalItemProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties_item: Option<<fields::AdditionalItemProp as FieldInternal>::Type>,
    #[ffd(tag = fields::ItemAgentTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_agent_by_product_type: Option<<fields::ItemAgentTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::Unit)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<<fields::Unit as FieldInternal>::Type>,
    #[ffd(tag = fields::SupplierInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_inn: Option<<fields::SupplierInn as FieldInternal>::Type>,
    #[ffd(tag = fields::ItemQuantityUnit)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items_quantity_measure: Option<<fields::ItemQuantityUnit as FieldInternal>::Type>,
    #[ffd(tag = fields::MarkingCodeControlCode)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_code: Option<<fields::MarkingCodeControlCode as FieldInternal>::Type>,
    #[ffd(tag = fields::MarkingCodeProcessingMode)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label_code_proces_mode: Option<<fields::MarkingCodeProcessingMode as FieldInternal>::Type>,
    #[ffd(tag = fields::ProductInfoCheckResult)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checking_prod_information_result:
        Option<<fields::ProductInfoCheckResult as FieldInternal>::Type>,
    #[ffd(tag = fields::OriginCountry)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_country_code: Option<<fields::OriginCountry as FieldInternal>::Type>,
    #[ffd(tag = fields::CustomsDeclarationNum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_entry_num: Option<<fields::CustomsDeclarationNum as FieldInternal>::Type>,
    #[ffd(tag = fields::ItemUnitVat)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_nds: Option<<fields::ItemUnitVat as FieldInternal>::Type>,
    #[ffd(tag = fields::ExciseDuty)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excise_duty: Option<<fields::ExciseDuty as FieldInternal>::Type>,
    #[ffd(tag = fields::VatRate)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nds: Option<<fields::VatRate as FieldInternal>::Type>,
    #[ffd(tag = fields::ItemTotalVat)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nds_sum: Option<<fields::ItemTotalVat as FieldInternal>::Type>,
    #[ffd(tag = fields::ProductCode)]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    pub product_code: Option<Vec<u8>>,
    #[ffd(tag = fields::ProductCodeNew)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_code_new: Option<ProductCodeNew>,
    #[ffd(tag = fields::SupplierData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<ProviderData>,
    #[ffd(tag = fields::PaymentAgentData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_agent_data: Option<PaymentAgentData>,
    #[ffd(tag = fields::MarkedProductFractionalQuantity)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labeled_prod_fractional_quantity: Option<Fraction>,
    #[ffd(tag = fields::IndustryItemProp)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub items_industry_details: Vec<IndustryDetails>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    #[ffd(tag = fields::AdditionalUserPropName)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property_name: Option<<fields::AdditionalUserPropName as FieldInternal>::Type>,
    #[ffd(tag = fields::AdditionalUserPropValue)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property_value: Option<<fields::AdditionalUserPropValue as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BuyerInfo {
    #[ffd(tag = fields::Client)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer: Option<<fields::Client as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_inn: Option<<fields::BuyerInn as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerBirthday)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_birthday: Option<<fields::BuyerBirthday as FieldInternal>::Type>,
    #[ffd(tag = fields::Citizenship)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_citizenship: Option<<fields::Citizenship as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerIdType)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_document_code: Option<<fields::BuyerIdType as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerIdData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_document_data: Option<<fields::BuyerIdData as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_address: Option<<fields::BuyerAddress as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationalDetails {
    #[ffd(tag = fields::OperationDateTime)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "as_localtime_opt")]
    pub date_time: Option<<fields::OperationDateTime as FieldInternal>::Type>,
    #[ffd(tag = fields::OperationId)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<<fields::OperationId as FieldInternal>::Type>,
    #[ffd(tag = fields::OperationData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_data: Option<<fields::OperationData as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct ProductCodeNew {
    #[ffd(tag = fields::KtN)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub undefined: Option<<fields::KtN as FieldInternal>::Type>,
    #[ffd(tag = fields::KtEan8)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub ean8: Option<<fields::KtEan8 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtEan13)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub ean13: Option<<fields::KtEan13 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtItf14)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub itf14: Option<<fields::KtItf14 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtGs1_0)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub gs1: Option<<fields::KtGs1_0 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtGs1M)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub gs1m: Option<<fields::KtGs1M as FieldInternal>::Type>,
    #[ffd(tag = fields::KtKmk)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub kmk: Option<<fields::KtKmk as FieldInternal>::Type>,
    #[ffd(tag = fields::KtMi)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub mi: Option<<fields::KtMi as FieldInternal>::Type>,
    #[ffd(tag = fields::KtEgais2_0)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub egais2: Option<<fields::KtEgais2_0 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtEgais3_0)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub egais3: Option<<fields::KtEgais3_0 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtF1)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub f1: Option<<fields::KtF1 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtF2)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub f2: Option<<fields::KtF2 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtF3)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub f3: Option<<fields::KtF3 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtF4)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub f4: Option<<fields::KtF4 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtF5)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub f5: Option<<fields::KtF5 as FieldInternal>::Type>,
    #[ffd(tag = fields::KtF6)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(deserialize_with = "marking_code_opt::deserialize")]
    pub f6: Option<<fields::KtF6 as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderData {
    #[ffd(tag = fields::SupplierPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(with = "one_or_many")]
    pub provider_phone: Vec<<fields::SupplierPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::SupplierName)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<<fields::SupplierName as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentAgentData {
    #[ffd(tag = fields::TransferOperatorPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub transfer_operator_phone: Vec<<fields::TransferOperatorPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentOperation)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub payment_agent_operation: Vec<<fields::PaymentAgentOperation as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub payment_agent_phone: Vec<<fields::PaymentAgentPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentOperatorPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub payment_operator_phone: Vec<<fields::PaymentOperatorPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorName)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub transfer_operator_name: Vec<<fields::TransferOperatorName as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorAddress)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub transfer_operator_address: Vec<<fields::TransferOperatorAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorInn)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub transfer_operator_inn: Vec<<fields::TransferOperatorInn as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fraction {
    #[ffd(tag = fields::FractionalPart)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fractional_part: Option<<fields::FractionalPart as FieldInternal>::Type>,
    #[ffd(tag = fields::Numerator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numerator: Option<<fields::Numerator as FieldInternal>::Type>,
    #[ffd(tag = fields::Denominator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denominator: Option<<fields::Denominator as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndustryDetails {
    #[ffd(tag = fields::FoivId)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id_foiv: Option<<fields::FoivId as FieldInternal>::Type>,
    #[ffd(tag = fields::FoundationDocDateTime)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foundation_doc_date_time: Option<<fields::FoundationDocDateTime as FieldInternal>::Type>,
    #[ffd(tag = fields::FoundationDocNum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foundation_doc_number: Option<<fields::FoundationDocNum as FieldInternal>::Type>,
    #[ffd(tag = fields::IndustryPropValue)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_prop_value: Option<<fields::IndustryPropValue as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct ReceiptBsoCorrection<const T: u16> {
    #[ffd(special = "tag")]
    pub code: Constant<T>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::ReceiptNum)]
    #[serde(rename = "requestNumber")]
    pub request_number: <fields::ReceiptNum as FieldInternal>::Type,
    #[ffd(tag = fields::DateTime)]
    #[serde(with = "as_localtime")]
    #[serde(rename = "dateTime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::ShiftNum)]
    #[serde(rename = "shiftNumber")]
    pub shift_number: <fields::ShiftNum as FieldInternal>::Type,
    #[ffd(tag = fields::PaymentType)]
    #[serde(rename = "operationType")]
    pub operation_type: <fields::PaymentType as FieldInternal>::Type,
    #[ffd(tag = fields::TaxType)]
    #[serde(rename = "appliedTaxationType")]
    pub applied_taxation_type: <fields::TaxType as FieldInternal>::Type,
    /// Not documented, probably an alias for appliedTaxationType for misbehaving software
    #[ffd(tag = fields::TaxType)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "taxationType")]
    pub taxation_type: Option<<fields::TaxType as FieldInternal>::Type>,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: Option<<fields::RetailPlaceAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlace")]
    pub retail_place: Option<<fields::RetailPlace as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalSum)]
    #[serde(rename = "totalSum")]
    pub total_sum: <fields::TotalSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalCashSum)]
    #[serde(rename = "cashTotalSum")]
    pub cash_total_sum: <fields::TotalCashSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalEcashSum)]
    #[serde(rename = "ecashTotalSum")]
    pub ecash_total_sum: <fields::TotalEcashSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalPrepaidSum)]
    #[serde(rename = "prepaidSum")]
    pub prepaid_sum: <fields::TotalPrepaidSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalCreditSum)]
    #[serde(rename = "creditSum")]
    pub credit_sum: <fields::TotalCreditSum as FieldInternal>::Type,
    #[ffd(tag = fields::TotalProvisionSum)]
    #[serde(rename = "provisionSum")]
    pub provision_sum: <fields::TotalProvisionSum as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(with = "fiscal_sign_6")]
    #[serde(rename = "fiscalSign")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::UserInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "userInn")]
    pub user_inn: Option<<fields::UserInn as FieldInternal>::Type>,
    #[ffd(tag = fields::MachineNumber)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "machineNumber")]
    pub machine_number: Option<<fields::MachineNumber as FieldInternal>::Type>,
    #[ffd(tag = fields::BuyerPhoneOrEmail)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buyerPhoneOrAddress")]
    pub buyer_phone_or_address: Option<<fields::BuyerPhoneOrEmail as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat20Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds18")]
    pub nds_18: Option<<fields::TotalVat20Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat10Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds10")]
    pub nds_10: Option<<fields::TotalVat10Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalSumWithVat0)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds0")]
    pub nds_0: Option<<fields::TotalSumWithVat0 as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalSumWithNoVat)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ndsNo")]
    pub nds_no: Option<<fields::TotalSumWithNoVat as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat20_120Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds18118")]
    pub nds_18_118: Option<<fields::TotalVat20_120Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::TotalVat10_110Sum)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nds10110")]
    pub nds_10_110: Option<<fields::TotalVat10_110Sum as FieldInternal>::Type>,
    #[ffd(tag = fields::OnlineKktFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "internetSign")]
    pub internet_sign: Option<<fields::OnlineKktFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::ReceiptSenderEmail)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sellerAddress")]
    pub seller_address: Option<<fields::ReceiptSenderEmail as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentTypes)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "paymentAgentType")]
    pub payment_agent_type: Option<<fields::PaymentAgentTypes as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorPhone")]
    pub transfer_operator_phone: Vec<<fields::TransferOperatorPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentOperation)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "paymentAgentOperation")]
    pub payment_agent_operation: Vec<<fields::PaymentAgentOperation as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentAgentPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "paymentAgentPhone")]
    pub payment_agent_phone: Vec<<fields::PaymentAgentPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::PaymentOperatorPhone)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "paymentOperatorPhone")]
    pub payment_operator_phone: Vec<<fields::PaymentOperatorPhone as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorName)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorName")]
    pub transfer_operator_name: Vec<<fields::TransferOperatorName as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorAddress)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorAddress")]
    pub transfer_operator_address: Vec<<fields::TransferOperatorAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::TransferOperatorInn)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    #[serde(rename = "transferOperatorInn")]
    pub transfer_operator_inn: Vec<<fields::TransferOperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::FnsUrl)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fnsUrl")]
    pub fns_url: Option<<fields::FnsUrl as FieldInternal>::Type>,
    #[ffd(tag = fields::AdditionalReceiptProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "propertiesData")]
    pub properties_data: Option<<fields::AdditionalReceiptProp as FieldInternal>::Type>,
    #[ffd(tag = fields::MarkedProductCheckResults)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "checkingLabeledProdResult")]
    pub checking_labeled_prod_result:
        Option<<fields::MarkedProductCheckResults as FieldInternal>::Type>,
    #[ffd(tag = fields::Operator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<<fields::Operator as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::ReceiptItem)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(deserialize_with = "one_or_many::deserialize")]
    pub items: Vec<Item>,
    #[ffd(tag = fields::AdditionalUserProp)]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "one_or_singleton::deserialize"
    )]
    pub properties: Option<Property>,
    #[ffd(tag = fields::IndustryReceiptProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "industryReceiptDetails")]
    pub industry_receipt_details: Option<IndustryDetails>,
    #[ffd(tag = fields::BuyerInfo)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "buyerInformation")]
    pub buyer_information: Option<BuyerInfo>,
    #[ffd(tag = fields::OperationalProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operationalDetails")]
    pub operational_details: Option<OperationalDetails>,
    #[ffd(tag = fields::CorrectionType)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "correctionType")]
    pub correction_type: Option<<fields::CorrectionType as FieldInternal>::Type>,
    #[ffd(tag = fields::CorrectionBasis)]
    #[serde(rename = "correctionBase")]
    pub correction_base: CorrectionBase,
}

pub type ReceiptCorrection = ReceiptBsoCorrection<31>;
pub type BsoCorrection = ReceiptBsoCorrection<41>;

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionBase {
    #[ffd(tag = fields::CorrectedPaymentDate)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "as_localtime_opt")]
    pub correction_document_date: Option<<fields::CorrectedPaymentDate as FieldInternal>::Type>,
    #[ffd(tag = fields::FnsActNumber)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correction_document_number: Option<<fields::FnsActNumber as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct CloseShift {
    #[ffd(special = "tag")]
    pub code: Constant<5>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::DateTime)]
    #[serde(rename = "dateTime", with = "as_localtime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::ShiftNum)]
    #[serde(rename = "shiftNumber")]
    pub shift_number: <fields::ShiftNum as FieldInternal>::Type,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::ReceiptCountPerShift)]
    #[serde(rename = "receiptQuantity")]
    pub receipt_quantity: <fields::ReceiptCountPerShift as FieldInternal>::Type,
    #[ffd(tag = fields::DocCountPerShift)]
    #[serde(rename = "documentsQuantity")]
    pub documents_quantity: <fields::DocCountPerShift as FieldInternal>::Type,
    #[ffd(tag = fields::FiscalSignValidityPeriod)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fdKeyResource")]
    pub fd_key_resource: Option<<fields::FiscalSignValidityPeriod as FieldInternal>::Type>,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(rename = "fiscalSign", with = "fiscal_sign_6")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::DriveStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fiscalDriveSumReports")]
    pub fiscal_drive_sum_reports: Option<DriveStats>,
    #[ffd(tag = fields::ShiftStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shiftSumReports")]
    pub shift_sum_reports: Option<DriveStats>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::Operator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<<fields::Operator as FieldInternal>::Type>,
    #[ffd(tag = fields::UserInn)]
    #[serde(rename = "userInn")]
    pub user_inn: <fields::UserInn as FieldInternal>::Type,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: Option<<fields::RetailPlaceAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlace")]
    pub retail_place: Option<<fields::RetailPlace as FieldInternal>::Type>,
    #[ffd(tag = fields::OfdResponseTimeoutFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "ofdResponseTimeoutSign")]
    pub ofd_response_timeout_sign: Option<<fields::OfdResponseTimeoutFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveReplacementRequiredFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "fiscalDriveReplaceRequiredSign")]
    pub fiscal_drive_replace_required_sign:
        Option<<fields::DriveReplacementRequiredFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveMemoryFullFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "fiscalDriveMemoryExceededSign")]
    pub fiscal_drive_memory_exceeded_sign:
        Option<<fields::DriveMemoryFullFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::DriveResourceExhaustionFlag)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(with = "bool_num_opt")]
    #[serde(rename = "fiscalDriveExhaustionSign")]
    pub fiscal_drive_exhaustion_sign:
        Option<<fields::DriveResourceExhaustionFlag as FieldInternal>::Type>,
    #[ffd(tag = fields::OperatorMessage)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorMessage")]
    pub operator_message: Option<<fields::OperatorMessage as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedDocCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notTransmittedDocumentsQuantity")]
    pub not_transmitted_documents_quantity:
        Option<<fields::UntransmittedDocCount as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedDocDateTime)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "notTransmittedDocumentsDateTime", with = "as_localtime_opt")]
    pub not_transmitted_documents_date_time:
        Option<<fields::UntransmittedDocDateTime as FieldInternal>::Type>,
    #[ffd(tag = fields::UntransmittedNotificationCount)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "undeliveredNotificationsNumber")]
    pub undelivered_notifications_number:
        Option<<fields::UntransmittedNotificationCount as FieldInternal>::Type>,
    #[ffd(tag = fields::IncorrectMarkingCodesFlags)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "invalidLabelCodesSign")]
    pub invalid_label_codes_sign:
        Option<<fields::IncorrectMarkingCodesFlags as FieldInternal>::Type>,
    #[ffd(tag = fields::IncorrectRequestsAndNotificationsFlags)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "invalidRequestsNotificationsSign")]
    pub invalid_requests_notifications_sign:
        Option<<fields::IncorrectRequestsAndNotificationsFlags as FieldInternal>::Type>,
    #[ffd(tag = fields::CloseShiftAdditionalProp)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropsCS")]
    pub additional_props_cs: Option<<fields::CloseShiftAdditionalProp as FieldInternal>::Type>,
    #[ffd(tag = fields::CloseShiftAdditionalData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalDataCS")]
    pub additional_data_cs: Option<<fields::CloseShiftAdditionalData as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Default, Ffd, Deserialize, Serialize)]
pub struct CloseArchive {
    #[ffd(special = "tag")]
    pub code: Constant<6>,
    #[ffd(special = "fps")]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "fiscal_sign_8_opt"
    )]
    #[serde(rename = "messageFiscalSign")]
    pub message_fiscal_sign: Option<[u8; 8]>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "base64_vec_opt"
    )]
    #[serde(rename = "rawData")]
    pub raw_data: Option<Vec<u8>>,
    #[ffd(tag = fields::FfdVer)]
    #[serde(rename = "fiscalDocumentFormatVer")]
    pub fiscal_document_format_ver: <fields::FfdVer as FieldInternal>::Type,
    #[ffd(tag = fields::User)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<<fields::User as FieldInternal>::Type>,
    #[ffd(tag = fields::UserInn)]
    #[serde(rename = "userInn")]
    pub user_inn: <fields::UserInn as FieldInternal>::Type,
    #[ffd(tag = fields::RetailPlaceAddress)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlaceAddress")]
    pub retail_place_address: Option<<fields::RetailPlaceAddress as FieldInternal>::Type>,
    #[ffd(tag = fields::RetailPlace)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "retailPlace")]
    pub retail_place: Option<<fields::RetailPlace as FieldInternal>::Type>,
    #[ffd(tag = fields::DateTime)]
    #[serde(rename = "dateTime", with = "as_localtime")]
    pub date_time: <fields::DateTime as FieldInternal>::Type,
    #[ffd(tag = fields::ShiftNum)]
    #[serde(rename = "shiftNumber")]
    pub shift_number: <fields::ShiftNum as FieldInternal>::Type,
    #[ffd(tag = fields::KktRegNum)]
    #[serde(rename = "kktRegId")]
    pub kkt_reg_id: <fields::KktRegNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocNum)]
    #[serde(rename = "fiscalDocumentNumber")]
    pub fiscal_document_number: <fields::DocNum as FieldInternal>::Type,
    #[ffd(tag = fields::DriveNum)]
    #[serde(rename = "fiscalDriveNumber")]
    pub fiscal_drive_number: <fields::DriveNum as FieldInternal>::Type,
    #[ffd(tag = fields::DocFiscalSign)]
    #[serde(rename = "fiscalSign", with = "fiscal_sign_6")]
    pub fiscal_sign: <fields::DocFiscalSign as FieldInternal>::Type,
    #[ffd(tag = fields::DriveStats)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fiscalDriveSumReports")]
    pub fiscal_drive_sum_reports: Option<DriveStats>,
    #[ffd(tag = fields::OperatorInn)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operatorInn")]
    pub operator_inn: Option<<fields::OperatorInn as FieldInternal>::Type>,
    #[ffd(tag = fields::Operator)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<<fields::Operator as FieldInternal>::Type>,
    #[ffd(tag = fields::CloseArchiveAdditionalAttribute)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalPropsCA")]
    pub additional_props_ca:
        Option<<fields::CloseArchiveAdditionalAttribute as FieldInternal>::Type>,
    #[ffd(tag = fields::CloseArchiveAdditionalData)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additionalDataCA")]
    pub additional_data_ca: Option<<fields::CloseArchiveAdditionalData as FieldInternal>::Type>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::large_enum_variant)]
pub enum Document {
    FiscalReport(FiscalReport),
    FiscalReportCorrection(FiscalReportCorrection),
    OpenShift(OpenShift),
    CurrentStateReport(CurrentStateReport),
    Receipt(Receipt),
    ReceiptCorrection(ReceiptCorrection),
    Bso(Bso),
    BsoCorrection(BsoCorrection),
    CloseShift(CloseShift),
    CloseArchive(CloseArchive),
}

impl TryFrom<Object> for Document {
    type Error = Error;
    fn try_from(value: Object) -> Result<Self, Self::Error> {
        Self::from_bytes(value.into_bytes()?)
    }
}

impl TryFrom<Document> for Object {
    type Error = Error;
    fn try_from(value: Document) -> Result<Self, Self::Error> {
        Self::from_bytes(value.into_bytes()?)
    }
}

impl TlvType for Document {
    fn from_bytes(bytes: Vec<u8>) -> fiscal_data::Result<Self> {
        let Some(&[a, b]) = bytes.get(0..2) else {
            return Err(Error::Eof);
        };
        let tag = u16::from_le_bytes([a, b]);
        Ok(match tag {
            1 => Self::FiscalReport(TlvType::from_bytes(bytes)?),
            11 => Self::FiscalReportCorrection(TlvType::from_bytes(bytes)?),
            2 => Self::OpenShift(TlvType::from_bytes(bytes)?),
            21 => Self::CurrentStateReport(TlvType::from_bytes(bytes)?),
            3 => Self::Receipt(TlvType::from_bytes(bytes)?),
            31 => Self::ReceiptCorrection(TlvType::from_bytes(bytes)?),
            4 => Self::Bso(TlvType::from_bytes(bytes)?),
            41 => Self::BsoCorrection(TlvType::from_bytes(bytes)?),
            5 => Self::CloseShift(TlvType::from_bytes(bytes)?),
            6 => Self::CloseArchive(TlvType::from_bytes(bytes)?),
            _ => return Err(Error::InvalidFormat),
        })
    }
    fn into_bytes(self) -> fiscal_data::Result<Vec<u8>> {
        match self {
            Self::FiscalReport(x) => x.into_bytes(),
            Self::FiscalReportCorrection(x) => x.into_bytes(),
            Self::OpenShift(x) => x.into_bytes(),
            Self::CurrentStateReport(x) => x.into_bytes(),
            Self::Receipt(x) => x.into_bytes(),
            Self::ReceiptCorrection(x) => x.into_bytes(),
            Self::Bso(x) => x.into_bytes(),
            Self::BsoCorrection(x) => x.into_bytes(),
            Self::CloseShift(x) => x.into_bytes(),
            Self::CloseArchive(x) => x.into_bytes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{fields, Object, TlvType};

    use super::Document;

    #[test]
    fn test_roundtrip_receipt() {
        let json = r#"{
            "receipt": {
                "messageFiscalSign": 9297210640046662345,
                "dateTime": 1800000000,
                "buyerPhoneOrAddress": "none",
                "cashTotalSum": 0,
                "code": 3,
                "creditSum": 0,
                "ecashTotalSum": 123456,
                "fiscalDocumentFormatVer": 4,
                "fiscalDocumentNumber": 12345,
                "fiscalDriveNumber": "7284441234567890",
                "fiscalSign": 12345678,
                "fnsUrl": "www.nalog.gov.ru",
                "items": [
                    {
                        "name": "Тест 1",
                        "nds": 1,
                        "ndsSum": 2667,
                        "paymentType": 4,
                        "price": 15999,
                        "productType": 1,
                        "quantity": 1,
                        "sum": 15999
                    },
                    {
                        "name": "Тест 2",
                        "nds": 2,
                        "ndsSum": 1582,
                        "paymentType": 4,
                        "price": 8699,
                        "productType": 1,
                        "quantity": 2,
                        "sum": 17398
                    },
                    {
                        "name": "Тест 3",
                        "nds": 1,
                        "ndsSum": 1667,
                        "paymentType": 4,
                        "price": 9999,
                        "productType": 2,
                        "quantity": 1,
                        "sum": 9999
                    },
                    {
                        "name": "Тест 4",
                        "nds": 1,
                        "ndsSum": 216,
                        "paymentType": 4,
                        "price": 58999,
                        "productType": 1,
                        "quantity": 0.022,
                        "sum": 1298
                    },
                    {
                        "name": "Тест 5",
                        "nds": 2,
                        "ndsSum": 921,
                        "paymentType": 4,
                        "price": 11999,
                        "productType": 1,
                        "quantity": 0.844,
                        "sum": 10127
                    }
                ],
                "kktRegId": "0006346012345678    ",
                "machineNumber": "123456789123456789",
                "nds10": 1234,
                "nds18": 12345,
                "operationType": 1,
                "prepaidSum": 0,
                "provisionSum": 0,
                "requestNumber": 43,
                "retailPlace": "Тест",
                "retailPlaceAddress": "Адрес",
                "sellerAddress": "noreply@example.org",
                "shiftNumber": 123,
                "taxationType": 1,
                "appliedTaxationType": 1,
                "totalSum": 123456,
                "user": "ООО \"Тест\"",
                "userInn": "1234567890  "
            }
        }"#;
        let obj1 = serde_json::from_str::<serde_json::Value>(json).unwrap();
        let val = serde_json::from_str::<Document>(json).unwrap();
        let obj2 = serde_json::from_str::<serde_json::Value>(&serde_json::to_string(&val).unwrap())
            .unwrap();
        assert_eq!(
            serde_json::to_string(&obj1).unwrap(),
            serde_json::to_string(&obj2).unwrap()
        );
        let ser = Object::try_from(val.clone()).unwrap();
        eprintln!("{:?}", val.clone().into_bytes());
        assert_eq!(
            ser.first_obj()
                .unwrap()
                .unwrap()
                .get::<fields::KktRegNum>()
                .unwrap()
                .unwrap(),
            match val {
                Document::Receipt(x) => x.kkt_reg_id.trim().to_owned(),
                _ => panic!(),
            }
        )
    }
}
