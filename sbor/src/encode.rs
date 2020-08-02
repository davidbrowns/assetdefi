extern crate alloc;
use alloc::vec::Vec;

use crate::*;

pub trait Encode {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.write_type(Self::sbor_type());
        self.encode_value(encoder);
    }

    fn encode_value(&self, encoder: &mut Encoder);

    fn sbor_type() -> u8;
}

pub struct Encoder {
    buf: Vec<u8>,
    with_metadata: bool,
}

impl Encoder {
    pub fn new(capacity: usize, with_metadata: bool) -> Self {
        Self {
            buf: Vec::with_capacity(capacity),
            with_metadata,
        }
    }

    pub fn with_metadata() -> Self {
        Self::new(256, true)
    }

    pub fn no_metadata() -> Self {
        Self::new(256, false)
    }

    #[inline(always)]
    pub fn write_type(&mut self, ty: u8) {
        if self.with_metadata {
            self.buf.push(ty);
        }
    }

    #[inline(always)]
    pub fn write_name(&mut self, value: &str) {
        if self.with_metadata {
            self.write_type(TYPE_STRING);
            self.write_len(value.len());
            self.buf.extend(value.as_bytes());
        }
    }

    #[inline(always)]
    pub fn write_len(&mut self, len: usize) {
        self.buf.extend(&(len as u16).to_le_bytes());
    }

    #[inline(always)]
    pub fn write_index(&mut self, len: usize) {
        self.buf.push(len as u8);
    }

    #[inline(always)]
    pub fn write_u8(&mut self, len: u8) {
        self.buf.push(len as u8);
    }

    #[inline(always)]
    pub fn write_slice(&mut self, slice: &[u8]) {
        self.buf.extend(slice);
    }
}

impl Into<Vec<u8>> for Encoder {
    fn into(self) -> Vec<u8> {
        self.buf
    }
}

// Implementation for basic types:
// - We keep one flat implementation per type, i.e., the `encode()` function;
// - Everything else is inlined.

impl Encode for () {
    #[inline]
    fn encode_value(&self, _encoder: &mut Encoder) {}

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_UNIT
    }
}

impl Encode for bool {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_u8(if *self { 1u8 } else { 0u8 })
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_BOOL
    }
}

impl Encode for i8 {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_u8(*self as u8);
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_I8
    }
}

impl Encode for u8 {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_u8(*self);
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_U8
    }
}

macro_rules! encode_basic_type {
    ($type:ident, $sbor_type:ident) => {
        impl Encode for $type {
            #[inline]
            fn encode_value(&self, encoder: &mut Encoder) {
                encoder.write_slice(&(*self).to_le_bytes());
            }

            #[inline]
            fn sbor_type() -> u8 {
                $sbor_type
            }
        }
    };
}

encode_basic_type!(i16, TYPE_I16);
encode_basic_type!(i32, TYPE_I32);
encode_basic_type!(i64, TYPE_I64);
encode_basic_type!(i128, TYPE_I128);
encode_basic_type!(u16, TYPE_U16);
encode_basic_type!(u32, TYPE_U32);
encode_basic_type!(u64, TYPE_U64);
encode_basic_type!(u128, TYPE_U128);

impl Encode for str {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_len(self.len());
        encoder.write_slice(self.as_bytes());
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_STRING
    }
}

impl Encode for String {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_len(self.len());
        encoder.write_slice(self.as_bytes());
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_STRING
    }
}

impl<T: Encode> Encode for Option<T> {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        match self {
            None => {
                encoder.write_index(0);
            }
            Some(v) => {
                encoder.write_index(1);
                v.encode(encoder);
            }
        }
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_OPTION
    }
}

impl<T: Encode, const N: usize> Encode for [T; N] {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_len(self.len());
        encoder.write_type(T::sbor_type());
        for v in self {
            v.encode_value(encoder);
        }
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_ARRAY
    }
}

