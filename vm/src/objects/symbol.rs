use super::object::{
    SYMBOLSIZE,
    ObjectHeader, ValidObject,
};

#[derive(Debug)]
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

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl ValidObject for Symbol {
    fn is_valid(obj: &Self) -> bool {
        obj.header.is_size(SYMBOLSIZE)
    }
}
