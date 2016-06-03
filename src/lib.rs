#![allow(dead_code, unused_variables)]

use std::fs;
use std::io::Read;

#[derive(Debug)]
pub enum HeaderValueType {
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
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
                Some(header_card) => cards.push(header_card),
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


impl HeaderCard {
    fn from_raw(data: &[u8]) -> Option<HeaderCard> {
        let keyword = String::from_utf8_lossy(&data[0..8]).into_owned();
        let equals = data[8] as char;

        Some(HeaderCard {
            keyword: keyword.trim().to_string(),
            value: HeaderValueType::Bool(true),
            comment: None,
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
    // #[test]
    // fn building_a_header_card() {
    //     use super::HeaderCard;

    //     let card = HeaderCard::without_comment("SIMPLE", true);
    //     assert_eq!(card.keyword, "SIMPLE");
    //     assert!(card.value);
    //     assert_eq!(card.comment, None);

    //     let card = HeaderCard::without_comment("BITPIX", 32);
    //     assert_eq!(card.keyword, "BITPIX");
    //     assert_eq!(card.value, 32);
    //     assert_eq!(card.comment, None);

    //     let card = HeaderCard::with_comment("NAXIS", 2, "Number of axes");
    //     assert_eq!(card.keyword, "NAXIS");
    //     assert_eq!(card.value, 2);
    //     assert_eq!(card.comment, Some("Number of axes".to_string()));
    // }

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
