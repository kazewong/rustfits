enum Precision {
    U8,
    I16,
    I32,
    SINGLE,
    DOUBLE,
}


struct Data {
    fitsblocks: Vec<[u8; 2880]>,
    bitpix: Precision,
    naxis: u8,
    pcount: u32,
    gcount: u32,
}

impl Data {

}