use crate::data::data::Precision;

#[derive(Debug, Clone, PartialEq)]
pub struct Primary {
    pub fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
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

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)*(self.naxisn.iter().product::<u32>())
    }

    pub fn convert_fitsblocks(&self) -> Precision {
        let mut fitsblocks: Vec<[u8; 2880]> = self.fitsblocks.to_vec();
        let mut precision: Precision = match self.bitpix {
            8 => Precision::U8(Vec::new()),
            16 => Precision::I16(Vec::new()),
            32 => Precision::I32(Vec::new()),
            64 => Precision::I64(Vec::new()),
            -32 => Precision::F32(Vec::new()),
            -64 => Precision::F64(Vec::new()),
            _ => Precision::U8(Vec::new()),
        };
        precision = Precision::convert_fitsblocks(fitsblocks, precision);
        precision
    }
}