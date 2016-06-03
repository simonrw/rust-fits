#![allow(dead_code, unused_variables)]

use std::fs;
use std::io::Read;

fn string_contains(string: &String, test: &str) -> bool {
    string.as_str().contains(test)
}

#[derive(Debug)]
pub enum HeaderValueType {
    Bool(bool),
    I64(i64),
    F64(f64),
    HString(String),
}

#[derive(Debug)]
struct HeaderCard {
    keyword: String,
    value: HeaderValueType,
    comment: Option<String>,
}

#[derive(Clone, Debug)]
enum HduType {
    Image,
    BinTable,
    AsciiTable,
}

#[derive(Default)]
struct FitsFile {
    offsets: Vec<(usize, HduLoc)>,
}

struct Page {
    hdu_type: Option<HduType>,
}

struct Header {
    hdu_type: Option<HduType>,
    cards: Vec<HeaderCard>,
}

impl Header {
    fn from_chunk(chunk: &[u8]) -> Header {
        let mut cards = Vec::new();

        for raw_card in chunk.chunks(80) {
            let header_card = HeaderCard::from_raw(raw_card);
            match header_card {
                Some(header_card) => {
                    let mut should_break = false;
                    if header_card.keyword.as_str() == "END" {
                        should_break = true;
                    }
                    cards.push(header_card);

                    if should_break {
                        break;
                    }
                }
                None => {},
            }
        }

        println!("{:?}", cards);

        Header {
            hdu_type: None,
            cards: cards,
        }
    }
}

impl Page {
    fn from_chunk(chunk: &[u8]) -> Page {
        let header = Header::from_chunk(&chunk);
        Page {
            hdu_type: header.hdu_type,
        }
    }
}

impl FitsFile {
    fn open(filename: &str) -> FitsFile {
        let mut loc = HduLoc::default();
        let mut page_number = 0;
        let mut npages = 0;

        let mut f = fs::File::open(filename).unwrap();
        let mut buffer = [0; 2880];

        let mut offsets = Vec::new();

        loop {
            let read_result = f.read(&mut buffer);
            match read_result {
                Ok(2880) => {
                },
                Ok(0) => {
                    // EOF
                    break;
                },
                Err(e) => panic!("Error reading from file"),
                _ => panic!("Woah wtf?!"),
            }

            let page = Page::from_chunk(&mut buffer);

            match page.hdu_type {
                Some(hdu_type) => {
                    offsets.push((page_number, loc.clone()));
                    loc = HduLoc::default();
                },
                None => {
                    npages += 1;
                },
            }

            page_number += 1;
        }

        FitsFile::default()
    }
}

#[derive(Clone, Debug)]
struct HduLoc {
    hdu_type: HduType,
    npages: usize,
    name: Option<String>,
}

impl Default for HduLoc {
    fn default() -> HduLoc {
        HduLoc {
            hdu_type: HduType::Image,
            npages: 0,
            name: None,
        }
    }
}

fn parse_value(string_value: &str) -> Result<HeaderValueType, i32> {
    let owned = string_value.to_string();

    // Parse booleans
    match owned.as_str() {
        "T" => return Ok(HeaderValueType::Bool(true)),
        "F" => return Ok(HeaderValueType::Bool(false)),
        _ => {},
    }

    // Integer type
    match owned.parse::<i64>() {
        Ok(int_value) => return Ok(HeaderValueType::I64(int_value)),
        _ => {},
    }

    // Floating point type
    match owned.parse::<f64>() {
        Ok(float_value) => return Ok(HeaderValueType::F64(float_value)),
        _ => {},
    }

    Err(1)
}

impl HeaderCard {
    fn from_raw(data: &[u8]) -> Option<HeaderCard> {
        let keyword = String::from_utf8_lossy(&data[0..8]).into_owned();
        let equals = data[8] as char;
        let rest = String::from_utf8_lossy(&data[9..]).into_owned();

        let (value, comment): (HeaderValueType, Option<String>) = if string_contains(&rest, "/") {
            let parts = rest.split('/').map(|s| s.trim()).collect::<Vec<_>>();
            let value = parse_value(parts[0]).unwrap();
            (value, Some(parts[1].to_string()))
        } else {
            (HeaderValueType::HString(rest), None)
        };

        Some(HeaderCard {
            keyword: keyword.trim().to_string(),
            value: value,
            comment: comment,
        })
    }

    pub fn to_string(&self) -> String {
        match self.comment {
            Some(ref comment) => {
                format!("{keyword:8}=", keyword = self.keyword)
            },
            None => {
                "".to_string()
            },
        }
    }
}


#[cfg(test)]
mod tests {
    #![allow(unused_imports)]

    #[test]
    fn parsing_a_boolean_card() {
        use super::{HeaderCard, HeaderValueType};
        //                            1         2         3         4         5         6         7         8
        //                   123456789012345678901234567890123456789012345678901234567890123456789012345678901
        let header_string = "SIMPLE  =                                  T / file does conform to FITS standard".to_string();
        let header_string_raw = header_string.as_bytes();

        let card = HeaderCard::from_raw(header_string_raw).unwrap();
        match card.value {
            HeaderValueType::Bool(value) => assert_eq!(value, true),
            _ => panic!("Invalid card value"),
        }

        match card.comment {
            Some(comment) => assert_eq!(comment, "file does conform to FITS standard"),
            _ => panic!("Invalid card comment"),
        }
    }

    #[test]
    fn parsing_an_integer_card() {
        use super::{HeaderCard, HeaderValueType};
        //                            1         2         3         4         5         6         7         8
        //                   123456789012345678901234567890123456789012345678901234567890123456789012345678901
        let header_string = "BITPIX  =                                      32 / number of bits per data pixel".to_string();
        let header_string_raw = header_string.as_bytes();

        let card = HeaderCard::from_raw(header_string_raw).unwrap();
        match card.value {
            HeaderValueType::I64(value) => assert_eq!(value, 32),
            _ => panic!("Invalid card type"),
        }

        match card.comment {
            Some(comment) => assert_eq!(comment, "number of bits per data pixel"),
            _ => panic!("Invalid card comment"),
        }
    }

    #[test]
    fn parsing_a_floating_point_card() {
        use super::{HeaderCard, HeaderValueType};
        //                            1         2         3         4         5         6         7         8
        //                   123456789012345678901234567890123456789012345678901234567890123456789012345678901
        let header_string = "DBLTEST =                                 0.09375 / Double value                 ".to_string();
        let header_string_raw = header_string.as_bytes();

        let card = HeaderCard::from_raw(header_string_raw).unwrap();
        match card.value {
            HeaderValueType::F64(value) => assert_eq!(value, 0.09375),
            _ => panic!("Invalid card type"),
        }

        match card.comment {
            Some(comment) => assert_eq!(comment, "Double value"),
            _ => panic!("Invalid card comment"),
        }
    }

    #[test]
    fn opening_a_file() {
        use super::{FitsFile, HduType};

        let f = FitsFile::open("testdata/test.fits");
        // assert_eq!(f.offsets.len(), 1);
        
        // let offset = &f.offsets[0].1;
        // assert_eq!(offset.hdu_type, HduType::Image);
        // assert_eq!(offset.npages, 1);
        // assert_eq!(offset.name, None);
    }
}
