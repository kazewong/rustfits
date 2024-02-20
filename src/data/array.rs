use crate::data::data::Precision;
use crate::header;
use byteorder::{BigEndian, ByteOrder};
use ndarray::Array;

pub enum ArrayType{
    Primary,
    Image,
    Spectrum,
    Cube
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayData {
    pub fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<usize>,
    pcount: usize,
    gcount: usize,
}

impl ArrayData {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &header::Header, pcount: Option<usize>, gcount:Option<usize>) -> ArrayData {
        let bitpix = header.get_keyword("BITPIX").unwrap().parse::<i8>().unwrap();
        let naxis = header.get_keyword("NAXIS").unwrap().parse::<u8>().unwrap();
        let mut naxisn: Vec<usize> = Vec::new();
        for i in 1..=naxis {
            let naxisn_i = header
                .get_keyword(&format!("NAXIS{}", i))
                .unwrap()
                .parse::<usize>()
                .unwrap();
            naxisn.push(naxisn_i);
        }
        let pcount = match pcount {
            Some(pcount) => pcount as usize,
            None => 0,
        };
        let gcount = match gcount {
            Some(gcount) => gcount as usize,
            None => 1,
        };
        ArrayData {
            fitsblocks: fitsblocks.to_vec(),
            bitpix,
            naxis,
            naxisn,
            pcount,
            gcount
        }
    }

    pub fn n_bits(&self) -> usize {
        (self.bitpix.abs() as usize)
            * self.gcount
            * (self.pcount + self.naxisn.iter().product::<usize>())
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
