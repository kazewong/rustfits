use crate::header;

use header::{Header, HeaderType};

pub struct Primary {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
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
}

pub struct Image {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
}

impl Image {
    pub fn new(fitsblocks: Vec<[u8; 2880]>, header: &Header) -> Image {
        let bitpix = header.get_keyword("BITPIX").unwrap().parse::<u8>().unwrap();
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
            fitsblocks,
            bitpix: bitpix,
            naxis: naxis,
            naxisn: naxisn,
            pcount: 0,
            gcount: 1,
        }
    }
}

pub struct ASCIITable {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
    tbcoln: Vec<u32>,
}

impl ASCIITable {
    pub fn new() -> ASCIITable {
        ASCIITable {
            fitsblocks: Vec::new(),
            bitpix: 8,
            naxis: 2,
            naxisn: Vec::new(),
            pcount: 0,
            gcount: 1,
            tfields: 0,
            tformn: Vec::new(),
            tbcoln: Vec::new(),
        }
    }
}

pub struct BinaryTable {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
}

impl BinaryTable {
    pub fn new() -> BinaryTable {
        BinaryTable {
            fitsblocks: Vec::new(),
            bitpix: 8,
            naxis: 2,
            naxisn: Vec::new(),
            pcount: 0,
            gcount: 1,
            tfields: 0,
            tformn: Vec::new(),
        }
    }
}

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

    pub fn from_header(fitsblocks: Vec<[u8; 2880]>, header: &Header) -> Data {
        let header_type = header.get_header_type().unwrap();
        match header_type {
            header::HeaderType::Primary => Data::Primary(Primary::new()),
            header::HeaderType::Image => Data::Image(Image::new(fitsblocks, header)),
            header::HeaderType::ASCIITable => Data::ASCIITable(ASCIITable::new()),
            header::HeaderType::BinaryTable => Data::BinaryTable(BinaryTable::new()),
        }
    }
}
