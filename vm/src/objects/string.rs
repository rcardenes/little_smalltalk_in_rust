use super::object::{
    STRINGSIZE,
    ObjectHeader, ObjectPointer,
};

pub struct StringObject {
    header: ObjectHeader,
    super_obj: ObjectPointer,
    value: String,
}

impl StringObject {
    pub fn new(value: String, super_obj: ObjectPointer) -> Self {
        StringObject {
            header: ObjectHeader::new(STRINGSIZE),
            super_obj,
            value,
        }
    }
}
