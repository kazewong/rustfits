use std::collections::HashMap;
use std::str;
use std::fmt;

#[derive(Debug)]
pub struct Header {
    fitsblocks: Vec<[u8; 2880]>,
    header_type: HeaderType,
    initiailzed: bool,
    keywords: HashMap<String, [String; 2]>,
}

impl Header {
    pub fn new() -> Header {
        Header {
            fitsblocks: Vec::new(),
            header_type: HeaderType::Primary,
            initiailzed: false,
            keywords: HashMap::new(),
        }
    }

    pub fn initialize_header(&mut self) {
        self.initiailzed = true;
        for i in 0..self.fitsblocks.len() {
            let chunk = &self.fitsblocks[i];
            for j in 0..36 {
                let (keyword, value) = Header::parse_line(&chunk[j * 80..(j + 1) * 80]);
                self.keywords.insert(keyword, value);
            }
        }
        self.header_type = self.check_type();
    }

    pub fn append(&mut self, chunk: [u8; 2880]) {
        self.fitsblocks.push(chunk);
    }

    pub fn is_empty(&self) -> bool {
        self.fitsblocks.len() == 0
    }

    pub fn get_header_type(&self) -> HeaderType {
        self.header_type
    }

    pub fn print(&self) {
        for i in 0..self.fitsblocks.len() {
            println!("{}", str::from_utf8(&self.fitsblocks[i]).unwrap());
        }
    }

    pub fn list_keywords(&self) -> Vec<(String, String)> {
        let mut keywords = Vec::new();
        for (key, value) in &self.keywords {
            keywords.push((key.to_string(), value[0].to_string()));
        }
        keywords
    }

    pub fn get_keyword(&self, keyword: &str) -> Option<String> {
        match self.keywords.get(keyword) {
            Some(value) => Some(value[0].to_string()),
            None => Some("Field not found".to_string()),
        }
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
                value.retain(|c| !c.is_whitespace() && c != '\'');
                (keyword, [value, comment])
            } else {
                let mut value = String::from(parts[0]);
                value.retain(|c| !c.is_whitespace() && c != '\'');
                (keyword, [value, String::from("")])
            }
        } else {
            let value = String::from(str::from_utf8(&buffer[9..80]).unwrap());
            (keyword, [String::from(""), value])
        }
    }

    fn check_type(&self) -> HeaderType {
        if self.initiailzed == false {
            panic!("Header not initialized");
        }
        if self.keywords.contains_key("SIMPLE") {
            return HeaderType::Primary;
        } else if self.keywords.contains_key("XTENSION") {
            match self.keywords.get("XTENSION").unwrap()[0].as_str() {
                "IMAGE" => return HeaderType::Image,
                "BINTABLE" => return HeaderType::BinaryTable,
                "TABLE" => return HeaderType::ASCIITable,
                _ => panic!(
                    "Invalid XTENSION value {:?}. Only standard extension is supported for now",
                    self.keywords.get("XTENSION").unwrap()[0]
                ),
            }
        } else {
            panic!("Invalid header type");
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum HeaderType {
    Primary,
    Image,
    ASCIITable,
    BinaryTable,
}

impl fmt::Display for HeaderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", match self {
            HeaderType::Primary => "Primary",
            HeaderType::Image => "Image",
            HeaderType::ASCIITable => "ASCII Table",
            HeaderType::BinaryTable => "Binary Table",
        })
    }
}