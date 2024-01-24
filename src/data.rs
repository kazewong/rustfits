use crate::header;
use header::Header;

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
}
