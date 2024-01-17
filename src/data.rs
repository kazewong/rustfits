enum Precision {
    U8,
    I16,
    I32,
    SINGLE,
    DOUBLE,
}

pub struct Primary{
    fitsblocks: Vec<[u8; 2880]>,

}
pub struct Image{
    fitsblocks: Vec<[u8; 2880]>,
}

pub struct ASCIITable{
    fitsblocks: Vec<[u8; 2880]>,
}

pub struct BinaryTable{
    fitsblocks: Vec<[u8; 2880]>,
}

pub enum Data{
    Primary(Primary),
    Image(Image),
    ASCIITable(ASCIITable),
    BinaryTable(BinaryTable),
}

impl Data{
    pub fn new() -> Data{
        Data::Primary(Primary{fitsblocks: Vec::new()})
    }

    pub fn append(&mut self, chunk: [u8; 2880]){
        match self{
            Data::Primary(primary) => primary.fitsblocks.push(chunk),
            Data::Image(image) => image.fitsblocks.push(chunk),
            Data::ASCIITable(ascii_table) => ascii_table.fitsblocks.push(chunk),
            Data::BinaryTable(binary_table) => binary_table.fitsblocks.push(chunk),
        }
    }
}