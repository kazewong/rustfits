use std::str;

struct Header{
    data: Vec<[u8; 2880]>,

}

impl Header{
    fn print(&self){
        for i in 0..self.data.len(){
            println!("{}", str::from_utf8(&self.data[i]).unwrap());
        }
    }

}

enum Precision{
    U8,
    I16,
    I32,
    SINGLE,
    DOUBLE,
}

struct Data{
    data: Vec<[u8; 2880]>,
    bitpix: Precision,
    naxis: u8,
    pcount: u32,
    gcount: u32,

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