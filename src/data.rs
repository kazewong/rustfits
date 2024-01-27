use std::u8;

use crate::{fits, header};
use header::Header;
use byteorder::{BigEndian, ByteOrder};

enum Precision{
    U8(Vec<[u8; 2880]>),
    I16(Vec<[i16; 1440]>),
    I32(Vec<[i32; 720]>),
    I64(Vec<[i64; 360]>),
    F32(Vec<[f32; 720]>),
    F64(Vec<[f64; 360]>),
}
impl Precision {

    fn convert_fitsblocks(fitsblocks: Vec<[u8; 2880]>, precision: Precision) -> Precision {
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

#[derive(Debug)]
pub struct Primary {
    fitsblocks: Vec<[u8; 2880]>,
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

#[derive(Debug)]
pub struct Image {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
}

impl Image {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &Header) -> Image {
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
            bitpix: bitpix,
            naxis: naxis,
            naxisn: naxisn,
            pcount: 0,
            gcount: 1,
        }
    }

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)*self.gcount*(self.pcount+self.naxisn.iter().product::<u32>())
    }

    // pub fn to_image
}

#[derive(Debug)]
pub struct ASCIITable {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
    tbcoln: Vec<u32>,
}

impl ASCIITable {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &Header) -> ASCIITable {
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
        let tfields = header
            .get_keyword("TFIELDS")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut tformn: Vec<String> = Vec::new();
        for i in 1..=tfields {
            let tformn_i = header.get_keyword(&format!("TFORM{}", i)).unwrap();
            tformn.push(tformn_i);
        }
        let mut tbcoln: Vec<u32> = Vec::new();
        for i in 1..=tfields {
            let tbcoln_i = header
                .get_keyword(&format!("TBCOL{}", i))
                .unwrap()
                .parse::<u32>()
                .unwrap();
            tbcoln.push(tbcoln_i);
        }
        ASCIITable {
            fitsblocks: fitsblocks.to_vec(),
            bitpix: bitpix,
            naxis: naxis,
            naxisn: naxisn,
            pcount: 0,
            gcount: 1,
            tfields: tfields,
            tformn: tformn,
            tbcoln: tbcoln,
        }
    }

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)*self.gcount*(self.pcount+self.naxisn.iter().product::<u32>())
    }
}

#[derive(Debug)]
pub struct BinaryTable {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
}

impl BinaryTable {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &Header) -> BinaryTable {
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
        let tfields = header
            .get_keyword("TFIELDS")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut tformn: Vec<String> = Vec::new();
        for i in 1..=tfields {
            let tformn_i = header.get_keyword(&format!("TFORM{}", i)).unwrap();
            tformn.push(tformn_i);
        }
        BinaryTable {
            fitsblocks: fitsblocks.to_vec(),
            bitpix: bitpix,
            naxis: naxis,
            naxisn: naxisn,
            pcount: 0,
            gcount: 1,
            tfields: tfields,
            tformn: tformn,
        }
    }

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)*self.gcount*(self.pcount+self.naxisn.iter().product::<u32>())
    }
}

#[derive(Debug)]
pub enum Data {
    Primary(Primary),
    Image(Image),
    ASCIITable(ASCIITable),
    BinaryTable(BinaryTable),
}

impl Data {
    pub fn new() -> Data {
        Data::Primary(Primary::new())
    }

    pub fn append(&mut self, chunk: [u8; 2880]) {
        match self {
            Data::Primary(primary) => primary.fitsblocks.push(chunk),
            Data::Image(image) => image.fitsblocks.push(chunk),
            Data::ASCIITable(ascii_table) => ascii_table.fitsblocks.push(chunk),
            Data::BinaryTable(binary_table) => binary_table.fitsblocks.push(chunk),
        }
    }

    pub fn from_header(fitsblocks: &Vec<[u8; 2880]>, header: &Header) -> Data {
        let header_type = header.get_header_type();
        match header_type {
            header::HeaderType::Primary => Data::Primary(Primary::new()),
            header::HeaderType::Image => Data::Image(Image::new(fitsblocks, header)),
            header::HeaderType::ASCIITable => Data::ASCIITable(ASCIITable::new(fitsblocks, header)),
            header::HeaderType::BinaryTable => {
                Data::BinaryTable(BinaryTable::new(fitsblocks, header))
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
