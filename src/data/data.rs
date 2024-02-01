use std::u8;

use crate::data::{primary, image, tables};
use crate::header;

pub enum Precision{
    U8(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Primary(primary::Primary),
    Image(image::Image),
    ASCIITable(tables::ASCIITable),
    BinaryTable(tables::BinaryTable),
}

impl Data {
    pub fn new() -> Data {
        Data::Primary(primary::Primary::new())
    }

    pub fn append(&mut self, chunk: [u8; 2880]) {
        match self {
            Data::Primary(primary) => primary.fitsblocks.push(chunk),
            Data::Image(image) => image.fitsblocks.push(chunk),
            Data::ASCIITable(ascii_table) => ascii_table.fitsblocks.push(chunk),
            Data::BinaryTable(binary_table) => binary_table.fitsblocks.push(chunk),
        }
    }

    pub fn from_header(fitsblocks: &Vec<[u8; 2880]>, header: &header::Header) -> Data {
        let header_type = header.get_header_type();
        match header_type {
            header::HeaderType::Primary => Data::Primary(primary::Primary::new()),
            header::HeaderType::Image => Data::Image(image::Image::new(fitsblocks, header)),
            header::HeaderType::ASCIITable => Data::ASCIITable(tables::ASCIITable::new(fitsblocks, header)),
            header::HeaderType::BinaryTable => {
                Data::BinaryTable(tables::BinaryTable::new(fitsblocks, header))
            }
        }
    }

    pub fn get_fitsblocks(&self) -> &Vec<[u8; 2880]> {
        match self {
            Data::Primary(primary) => &primary.fitsblocks,
            Data::Image(image) => &image.fitsblocks,
            Data::ASCIITable(ascii_table) => &ascii_table.fitsblocks,
            Data::BinaryTable(binary_table) => &binary_table.fitsblocks,
        }
    }
    
}
