use super::object::{
    BYTEARRAYSIZE,
    ObjectHeader, ObjectSize,
};

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
