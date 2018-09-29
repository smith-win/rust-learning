
/// Csv Parsing Config
/// 1 - basic parser (no qutoes)
/// 2 - quotes (no escapes / quotes in quotes)
/// 3 - escape 
/// 4 - multi-line ?

// IDEA ... can we use function programming / lambdas
// - an iterator of bytes -- Read trait provides this
// - an iterator of chars  (utf8 bytes)  -- sort of done, we need char interface to support encoding
// - an iterator of tokens (newrecord, attr=1, attr=Stuart, attr=Amersham, eof
// - also see "encoding" crate

// measure performance against CommonsCsv ?


// locating test resources -- 
//  let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));


use std::fs::File;
use std::io::{BufReader, BufRead,  Read};
use std::iter::Iterator;


const NEWLINE:u8 = 10;
const COMMA:u8 = 44;

///
pub enum CsvParserToken {
    EndAttr,
    EndRecord,
    Character(u8),
    AnotherSignificantChange
}

/// Structure of the CSV parser
pub struct CsvParser<'a> {
    pub delimiter: &'a str,

    buf_reader: Box<BufRead>,  // trait .. unknown size at compile so need to box it!
    intenal_buffer: [u8; 1024], // this is used to read into .. if we can dynamically size ... Box it
    buf_max_size: usize,
    buf_pos: usize,
    curr_attr_len: usize,
    next_end_record: bool
}



impl <'a> CsvParser<'a> {

    /// Create a new instance
    pub fn create(filename: &'a str) -> CsvParser<'a> {
        let file = File::open(filename).unwrap();

        // start with a default 4k buffer
        let buf_reader = BufReader::with_capacity(4096, file);
        CsvParser {
            delimiter: ","
            , buf_reader: Box::new(buf_reader)
            , intenal_buffer: [0u8; 1024]
            , buf_max_size: 0
            , buf_pos: 0

            , curr_attr_len: 0
            , next_end_record: false
        }
    }
}


/// Implementation of an iterator for CsvParser
/// Learning note: compare with Java, only single function as use Option
/// .. not next() and "hasNext()" ... just next.
/// ... makes ite easy to implement ?? .. (Like Guavas abstact iterator ?)
impl <'a> Iterator for CsvParser<'a> {

    type Item = CsvParserToken; // Associated type?

    // NV
    fn next(&mut self) -> Option<CsvParserToken> {


        if self.next_end_record {
            self.next_end_record = false;
            return Some(CsvParserToken::EndRecord);
        }

        // we'd move this to the struct itself .. 

        if self.buf_pos >= self.buf_max_size {
            // we can take more the internal buffer 
            // try and read from 
            self.buf_pos = 0;

            self.buf_max_size = self.buf_reader.read(&mut self.intenal_buffer).unwrap(); // TODO: SAFELY danger Will robinson
            println!("Read {} bytes from file", self.buf_max_size); //cd ..

            if self.buf_max_size == 0 {
                // No more data ..  
                return {
                    if self.curr_attr_len > 0 { 
                        self.curr_attr_len = 0;
                        self.next_end_record = true;
                        Some(CsvParserToken::EndAttr)
                    } else {
                        None
                    }
                }
            }
        }

        // a character in rust if 4bytes .. its a unicode "scalar value"
        let result = self.intenal_buffer[self.buf_pos];
        self.buf_pos += 1;

        // TODO: if do EndRecord ... need to do EndAttr id chars is +1

        // TODO: this dodgy, need effective UTF-8 stream de-coding ... 
        // detect start of UTF-8 char > 127 etc
        match result {
            NEWLINE => {
                // Next we make the "next" return end record if
                // we return attr
                if self.curr_attr_len != 0 {
                    self.curr_attr_len = 0;
                    self.next_end_record = true;
                    Some(CsvParserToken::EndAttr)
                } else {
                    Some(CsvParserToken::EndRecord)
                }
            },
            COMMA => {
                self.curr_attr_len = 0;
                Some(CsvParserToken::EndAttr)
            },
            _  =>  {
                self.curr_attr_len += 1;
                Some(CsvParserToken::Character(result))
            }
        }
    }

}
