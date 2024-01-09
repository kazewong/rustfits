use std::str;
pub struct HDU{
    pub header: Vec<[u8; 2880]>,
    pub data: Vec<[u8; 2880]>
}

impl HDU{

    fn print_fitblock(block: [u8; 2880]){
        for i in 0..36{
            println!("{}", str::from_utf8(&block[i*80..(i+1)*80]).unwrap());
        }
    }

    pub fn print_header(&self){
        for i in 0..self.header.len(){
            HDU::print_fitblock(self.header[i]);
        }
    }
}