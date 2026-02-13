use super::object::{
    ObjectHeader, ObjectPointer, PROCSIZE
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
    pub fn new(interpreter: ObjectPointer) -> Self {
        Process {
            header: ObjectHeader::new(PROCSIZE),
            interpreter,
            state: ProcessState::Ready,
            next: None,
            prev: None,
        }
    }
}
