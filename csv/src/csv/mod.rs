
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

use std::fs::File;
use std::io::{BufReader, BufRead,  Read};
use std::iter::Iterator;

///
pub enum CsvParserToken {
    NewAttr,
    NewRecord,
    Character(u8)
}

/// Structure of the CSV parser
pub struct CsvParser<'a> {
    pub delimiter: &'a str,
    buf_reader: Box<BufRead>,  // trait .. unknown size at comiple so need to box it!
    intenal_buffer: [u8; 1024], // this is used to read into .. if we can dynamically size ... Box it
    buf_max_size: usize,
    buf_pos: usize
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
        }
    }

}


/// Implementation of an iterator for CsvParser
/// Learning note: compare with Java, only single function as use Option
/// .. not next() and "hasNext()" ... just next.
/// ... makes ite easy to implement ?? .. (Like Guavas abstact iterator ?)
impl <'a> Iterator for CsvParser<'a> {
    type Item = CsvParserToken;


    // NV
    fn next(&mut self) -> Option<CsvParserToken> {
        // we'd move this to the struct itself .. 

        if self.buf_pos >= self.buf_max_size {
            // we can take more the internal buffer 
            // try and read from 
            self.buf_pos = 0;
            self.buf_max_size = self.buf_reader.read(&mut self.intenal_buffer).unwrap(); // danger Will robinson
            println!("Read {} bytes from file", self.buf_max_size);

            if self.buf_max_size == 0 {
                return None
            }
        }

        // a character in rust if 4byes .. its a unicode "scalar value"
        let result = self.intenal_buffer[self.buf_pos];
        self.buf_pos += 1;

        match result {
            10 => Some(CsvParserToken::NewRecord),
            44 => Some(CsvParserToken::NewAttr),
            _  =>  Some(CsvParserToken::Character(result))
        }
    }


}
