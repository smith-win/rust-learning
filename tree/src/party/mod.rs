/// A really basic party struct

#[derive(Debug)]
pub struct Party<'a> {

    pub legal_name: &'a str,
    pub org_id: i32

}


impl <'a> Party<'a> {

    pub fn new(id: i32, name: &str ) -> Party {
        Party {
            legal_name: name, org_id: id
        }
    }

}

