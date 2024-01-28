use crate::data::data::Precision;
use crate::header;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
}

impl Image {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &header::Header) -> Image {
        let bitpix = header.get_keyword("BITPIX").unwrap().parse::<i8>().unwrap();
        let naxis = header.get_keyword("NAXIS").unwrap().parse::<u8>().unwrap();
        let mut naxisn: Vec<u32> = Vec::new();
        for i in 1..=naxis {
            let naxisn_i = header
                .get_keyword(&format!("NAXIS{}", i))
                .unwrap()
                .parse::<u32>()
                .unwrap();
            naxisn.push(naxisn_i);
        }
        Image {
            fitsblocks: fitsblocks.to_vec(),
            bitpix,
            naxis,
            naxisn,
            pcount: 0,
            gcount: 1,
        }
    }

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)*self.gcount*(self.pcount+self.naxisn.iter().product::<u32>())
    }

    // pub fn to_image
}
