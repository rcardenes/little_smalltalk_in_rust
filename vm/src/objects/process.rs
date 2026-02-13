use super::object::{
    PROCSIZE,
    ObjectHeader, ObjectPointer, ObjectSize,
};

pub enum ProcessState {
    Active,
    Suspended,
    Ready,
    Blocked,
    Unblocked,
    Terminated,
}

pub struct Process {
    header: ObjectHeader,
    interpreter: ObjectPointer,
    state: ProcessState,
    next: Option<ObjectPointer>,
    prev: Option<ObjectPointer>,
}

impl Process {
    const SIZE: ObjectSize = PROCSIZE;

    pub fn new(interpreter: ObjectPointer) -> Self {
        Process {
            header: ObjectHeader::new(Self::SIZE),
            interpreter,
            state: ProcessState::Ready,
            next: None,
            prev: None,
        }
    }
}
