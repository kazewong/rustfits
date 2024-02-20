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

#[derive(Debug, Clone, PartialEq)]
pub struct Empty {
    pub fitsblocks: Vec<[u8; 2880]>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Empty(Empty),
    Primary(array::ArrayData),
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
            Data::Primary(primary) => primary.fitsblocks.push(chunk),
            Data::Array(array) => array.fitsblocks.push(chunk),
            Data::ASCIITable(ascii_table) => ascii_table.fitsblocks.push(chunk),
            Data::BinaryTable(binary_table) => binary_table.fitsblocks.push(chunk),
        }
    }

    pub fn from_header(fitsblocks: &Vec<[u8; 2880]>, header: &header::Header) -> Data {
        let header_type = header.get_header_type();
        match header_type {
            header::HeaderType::Primary => Data::Primary(array::ArrayData::new(fitsblocks, header, Some(0), Some(1))),
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
            Data::Primary(primary) => &primary.fitsblocks,
            Data::Array(array) => &array.fitsblocks,
            Data::ASCIITable(ascii_table) => &ascii_table.fitsblocks,
            Data::BinaryTable(binary_table) => &binary_table.fitsblocks,
        }
    }
}
