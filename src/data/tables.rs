use crate::header;

use header::Header;

pub struct Matrix<T> {
    data: Vec<T>,
    n_row: u32,
    n_column: u32,
}

impl<T: std::clone::Clone> Matrix<T> {
    pub fn new(data: Vec<T>, n_row: u32, n_column: u32) -> Matrix<T> {
        Matrix { data, n_row, n_column }
    }

    pub fn get_row(&self, row: u32) -> Vec<T> {
        let row_length = self.data.len() as u32 / self.n_row;
        let row_start = row * row_length;
        let row_end = (row + 1) * row_length;
        self.data[row_start as usize..row_end as usize].to_vec()
    }

    pub fn get_column(&self, column: u32) -> Vec<T> {
        let row_length = self.data.len() as u32 / self.n_row;
        let mut column_data: Vec<T> = Vec::new();
        for i in 0..self.n_row {
            let row_start = i * row_length;
            let row_end = (i + 1) * row_length;
            column_data.push(self.data[row_start as usize + column as usize].clone());
        }
        column_data
    }

    pub fn append_row(&mut self, row: Vec<T>) {
        self.data.extend(row);
        self.n_row += 1;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASCIIField {
    Character(String),
    Integer(i32),
    FloatDecimal(f32),
    FloatExponential(f32),
    DoubleExponential(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ASCIITable {
    pub fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
    tbcoln: Vec<u32>,
}

impl ASCIITable {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &Header) -> ASCIITable {
        let bitpix = header.get_keyword("BITPIX").unwrap().parse::<i8>().unwrap();
        let naxis = header.get_keyword("NAXIS").unwrap().parse::<u8>().unwrap();
        let mut naxisn: Vec<u32> = Vec::new();
        for i in 1..=naxis {
            let naxisn_i = header
                .get_keyword(&format!("NAXIS{}", i))
                .unwrap()
                .parse::<u32>()
                .unwrap();
            naxisn.push(naxisn_i);
        }
        let tfields = header
            .get_keyword("TFIELDS")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut tformn: Vec<String> = Vec::new();
        for i in 1..=tfields {
            let tformn_i = header.get_keyword(&format!("TFORM{}", i)).unwrap();
            tformn.push(tformn_i);
        }
        let mut tbcoln: Vec<u32> = Vec::new();
        for i in 1..=tfields {
            let tbcoln_i = header
                .get_keyword(&format!("TBCOL{}", i))
                .unwrap()
                .parse::<u32>()
                .unwrap();
            tbcoln.push(tbcoln_i);
        }
        ASCIITable {
            fitsblocks: fitsblocks.to_vec(),
            bitpix,
            naxis,
            naxisn,
            pcount: 0,
            gcount: 1,
            tfields,
            tformn,
            tbcoln,
        }
    }

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)
            * self.gcount
            * (self.pcount + self.naxisn.iter().product::<u32>())
    }

    fn parse_row(&self, data: &[u8]) -> Vec<ASCIIField>{
        let mut result: Vec<ASCIIField> = Vec::new();
        result
    }

    pub fn format_data(&self) -> Matrix<ASCIIField> {
        let fitsblocks_flat: Vec<u8> = self.fitsblocks.iter().flatten().cloned().collect();
        let row_length: u32 = self.naxisn[0];
        let n_row: u32 = self.naxisn[1];
        let n_field: u32 = self.tfields;
        let mut result: Matrix<ASCIIField> = Matrix::new(Vec::new(), n_row, n_field);
        for i in 0..n_row {
            result.append_row(self.parse_row(&fitsblocks_flat
                [i as usize * row_length as usize..(i + 1) as usize * row_length as usize]));
        }
        result
    }
}

enum BinaryField {
    Logical(bool),
    Bit(u8),
    Byte(u8),
    I16(i16),
    I32(i32),
    I64(i64),
    Character(u8),
    F32(f32),
    F64(f64),
    Complex32(f32, f32),
    Complex64(f64, f64),
    Array32(f32, f32),
    Array64(f64, f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryTable {
    pub fitsblocks: Vec<[u8; 2880]>,
    bitpix: i8,
    naxis: u8,
    naxisn: Vec<u32>,
    pcount: u32,
    gcount: u32,
    tfields: u32,
    tformn: Vec<String>,
}

impl BinaryTable {
    pub fn new(fitsblocks: &Vec<[u8; 2880]>, header: &Header) -> BinaryTable {
        let bitpix = header.get_keyword("BITPIX").unwrap().parse::<i8>().unwrap();
        let naxis = header.get_keyword("NAXIS").unwrap().parse::<u8>().unwrap();
        let mut naxisn: Vec<u32> = Vec::new();
        for i in 1..=naxis {
            let naxisn_i = header
                .get_keyword(&format!("NAXIS{}", i))
                .unwrap()
                .parse::<u32>()
                .unwrap();
            naxisn.push(naxisn_i);
        }
        let tfields = header
            .get_keyword("TFIELDS")
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let mut tformn: Vec<String> = Vec::new();
        for i in 1..=tfields {
            let tformn_i = header.get_keyword(&format!("TFORM{}", i)).unwrap();
            tformn.push(tformn_i);
        }
        BinaryTable {
            fitsblocks: fitsblocks.to_vec(),
            bitpix,
            naxis,
            naxisn,
            pcount: 0,
            gcount: 1,
            tfields,
            tformn,
        }
    }

    pub fn n_bits(&self) -> u32 {
        (self.bitpix.abs() as u32)
            * self.gcount
            * (self.pcount + self.naxisn.iter().product::<u32>())
    }
}
