pub struct Primary{
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
    naxis: u8,
    naxisn: Vec<u32>,
}

impl Primary{
    pub fn new() -> Primary{
        Primary{
            fitsblocks: Vec::new(),
            bitpix: 0,
            naxis: 0,
            naxisn: Vec::new(),
        }
    }
}

pub struct Image{
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
}

impl Image{
    pub fn new() -> Image{
        Image{
            fitsblocks: Vec::new(),
            bitpix: 0,
            naxis: 0,
            naxisn: Vec::new(),
            pcount: 0,
            gcount: 1,
        }
    }
}

pub struct ASCIITable{
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

impl ASCIITable{
    pub fn new() -> ASCIITable{
        ASCIITable{
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

pub struct BinaryTable{
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: u8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
}

impl BinaryTable{
    pub fn new() -> BinaryTable{
        BinaryTable{
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


pub enum Data{
    Primary(Primary),
    Image(Image),
    ASCIITable(ASCIITable),
    BinaryTable(BinaryTable),
}

impl Data{
    pub fn new(&self) -> Data{
        match self{
            Data::Primary(primary) => Data::Primary(Primary::new()),
            Data::Image(image) => Data::Image(Image::new()),
            Data::ASCIITable(ascii_table) => Data::ASCIITable(ASCIITable::new()),
            Data::BinaryTable(binary_table) => Data::BinaryTable(BinaryTable::new()),
        }
    }

    pub fn append(&mut self, chunk: [u8; 2880]){
        match self{
            Data::Primary(primary) => primary.fitsblocks.push(chunk),
            Data::Image(image) => image.fitsblocks.push(chunk),
            Data::ASCIITable(ascii_table) => ascii_table.fitsblocks.push(chunk),
            Data::BinaryTable(binary_table) => binary_table.fitsblocks.push(chunk),
        }
    }
}