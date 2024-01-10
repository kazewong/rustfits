use std::str;
use std::collections::HashMap;

enum Keywords{

}

pub struct Header{
    data: Vec<[u8; 2880]>,
}

impl Header{
    
    pub fn new() -> Header{
        Header{data: Vec::new()}
    }

    fn print(&self){
        for i in 0..self.data.len(){
            println!("{}", str::from_utf8(&self.data[i]).unwrap());
        }
    }

    pub fn append(&mut self, chunk: [u8; 2880]){
        self.data.push(chunk);
    }

    pub fn is_empty(&self) -> bool{
        self.data.len() == 0
    }

}

enum Precision{
    U8,
    I16,
    I32,
    SINGLE,
    DOUBLE,
}

pub struct Data{
    data: Vec<[u8; 2880]>,
    bitpix: Precision,
    naxis: u8,
    pcount: u32,
    gcount: u32,

}

impl Data{
    
        pub fn new() -> Data{
            Data{data: Vec::new(), bitpix: Precision::U8, naxis: 0, pcount: 0, gcount: 0}
        }

        pub fn append(&mut self, chunk: [u8; 2880]){
            self.data.push(chunk);
        }
    
    
}

pub struct HDU{
    pub header: Header,
    pub data: Data
}

impl HDU{

    fn print_fitblock(block: [u8; 2880]){
        for i in 0..36{
            println!("{}", str::from_utf8(&block[i*80..(i+1)*80]).unwrap());
        }
    }

}