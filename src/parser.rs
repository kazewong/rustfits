use std::{str, array, convert::TryInto};

pub struct FITSBlock{
    header: bool,
    data: [u8; 2880],
}

impl FITSBlock{
    fn new() -> FITSBlock{
        FITSBlock{
            header: false,
            data: [0; 2880],
        }
    }

    // fn header_block(&self) -> [[u8; 80]; 36]{
    //     self.header = true;
    //     self.data = block;
    // }
}

// enum start{
//     Header="SIMPLE",
//     Extension="XTENSION",
// }

fn check_end(){

}


pub fn bytes_to_fitsblocks(buffer: &Vec<u8>) -> Vec<FITSBlock> {
    let n_chunks = buffer.len() / 2880;
    let mut header = Vec::new();
    let mut data: Vec<u8> = Vec::new();
    for i in 0..n_chunks {
        let start = i * 2880;
        let end = start + 2880;
        let chunk: [u8; 2880] = buffer[start..end].try_into().expect("slice with incorrect length");
        let chunk = str::from_utf8(&chunk).unwrap().to_string();
        
    }
    header
}