impl<T: Encode> Encode for Vec<T> {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_len(self.len());
        encoder.write_type(T::sbor_type());
        for v in self {
            v.encode_value(encoder);
        }
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_VEC
    }
}

// TODO expand to different lengths
impl<A: Encode, B: Encode> Encode for (A, B) {
    #[inline]
    fn encode_value(&self, encoder: &mut Encoder) {
        encoder.write_len(2);

        self.0.encode(encoder);
        self.1.encode(encoder);
    }

    #[inline]
    fn sbor_type() -> u8 {
        TYPE_TUPLE
    }
}

#[cfg(test)]
mod tests {
    use super::{Encode, Encoder};

    #[test]
    pub fn test_encoding() {
        let mut enc = Encoder::with_metadata();
        ().encode(&mut enc);
        true.encode(&mut enc);
        1i8.encode(&mut enc);
        1i16.encode(&mut enc);
        1i32.encode(&mut enc);
        1i64.encode(&mut enc);
        1i128.encode(&mut enc);
        1u8.encode(&mut enc);
        1u16.encode(&mut enc);
        1u32.encode(&mut enc);
        1u64.encode(&mut enc);
        1u128.encode(&mut enc);
        "hello".encode(&mut enc);
        Some(1u32).encode(&mut enc);
        [1u32, 2u32, 3u32].encode(&mut enc);
        vec![1u32, 2u32, 3u32].encode(&mut enc);
        (1u32, 2u32).encode(&mut enc);

        let bytes: Vec<u8> = enc.into();
        assert_eq!(
            vec![
                0, // unit
                1, 1, // bool
                2, 1, // i8
                3, 1, 0, // i16
                4, 1, 0, 0, 0, // i32
                5, 1, 0, 0, 0, 0, 0, 0, 0, // i64
                6, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // i128
                7, 1, // u8
                8, 1, 0, // u16
                9, 1, 0, 0, 0, // u32
                10, 1, 0, 0, 0, 0, 0, 0, 0, // u64
                11, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // u128
                12, 5, 0, 104, 101, 108, 108, 111, // string
                13, 1, 9, 1, 0, 0, 0, // option
                14, 3, 0, 9, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // array
                15, 3, 0, 9, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // vector
                16, 2, 0, 9, 1, 0, 0, 0, 9, 2, 0, 0, 0 // tuple
            ],
            bytes
        );
    }

    #[test]
    pub fn test_encoding_no_metadata() {
        let mut enc = Encoder::no_metadata();
        ().encode(&mut enc);
        true.encode(&mut enc);
        1i8.encode(&mut enc);
        1i16.encode(&mut enc);
        1i32.encode(&mut enc);
        1i64.encode(&mut enc);
        1i128.encode(&mut enc);
        1u8.encode(&mut enc);
        1u16.encode(&mut enc);
        1u32.encode(&mut enc);
        1u64.encode(&mut enc);
        1u128.encode(&mut enc);
        "hello".encode(&mut enc);
        Some(1u32).encode(&mut enc);
        [1u32, 2u32, 3u32].encode(&mut enc);
        vec![1u32, 2u32, 3u32].encode(&mut enc);
        (1u32, 2u32).encode(&mut enc);

        let bytes: Vec<u8> = enc.into();
        assert_eq!(
            vec![
                // unit
                1, // bool
                1, // i8
                1, 0, // i16
                1, 0, 0, 0, // i32
                1, 0, 0, 0, 0, 0, 0, 0, // i64
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // i128
                1, // u8
                1, 0, // u16
                1, 0, 0, 0, // u32
                1, 0, 0, 0, 0, 0, 0, 0, // u64
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // u128
                5, 0, 104, 101, 108, 108, 111, // string
                1, 1, 0, 0, 0, // option
                3, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // array
                3, 0, 1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, // vector
                2, 0, 1, 0, 0, 0, 2, 0, 0, 0 // tuple
            ],
            bytes
        );
    }
}