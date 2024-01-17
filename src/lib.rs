mod hdu;
mod header;
mod data;
mod utils;
pub mod parser;


struct Fits{
    hdus: Vec<hdu::HDU>,
}