
use crate::header;
use crate::data;

use header::Header;
use data::Data;
use std::convert::TryInto;

pub struct HDU {
    pub header: Header,
    pub data: Data,
}

pub struct FITS {
    pub hdus: Vec<HDU>,
}

impl FITS{

    pub fn new_from_buffer(buffer: &Vec<u8>) -> FITS {
        let mut hdus = FITS::bytes_to_hdu(&buffer);
        for i in 0..hdus.len() {
            hdus[i].header.initialize_header();
            hdus[i].data = Data::from_header(hdus[i].data.get_fitsblocks(), &hdus[i].header)
        }
        FITS { hdus: hdus }
    }

    fn check_header_beginning(chunk: [u8; 2880]) -> bool {
        let mut result = false;
        // The padding for 32 after SIMPLE is to reduce the number of false positives
        if chunk[0..8] == [83, 73, 77, 80, 76, 69, 32, 32]
            || chunk[0..8] == [88, 84, 69, 78, 83, 73, 79, 78]
        {
            result = true;
        }
        result
    }
    
    fn check_end(chunks: [u8; 2880]) -> bool {
        let mut end = false;
        for i in 0..36 {
            if chunks[i * 80..(i + 1) * 80][0..8] == [69, 78, 68, 32, 32, 32, 32, 32] {
                end = true;
                break;
            }
        }
        end
    }
    
    pub fn bytes_to_hdu(buffer: &Vec<u8>) -> Vec<HDU> {
        let n_chunks = buffer.len() / 2880;
        let mut read_header = false;
        let mut hdus: Vec<HDU> = Vec::new();
        let mut current_hdu: HDU = HDU {
            header: Header::new(),
            data: Data::new(),
        };
        for i in 0..n_chunks {
            let start = i * 2880;
            let end = start + 2880;
            let chunk: [u8; 2880] = buffer[start..end]
                .try_into()
                .expect("slice with incorrect length");
            if FITS::check_header_beginning(chunk) == true {
                read_header = true;
                if !current_hdu.header.is_empty() {
                    hdus.push(current_hdu);
                    current_hdu = HDU {
                        header: Header::new(),
                        data: Data::new(),
                    };
                }
            }
            if read_header == true {
                current_hdu.header.append(chunk);
                if FITS::check_end(chunk) {
                    read_header = false;
                }
            } else {
                current_hdu.data.append(chunk);
            }
            if i == n_chunks - 1 {
                hdus.push(current_hdu);
                break;
            }
        }
        hdus
    }

    pub fn list_headers(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for hdu in &self.hdus {
            result.push(hdu.header.get_header_type().to_string());
        }
        result
    }
    
}

