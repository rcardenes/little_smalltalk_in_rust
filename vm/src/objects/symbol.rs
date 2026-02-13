use super::object::{
    SYMBOLSIZE,
    ObjectHeader, ValidObject, ObjectSize,
};

#[derive(Debug)]
pub struct Symbol {
    header: ObjectHeader,
    value: String,
}

impl Symbol {
    const SIZE: ObjectSize = SYMBOLSIZE;

    pub fn new(value: String) -> Self {
        Symbol {
            header: ObjectHeader::new(Self::SIZE),
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
        obj.header.is_size(Self::SIZE)
    }
}
