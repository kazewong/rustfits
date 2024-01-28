use crate::header;

use header::Header;



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
        (self.bitpix.abs() as u32)*self.gcount*(self.pcount+self.naxisn.iter().product::<u32>())
    }

    pub fn format_data(&self) -> Vec<Vec<u8>>{
        let fitsblocks_flat: Vec<u8> = self.fitsblocks.iter().flatten().cloned().collect();
        let mut data: Vec<Vec<u8>> = Vec::new();
        let row_length: u32 = self.naxisn[0];
        let n_row: u32 = self.naxisn[1];
        let n_field: u32 = self.tfields;
        for i in 0..n_row {
            let row: Vec<u8> = fitsblocks_flat[i as usize*row_length as usize..(i+1) as usize*row_length as usize].to_vec();
            data.push(row);
        }
        data
    }
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
        (self.bitpix.abs() as u32)*self.gcount*(self.pcount+self.naxisn.iter().product::<u32>())
    }
}
