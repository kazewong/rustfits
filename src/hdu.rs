use crate::header::Header;
use crate::data::Data;
enum Precision {
    U8,
    I16,
    I32,
    SINGLE,
    DOUBLE,
}


pub struct HDU {
    pub header: Header,
    pub data: Data,
}

impl HDU {}
