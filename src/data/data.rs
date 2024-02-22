use std::u8;

use crate::data::{array, tables};
use crate::header;

#[derive(Debug, Clone, PartialEq)]
pub enum Precision {
    U8(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

impl Precision {
    pub fn to_u8(&self) -> u8 {
        match self {
            Precision::U8(value) => *value,
            Precision::I16(value) => *value as u8,
            Precision::I32(value) => *value as u8,
            Precision::I64(value) => *value as u8,
            Precision::F32(value) => *value as u8,
            Precision::F64(value) => *value as u8,
        }
    }

    pub fn to_i16(&self) -> i16 {
        match self {
            Precision::U8(value) => *value as i16,
            Precision::I16(value) => *value,
            Precision::I32(value) => *value as i16,
            Precision::I64(value) => *value as i16,
            Precision::F32(value) => *value as i16,
            Precision::F64(value) => *value as i16,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Precision::U8(value) => *value as i32,
            Precision::I16(value) => *value as i32,
            Precision::I32(value) => *value,
            Precision::I64(value) => *value as i32,
            Precision::F32(value) => *value as i32,
            Precision::F64(value) => *value as i32,
        }
    }

    pub fn to_i64(&self) -> i64 {
        match self {
            Precision::U8(value) => *value as i64,
            Precision::I16(value) => *value as i64,
            Precision::I32(value) => *value as i64,
            Precision::I64(value) => *value,
            Precision::F32(value) => *value as i64,
            Precision::F64(value) => *value as i64,
        }
    }

    pub fn to_f32(&self) -> f32 {
        match self {
            Precision::U8(value) => *value as f32,
            Precision::I16(value) => *value as f32,
            Precision::I32(value) => *value as f32,
            Precision::I64(value) => *value as f32,
            Precision::F32(value) => *value,
            Precision::F64(value) => *value as f32,
        }
    }

    pub fn to_f64(&self) -> f64 {
        match self {
            Precision::U8(value) => *value as f64,
            Precision::I16(value) => *value as f64,
            Precision::I32(value) => *value as f64,
            Precision::I64(value) => *value as f64,
            Precision::F32(value) => *value as f64,
            Precision::F64(value) => *value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Empty {
    pub fitsblocks: Vec<[u8; 2880]>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Empty(Empty),
    Array(array::ArrayData),
    ASCIITable(tables::ASCIITable),
    BinaryTable(tables::BinaryTable),
}

impl Data {
    pub fn new() -> Data {
        Data::Empty({
            Empty {
                fitsblocks: Vec::new(),
            }
        })
    }

    pub fn append(&mut self, chunk: [u8; 2880]) {
        match self {
            Data::Empty(empty) => empty.fitsblocks.push(chunk),
            Data::Array(array) => array.fitsblocks.push(chunk),
            Data::ASCIITable(ascii_table) => ascii_table.fitsblocks.push(chunk),
            Data::BinaryTable(binary_table) => binary_table.fitsblocks.push(chunk),
        }
    }

    pub fn from_header(fitsblocks: &Vec<[u8; 2880]>, header: &header::Header) -> Data {
        let header_type = header.get_header_type();
        match header_type {
            header::HeaderType::Primary => Data::Array(array::ArrayData::new(fitsblocks, header, Some(0), Some(1))),
            header::HeaderType::Image => Data::Array(array::ArrayData::new(fitsblocks, header, Some(0), Some(1))),
            header::HeaderType::ASCIITable => {
                Data::ASCIITable(tables::ASCIITable::new(fitsblocks, header))
            }
            header::HeaderType::BinaryTable => {
                Data::BinaryTable(tables::BinaryTable::new(fitsblocks, header))
            }
        }
    }

    pub fn get_fitsblocks(&self) -> &Vec<[u8; 2880]> {
        match self {
            Data::Empty(empty) => &empty.fitsblocks,
            Data::Array(array) => &array.fitsblocks,
            Data::ASCIITable(ascii_table) => &ascii_table.fitsblocks,
            Data::BinaryTable(binary_table) => &binary_table.fitsblocks,
        }
    }
}
