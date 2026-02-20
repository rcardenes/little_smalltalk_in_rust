use proc_macros::ValidSmalltalkObject;

use super::object::{
    BYTEARRAYSIZE,
    ValidObject,
    ObjectHeader, ObjectSize,
};

#[derive(Debug, ValidSmalltalkObject)]
pub struct ByteArray {
    header: ObjectHeader,
    value: Vec<u8>,
}

impl ByteArray {
    const SIZE: ObjectSize = BYTEARRAYSIZE;

    pub fn new(value: Vec<u8>) -> Self {
        ByteArray {
            header: ObjectHeader::new(Self::SIZE),
            value,
        }
    }
}

impl PartialEq for ByteArray {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
