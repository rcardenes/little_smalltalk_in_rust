use super::object::{
    STRINGSIZE,
    ObjectHeader, ObjectPointer, ObjectSize,
};

pub struct StringObject {
    header: ObjectHeader,
    super_obj: ObjectPointer,
    value: String,
}

impl StringObject {
    const SIZE: ObjectSize = STRINGSIZE;

    pub fn new(value: String, super_obj: ObjectPointer) -> Self {
        StringObject {
            header: ObjectHeader::new(Self::SIZE),
            super_obj,
            value,
        }
    }
}
