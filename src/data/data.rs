use std::u8;

use crate::data::{primary, image, tables};
use crate::header;
use byteorder::{BigEndian, ByteOrder};

pub enum Precision{
    U8(Vec<[u8; 2880]>),
    I16(Vec<[i16; 1440]>),
    I32(Vec<[i32; 720]>),
    I64(Vec<[i64; 360]>),
    F32(Vec<[f32; 720]>),
    F64(Vec<[f64; 360]>),
}
impl Precision {

    pub fn convert_fitsblocks(fitsblocks: Vec<[u8; 2880]>, precision: Precision) -> Precision {
        match precision {
            Precision::U8(_) => Precision::U8(fitsblocks),
            Precision::I16(_) => Precision::I16(Precision::convert_fitsblocks_to_i16(fitsblocks)),
            Precision::I32(_) => Precision::I32(Precision::convert_fitsblocks_to_i32(fitsblocks)),
            Precision::I64(_) => Precision::I64(Precision::convert_fitsblocks_to_i64(fitsblocks)),
            Precision::F32(_) => Precision::F32(Precision::convert_fitsblocks_to_f32(fitsblocks)),
            Precision::F64(_) => Precision::F64(Precision::convert_fitsblocks_to_f64(fitsblocks)),
        }
    }

    fn convert_fitsblocks_to_i16(fitsblocks: Vec<[u8; 2880]>) -> Vec<[i16; 1440]>  {
        let mut result: Vec<[i16; 1440]> = Vec::new();
        for i in 0..fitsblocks.len() {
            let mut fitsblock: [i16; 1440] = [0; 1440];
            for j in 0..1440 {
                fitsblock[j] = BigEndian::read_i16(&fitsblocks[i][j*2..(j+1)*2]);
            }
            result.push(fitsblock);
        }
        result
    }

    fn convert_fitsblocks_to_i32(fitsblocks: Vec<[u8; 2880]>) -> Vec<[i32; 720]>  {
        let mut result: Vec<[i32; 720]> = Vec::new();
        for i in 0..fitsblocks.len() {
            let mut fitsblock: [i32; 720] = [0; 720];
            for j in 0..720 {
                fitsblock[j] = BigEndian::read_i32(&fitsblocks[i][j*4..(j+1)*4]);
            }
            result.push(fitsblock);
        }
        result
    }

    fn convert_fitsblocks_to_i64(fitsblocks: Vec<[u8; 2880]>) -> Vec<[i64; 360]>  {
        let mut result: Vec<[i64; 360]> = Vec::new();
        for i in 0..fitsblocks.len() {
            let mut fitsblock: [i64; 360] = [0; 360];
            for j in 0..360 {
                fitsblock[j] = BigEndian::read_i64(&fitsblocks[i][j*8..(j+1)*8]);
            }
            result.push(fitsblock);
        }
        result
    }

    fn convert_fitsblocks_to_f32(fitsblocks: Vec<[u8; 2880]>) -> Vec<[f32; 720]>  {
        let mut result: Vec<[f32; 720]> = Vec::new();
        for i in 0..fitsblocks.len() {
            let mut fitsblock: [f32; 720] = [0.0; 720];
            for j in 0..720 {
                fitsblock[j] = BigEndian::read_f32(&fitsblocks[i][j*4..(j+1)*4]);
            }
            result.push(fitsblock);
        }
        result
    }

    fn convert_fitsblocks_to_f64(fitsblocks: Vec<[u8; 2880]>) -> Vec<[f64; 360]>  {
        let mut result: Vec<[f64; 360]> = Vec::new();
        for i in 0..fitsblocks.len() {
            let mut fitsblock: [f64; 360] = [0.0; 360];
            for j in 0..360 {
                fitsblock[j] = BigEndian::read_f64(&fitsblocks[i][j*8..(j+1)*8]);
            }
            result.push(fitsblock);
        }
        result
    }
}

// pub trait Data{
//     fn get_fitsblocks(&self) -> &Vec<[u8; 2880]>;
//     fn append_fitsblock(&mut self, fitsblock: [u8; 2880]);
//     fn from_header(fitsblocks: &Vec<[u8; 2880]>, header: &header::Header) -> Self;
// }

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

    // pub fn convert_fitsblocks(&self) -> Vec<[i16; 1440]>{
    //     match self {
    //         Data::Primary(_) => Vec::new(),
    //         Data::Image(image) => image.convert_fitsblocks(),
    //         Data::ASCIITable(_) => Vec::new(),
    //         Data::BinaryTable(_) => Vec::new(),
    //     }
    // }

}
