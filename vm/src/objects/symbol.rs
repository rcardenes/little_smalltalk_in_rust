use super::object::{
    ObjectHeader,
    SYMBOLSIZE,
};

pub struct Symbol {
    header: ObjectHeader,
    value: String,
}

impl Symbol {
    pub fn new(value: String) -> Self {
        Symbol {
            header: ObjectHeader::new(SYMBOLSIZE),
            value,
        }
    }
}
