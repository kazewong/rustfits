use crate::data::data::Precision;
use byteorder::{BigEndian, ByteOrder};
use ndarray::Array;

#[derive(Debug, Clone, PartialEq)]
pub struct Primary {
    pub fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<usize>,
}

impl Primary {
    pub fn new() -> Primary {
        Primary {
            fitsblocks: Vec::new(),
            bitpix: 0,
            naxis: 0,
            naxisn: Vec::new(),
        }
    }

    pub fn n_bits(&self) -> usize {
        (self.bitpix.abs() as usize) * (self.naxisn.iter().product::<usize>())
    }

    pub fn format_data(&self) -> Array<Precision, ndarray::IxDyn> {
        let fitsblocks_flat: Vec<u8> = self.fitsblocks.iter().flatten().cloned().collect();
        let mut local_vec: Vec<Precision> = Vec::new();
        match self.bitpix {
            8 => {
                for i in 0..fitsblocks_flat.len() {
                    local_vec.push(Precision::U8(fitsblocks_flat[i]));
                }
            }
            16 => {
                for i in 0..fitsblocks_flat.len() / 2 {
                    local_vec.push(Precision::I16(BigEndian::read_i16(
                        &fitsblocks_flat[i * 2..(i + 1) * 2],
                    )));
                }
            }
            32 => {
                for i in 0..fitsblocks_flat.len() / 4 {
                    local_vec.push(Precision::I32(BigEndian::read_i32(
                        &fitsblocks_flat[i * 4..(i + 1) * 4],
                    )));
                }
            }
            64 => {
                for i in 0..fitsblocks_flat.len() / 8 {
                    local_vec.push(Precision::I64(BigEndian::read_i64(
                        &fitsblocks_flat[i * 8..(i + 1) * 8],
                    )));
                }
            }
            -32 => {
                for i in 0..fitsblocks_flat.len() / 4 {
                    local_vec.push(Precision::F32(BigEndian::read_f32(
                        &fitsblocks_flat[i * 4..(i + 1) * 4],
                    )));
                }
            }
            -64 => {
                for i in 0..fitsblocks_flat.len() / 8 {
                    local_vec.push(Precision::F64(BigEndian::read_f64(
                        &fitsblocks_flat[i * 8..(i + 1) * 8],
                    )));
                }
            }
            _ => {
                panic!("Unsupported bitpix value: {}", self.bitpix);
            }
        }
        Array::from_shape_vec(self.naxisn.clone(), local_vec).unwrap()
    }
}
