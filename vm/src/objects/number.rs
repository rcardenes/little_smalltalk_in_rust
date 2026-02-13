use super::object::{
    FLOATSIZE, INTEGERSIZE,
    ObjectHeader,
};

pub struct Integer {
    header: ObjectHeader,
    value: i32,
}

impl Integer {
    pub fn new(value: i32) -> Self {
        Integer {
            header: ObjectHeader::new(INTEGERSIZE),
            value,
        }
    }
}

pub struct Float {
    header: ObjectHeader,
    value: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Float {
            header: ObjectHeader::new(FLOATSIZE),
            value,
        }
    }
}
