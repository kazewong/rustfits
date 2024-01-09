use std::{str, array, convert::TryInto};

pub struct FITSBlock{
    header: bool,
    data: [u8; 2880],
}

impl FITSBlock{
    fn new(header: bool, buffer: [u8; 2880]) -> FITSBlock{
        FITSBlock{
            header: header,
            data: buffer,
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

fn check_header_beginning(chunk: [u8; 2880]) -> bool{
    let mut result = false;
    if chunk[0..6] == [83, 73, 77, 80, 76, 69] || chunk[0..8] == [88, 84, 69, 78, 83, 73, 79, 78]{
        result = true;
    }
    result
}

fn check_end(chunks: [u8; 2880]) -> bool{
    let mut end = false;
    for i in 0..36{
        if chunks[i*80..(i+1)*80][0..3] == [69, 78, 68]{
            end = true;
            break;
        }
    }
    end
}

pub fn bytes_to_fitsblocks(buffer: &Vec<u8>) {//-> Vec<FITSBlock> {
    let n_chunks = buffer.len() / 2880;
    for i in 0..n_chunks {
        let start = i * 2880;
        let end = start + 2880;

        let chunk: [u8; 2880] = buffer[start..end].try_into().expect("slice with incorrect length");
        println!("Is header beginning: {:?}", check_header_beginning(chunk));
        println!("Is header end: {:?}", check_end(chunk));
        // let chunk = str::from_utf8(&chunk).unwrap().to_string();
        
    }
    // header
}