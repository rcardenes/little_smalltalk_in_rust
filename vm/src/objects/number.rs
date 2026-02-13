use super::object::{
    FLOATSIZE, INTEGERSIZE,
    ValidObject,
    ObjectHeader, ObjectSize,
};
use proc_macros::ValidSmalltalkObject;

#[derive(Debug, ValidSmalltalkObject)]
pub struct Integer {
    header: ObjectHeader,
    value: i32,
}

impl Integer {
    const SIZE: ObjectSize = INTEGERSIZE;

    pub fn new(value: i32) -> Self {
        Integer {
            header: ObjectHeader::new(Self::SIZE),
            value,
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        Integer::is_valid(other) && self.value == other.value
    }
}

#[derive(Debug, ValidSmalltalkObject)]
pub struct Float {
    header: ObjectHeader,
    value: f64,
}

impl Float {
    const SIZE: ObjectSize = FLOATSIZE;

    pub fn new(value: f64) -> Self {
        Float {
            header: ObjectHeader::new(Self::SIZE),
            value,
        }
    }
}
