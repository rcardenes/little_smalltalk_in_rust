use proc_macros::ValidSmalltalkObject;

use super::object::{
    SYMBOLSIZE,
    ValidObject,
    ObjectHeader, ObjectSize,
};

#[derive(Debug, ValidSmalltalkObject)]
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
