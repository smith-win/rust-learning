

mod csv;

/// This my first attempt at unit testing in Rust



#[cfg(test)]
mod tests {
    use csv::{CsvParser, CsvParserToken};

    #[test]
    fn it_works() {

        let mut  count = 0;
        let csv_parser = CsvParser::create("/home/spuka/rust-learning/csv/testfiles/test1.csv");
        let mut string = String::new();
        for y in csv_parser {
            count+=1;
            match y {
                CsvParserToken::Character(n) => string.push(n as char),
                CsvParserToken::EndAttr => {print!("[{}]", string); string.clear()}
                CsvParserToken::EndRecord => println!("<"),
                _ => {} //hack
            }
        }
        assert_eq!(49, count);
    }
}
