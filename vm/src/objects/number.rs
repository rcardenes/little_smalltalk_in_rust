use super::object::{
    FLOATSIZE, INTEGERSIZE,
    ValidObject,
    ObjectHeader,
};

#[derive(Debug)]
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

impl ValidObject for Integer {
    fn is_valid(obj: &Self) -> bool {
        obj.header.is_size(INTEGERSIZE)
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        Integer::is_valid(other) && self.value == other.value
    }
}

#[derive(Debug)]
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

impl ValidObject for Float {
    fn is_valid(obj: &Self) -> bool {
        obj.header.is_size(FLOATSIZE)
    }
}
