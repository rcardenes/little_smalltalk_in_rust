use super::object::{
    ObjectHeader,
    SYMBOLSIZE,
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
