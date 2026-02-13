use super::object::{
    FILESIZE,
    ObjectHeader,
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
