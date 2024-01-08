use std::str;

pub fn bytes_to_hdu(buffer: &Vec<u8>) -> Vec<String> {
    let n_chunks = buffer.len() / 2880;
    let mut hdu = Vec::new();
    for i in 0..n_chunks {
        let start = i * 2880;
        let end = start + 2880;
        let chunk = buffer[start..end].to_vec();
        let chunk = str::from_utf8(&chunk).unwrap().to_string();
        hdu.push(chunk);
    }
    hdu
}