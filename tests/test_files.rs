use rustfits::parser;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[test]
fn test_wfpc2() -> io::Result<()> {
    let mut f = File::open("tests/data/WFPC2u5780205r_c0fx.fits")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let mut hdus = parser::bytes_to_hdu(&buffer);
    for i in 0..hdus.len(){
        hdus[i].header.initialize_header();
    }
    for i in 0..hdus.len(){
        println!("Keyword: NAXIS Value: {}\n", hdus[i].header.get_keyword("NAXIS").unwrap());
    }
    Ok(())
}

#[test]
fn test_euv() -> io::Result<()> {
    let mut f = File::open("tests/data/EUVEngc4151imgx.fits")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let hdus = parser::bytes_to_hdu(&buffer);
    for i in 0..hdus.len(){
        hdus[i].header.print();
    }
    Ok(())
}
