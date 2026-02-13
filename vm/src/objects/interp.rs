use super::object::{
    INTERPSIZE,
    ObjectHeader, ObjectPointer,
};

pub struct Interpreter {
    header: ObjectHeader,
    creator: ObjectPointer,
    sender: ObjectPointer,
    bytecode: ObjectPointer,
    receiver: ObjectPointer,
    literals: ObjectPointer,
    context: ObjectPointer,
    stack: ObjectPointer,
    // stacktop: ... In C this is object**. Figure out how it is used
    current_byte: u32,
}

// impl Interpreter {
//     pub fn new() -> Self {
//         Interpreter {
//             header: ObjectHeader::new(INTERPSIZE),
//             // creator: ObjectPointer::null(),
//             // sender: ObjectPointer::null(),
//             // bytecode: ObjectPointer::null(),
//             // receiver: ObjectPointer::null(),
//             // literals: ObjectPointer::null(),
//             // context: ObjectPointer::null(),
//             // stack: ObjectPointer::null(),
//             current_byte: 0,
//         }
//     }
// }
