use std::{str, convert::TryInto, fs::read, env::current_exe};

pub struct HDU{
    header: Vec<[u8; 2880]>,
    data: Vec<[u8; 2880]>
}

impl HDU{
    
}

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

pub fn bytes_to_hdu(buffer: &Vec<u8>) -> Vec<HDU>{
    let n_chunks = buffer.len() / 2880;
    let mut read_header = false;
    let mut hdus: Vec<HDU> = Vec::new();
    let mut current_hdu: HDU = HDU{header: Vec::new(), data: Vec::new()};
    for i in 0..n_chunks {
        let start = i * 2880;
        let end = start + 2880;
        let chunk: [u8; 2880] = buffer[start..end].try_into().expect("slice with incorrect length");
        if check_header_beginning(chunk) == true{
            read_header = true;
            if current_hdu.header.len() > 0{
                hdus.push(current_hdu);
                current_hdu = HDU{header: Vec::new(), data: Vec::new()};
            }
        }
        if read_header == true{
            current_hdu.header.push(chunk);
            if !check_end(chunk){
                read_header = false;
            }
        }else{
            current_hdu.data.push(chunk);
        }
        if i == n_chunks - 1{
            hdus.push(current_hdu);
            break;
        }
    }
    hdus
}