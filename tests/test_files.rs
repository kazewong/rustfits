use rustfits::data::data::Data::{ASCIITable, BinaryTable, Primary, Array};
use rustfits::fits::{FITS, HDU};
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
        println!(
            "Keyword: NAXIS Value: {}",
            fits.hdus[i].header.get_keyword("NAXIS").unwrap()
        );
        // println!("Data: {:?}\n", fits.hdus[i].data);
    }
    match &fits.hdus[0].data{
        Primary(data) => {
            data.format_data();
        }
        _ => {
            println!("Not a Primary");
        }
    }
    match &fits.hdus[1].data {
        ASCIITable(table) => {
            table.format_data();
        }
        _ => {
            println!("Not an ASCIITable");
        }
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
        // println!("Data: {:?}\n", fits.hdus[i].data);
    }
    match &fits.hdus[3].data {
        BinaryTable(table) => {
            table.format_data();
        }
        Array(data) => {
            println!("Array: {:?}", data.format_data().into_dimensionality::<ndarray::Ix2>().unwrap()[[3, 0]].to_f64());
        }
        _ => {
            println!("Not an BinaryTable");
        }
    }
    Ok(())
}
