use rustfits::fits::{HDU, FITS};
use std::fs::File;
use std::io;
use std::io::prelude::*;

#[test]
fn test_wfpc2() -> io::Result<()> {
    let mut f = File::open("tests/data/WFPC2u5780205r_c0fx.fits")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let fits = FITS::new_from_buffer(&buffer);
    for i in 0..fits.hdus.len() {
        println!("HDU type: {:?}", fits.hdus[i].header.get_header_type());
        // hdus[i].header.list_keywords();
        println!(
            "Keyword: NAXIS Value: {}",
            fits.hdus[i].header.get_keyword("NAXIS").unwrap()
        );
        println!("Data: {:?}\n",fits.hdus[i].data);
    }
    Ok(())
}

#[test]
fn test_euv() -> io::Result<()> {
    let mut f = File::open("tests/data/EUVEngc4151imgx.fits")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let fits = FITS::new_from_buffer(&buffer);
    for i in 0..fits.hdus.len() {
        println!("HDU type: {:?}", fits.hdus[i].header.get_header_type());
        // hdus[i].header.list_keywords();
        println!(
            "Keyword: NAXIS Value: {}",
            fits.hdus[i].header.get_keyword("NAXIS").unwrap()
        );
        println!("Data: {:?}\n",fits.hdus[i].data);
    }
    Ok(())
}
