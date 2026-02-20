// Object descriptors for special types

pub type RefCount = u32;
pub type ObjectSize = i32;
pub type ObjectPointer = u32;

pub const INVALIDSIZE: ObjectSize =   -1;

pub const BLOCKSIZE: ObjectSize =     -83;
pub const BYTEARRAYSIZE: ObjectSize = -567;
pub const CHARSIZE: ObjectSize =      -33;
pub const CLASSSIZE: ObjectSize =     -3;
pub const FILESIZE: ObjectSize =      -5;
pub const FLOATSIZE: ObjectSize =     -31415;
pub const INTEGERSIZE: ObjectSize =   -17;
pub const INTERPSIZE: ObjectSize =    -15;
pub const PROCSIZE: ObjectSize =      -100;
pub const STRINGSIZE: ObjectSize =    -258;
pub const SYMBOLSIZE: ObjectSize =    -14;


// Extension trait for object pointers
//
// This implementation of smalltalk preallocates blocks of memory that are then used
// to store objects. The size of each memory block is in multiples of the size of the
// specific object class being allocated (e.g. Block, ByteArray, etc).
//
// These memory blocks are stored in vectors (one per object class). A pointer must
// then be able to reference two pieces of information:
//
//   * the index of the memory block in the vector
//   * the offset within that memory block (expressed as an index)
//
// Naively we'll use the upper 16 bits of the pointer to store the block index and
// the lower 16 bits to store the offset.

pub trait Pointer {
    fn null() -> Self;
    fn is_null(&self) -> bool;
    fn new_from_index_and_offset(block_index: usize, offset: usize) -> Self;
    fn block_index(&self) -> usize;
    fn offset(&self) -> usize;
}

impl Pointer for ObjectPointer {
    fn null() -> Self
        where Self: Sized
    {
        u32::max_value()
    }

    fn is_null(&self) -> bool {
        *self == Self::null()
    }

    fn new_from_index_and_offset(block_index: usize, offset: usize) -> Self {
        ((block_index as u32) << 16) | (offset as u32)
    }

    fn block_index(&self) -> usize {
        (*self as u32 >> 16) as usize
    }

    fn offset(&self) -> usize {
        (*self & 0xFFFF) as usize
    }
}

pub trait ValidObject {
    fn is_valid(obj: &Self) -> bool;
    fn set_invalid(obj: &mut Self);
}

#[derive(Debug)]
pub struct ObjectHeader {
    ref_count:  RefCount,
    size:       ObjectSize,
}

impl ObjectHeader {
    pub fn new(size: ObjectSize) -> Self {
        Self {
            ref_count: 0,
            size,
        }
    }

    pub fn is_size(&self, size: ObjectSize) -> bool {
        self.size == size
    }

    pub fn set_invalid(&mut self) {
        self.ref_count = 0;
        self.size = INVALIDSIZE;
    }

    pub fn null() -> Self {
        Self {
            ref_count: 0,
            size: INVALIDSIZE,
        }
    }
}

pub enum ObjectType {
    Block,
    ByteArray,
    Char,
    Class,
    File,
    Float,
    Integer,
    Interpreter,
    Object,
    Process,
    String,
    Symbol,
}

impl ObjectType {
    pub fn find(ptr: *const u8) -> Option<ObjectType> {
        let header: &ObjectHeader = unsafe {
            &*(ptr as *const ObjectHeader)
        };

        match header.size {
            BLOCKSIZE => Some(ObjectType::Block),
            BYTEARRAYSIZE => Some(ObjectType::ByteArray),
            CHARSIZE => Some(ObjectType::Char),
            CLASSSIZE => Some(ObjectType::Class),
            FILESIZE => Some(ObjectType::File),
            FLOATSIZE => Some(ObjectType::Float),
            INTEGERSIZE => Some(ObjectType::Integer),
            INTERPSIZE => Some(ObjectType::Interpreter),
            PROCSIZE => Some(ObjectType::Process),
            STRINGSIZE => Some(ObjectType::String),
            SYMBOLSIZE => Some(ObjectType::Symbol),
            s if s >= 0 => Some(ObjectType::Object),
            _ => None,
        }
    }
}

pub struct Object {
    header:     ObjectHeader,
    class:      ObjectPointer,
    super_obj:  ObjectPointer,
    inst_var:   Vec<ObjectPointer>,
}
