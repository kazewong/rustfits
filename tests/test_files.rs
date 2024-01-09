use rustfits::parser;
use std::io;
use std::io::prelude::*;
use std::fs::File;

#[test]
fn test_wfpc2() -> io::Result<()> {
    let mut f = File::open("tests/data/WFPC2u5780205r_c0fx.fits")?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    parser::bytes_to_hdu(&buffer);
    Ok(())

}
