use super::object::{
    BLOCKSIZE,
    ObjectHeader, ObjectPointer,
};

pub struct Block {
    header: ObjectHeader,
    interpreter: ObjectPointer,
    numargs: u32,
    arglocation: u32,
}

impl Block {
    pub fn new(interpreter: ObjectPointer, numargs: u32, arglocation: u32) -> Self {
        Block {
            header: ObjectHeader::new(BLOCKSIZE),
            interpreter,
            numargs,
            arglocation,
        }
    }
}
