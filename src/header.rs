use std::collections::HashMap;
use std::str;

pub struct Header {
    data: Vec<[u8; 2880]>,
    header_type: HeaderType,
    initiailzed: bool,
    keywords: HashMap<String, [String; 2]>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            data: Vec::new(),
            header_type: HeaderType::Primary,
            initiailzed: false,
            keywords: HashMap::new(),
        }
    }

    pub fn initialize_header(&mut self) {
        self.initiailzed = true;
        for i in 0..self.data.len() {
            let chunk = &self.data[i];
            for j in 0..36 {
                let (keyword, value) = Header::parse_line(&chunk[j * 80..(j + 1) * 80]);
                self.keywords.insert(keyword, value);
            }
        }
    }

    pub fn append(&mut self, chunk: [u8; 2880]) {
        self.data.push(chunk);
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    fn parse_line(buffer: &[u8]) -> (String, [String; 2]) {
        let mut keyword = String::from(str::from_utf8(&buffer[0..8]).unwrap());
        keyword.retain(|c| !c.is_whitespace());
        if buffer[8..10] == [61, 32] {
            let value = String::from(str::from_utf8(&buffer[11..80]).unwrap());
            let parts = value.split("/").collect::<Vec<&str>>();
            if parts.len() == 2 {
                let comment = String::from(parts[1]);
                let mut value = String::from(parts[0]);
                value.retain(|c| !c.is_whitespace());
                (keyword, [value, comment])
            } else {
                let mut value = String::from(parts[0]);
                value.retain(|c| !c.is_whitespace());
                (keyword, [value, String::from("")])
            }
        } else {
            let value = String::from(str::from_utf8(&buffer[9..80]).unwrap());
            (keyword, [String::from(""), value])
        }
    }

    pub fn print(&self) {
        for i in 0..self.data.len() {
            println!("{}", str::from_utf8(&self.data[i]).unwrap());
        }
    }

    pub fn list_keywords(&self) {
        for (key, value) in &self.keywords {
            println!("{}: {}", key, value[0]);
        }
    }

    pub fn get_keyword(&self, keyword: &str) -> Option<String> {
        match self.keywords.get(keyword) {
            Some(value) => Some(value[0].to_string()),
            None => Some("Field not found".to_string()),
        }
    }
}

enum HeaderType {
    Primary,
    Conforming,
    Image,
    ASCIITable,
    BinaryTable,
    CompressImage,
    CompressTable,
    RandomGroup,
}

// Mandatory keywords for all structures
enum PrimaryKeyword {
    SIMPLE,
    BITPIX,
    NAXIS,
}

enum ConformingKeyword {
    XTENSION,
    BITPIX,
    NAXIS,
    PCOUNT,
    GCOUNT,
}

enum ImageExtensionKeyword {
    XTENSION,
    BITPIX,
    NAXIS,
    PCOUNT,
    GCOUNT,
}

enum ASCIITableKeyword {
    XTENSION,
    BITPIX,
    NAXIS,
    NAXIS1,
    NAXIS2,
    PCOUNT,
    GCOUNT,
    TFIELDS,
    TFORM,
    TBCOL,
}

enum BinaryTableKeyword {
    XTENSION,
    BITPIX,
    NAXIS,
    NAXIS1,
    NAXIS2,
    PCOUNT,
    GCOUNT,
    TFIELDS,
    TFORM,
}
