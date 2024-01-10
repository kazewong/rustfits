use crate::header::Header;

enum Precision {
    U8,
    I16,
    I32,
    SINGLE,
    DOUBLE,
}

pub struct Data {
    data: Vec<[u8; 2880]>,
    bitpix: Precision,
    naxis: u8,
    pcount: u32,
    gcount: u32,
}

impl Data {
    pub fn new() -> Data {
        Data {
            data: Vec::new(),
            bitpix: Precision::U8,
            naxis: 0,
            pcount: 0,
            gcount: 0,
        }
    }

    pub fn append(&mut self, chunk: [u8; 2880]) {
        self.data.push(chunk);
    }
}

pub struct HDU {
    pub header: Header,
    pub data: Data,
}

impl HDU {}
