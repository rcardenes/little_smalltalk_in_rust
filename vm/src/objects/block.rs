use super::object::{
    BLOCKSIZE,
    ObjectHeader, ObjectPointer, ObjectSize,
};

pub struct Block {
    header: ObjectHeader,
    interpreter: ObjectPointer,
    numargs: u32,
    arglocation: u32,
}

impl Block {
    const SIZE: ObjectSize = BLOCKSIZE;

    pub fn new(interpreter: ObjectPointer, numargs: u32, arglocation: u32) -> Self {
        Block {
            header: ObjectHeader::new(Self::SIZE),
            interpreter,
            numargs,
            arglocation,
        }
    }
}
