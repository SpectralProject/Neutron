// use crate::types::Binary;

// !TEMP: dynamically allocated stuff
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

// Semantic Filesystem
pub struct EmberFS {
    files: Vec<File>
}

// Hierarchical Filesystem
pub struct Filesystem {
    pub files: Vec<File>
}

type NBits = u64;

pub struct File {
    pub size: NBits //in bits, e.g. 10270bits
}

struct BinaryFile {
    metadata: File,
    // content: Binary
}

struct AsciiFile {
    pub metadata: File,
    pub content: Vec<u8>
}

impl Filesystem {
    pub fn new(&self) -> Filesystem {
        let f = File{size: 100};
        Filesystem{files: Vec::new()}
    }
}