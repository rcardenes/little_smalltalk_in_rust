use super::object::{
    BYTEARRAYSIZE,
    ObjectHeader
};

pub struct ByteArray {
    header: ObjectHeader,
    value: Vec<u8>,
}

impl ByteArray {
    pub fn new(value: Vec<u8>) -> Self {
        ByteArray {
            header: ObjectHeader::new(BYTEARRAYSIZE),
            value,
        }
    }
}
