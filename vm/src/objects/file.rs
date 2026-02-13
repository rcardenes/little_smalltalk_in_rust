use super::object::{
    FILESIZE,
    ObjectHeader, ObjectSize,
};

enum FileMode {
    CharMode,
    StrMode,
    InteMode,
}

pub struct File {
    header: ObjectHeader,
    file_mode: FileMode,
    // Placeholder. Later we may implement it based on std::fs::File
    descriptor: usize,
}

impl File {
    const SIZE: ObjectSize = FILESIZE;
}

