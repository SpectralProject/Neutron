use alloc::vec::Vec;

pub struct AddressSpace {
    vmos: Vec<VirtualMemObject>,
}

pub fn map_vmo() {}

/// A vm object that is backed up by some memory feature like a block of RAM, an MMIO region, or a CoW view. Can be readily mapped into a process' address space
pub struct VirtualMemObject {}
