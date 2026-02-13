use super::object::{
    ObjectPointer, ObjectHeader,
    Pointer,
};


struct Class {
    header:         ObjectHeader,
    name:           ObjectPointer,
    super_class:    ObjectPointer,
    file_name:      ObjectPointer,
    c_inst_vars:    ObjectPointer,
    context_size:   u32,
    message_names:  ObjectPointer,
    methods:        ObjectPointer,
    stack_max:      u32,
}

impl Class {
    pub fn new() -> Self {
        Self {
            header: ObjectHeader::new(0),
            name: ObjectPointer::null(),
            super_class: ObjectPointer::null(),
            file_name: ObjectPointer::null(),
            c_inst_vars: ObjectPointer::null(),
            context_size: 0,
            message_names: ObjectPointer::null(),
            methods: ObjectPointer::null(),
            stack_max: 0,
        }
    }
}
