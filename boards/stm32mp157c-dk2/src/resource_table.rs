//! Based on https://github.com/cambridgeconsultants/rust-beagleboardx15-demo
//! (Under MIT License)
//! 
#![allow(missing_docs)]

use core::mem::size_of;


/// The types of entry you can have in a Resource Table.
#[repr(u32)]

pub enum ResourceType {
    /// Get the host to allocate you some memory
    CARVEOUT = 0,
    /// Map some MMIO registers in
    DEVMEM = 1,
    /// Point at a buffer where you can write strings
    TRACE = 2,
    /// Map a VirtIO device
    VDEV = 3,
}

/// All resource tables start with this header, followed by
/// the offset array.
#[repr(C)]
pub struct Header {
    pub ver: u32,
    pub num: usize,
    pub reserved: [u32; 2],
}

/// This is the structure for `ResourceType::TRACE`.
#[repr(C)]
#[allow(dead_code)]
pub struct Trace {
    pub rtype: ResourceType,
    pub da: usize,
    pub len: usize,
    pub reserved: u32,
    pub name: [u8; 32],
}

/// This is the structure for `ResourceType::VDEV`. It must be followed by the
/// appropriate number of `VdevVring` structures.
#[repr(C)]
#[allow(dead_code)]
pub struct Vdev {
    pub rtype: ResourceType,
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u8,
    pub num_of_vrings: u8,
    pub reserved: [u8; 2],
}

/// The individual vrings follow on from their `Vdev`.
#[repr(C)]
#[allow(dead_code)]
pub struct VdevVring {
    pub da: usize,
    pub align: usize,
    pub num: usize,
    pub notifyid: u32,
    pub reserved: u32,
}

const NUM_ENTRIES: usize = 1;

/// This resource table structure is processed by the kernel. We can map as
/// many resources as we require, but ensure that the offsets array is
/// calculated correctly. Resource tables are specific to each application,
/// but in this case it closely matches the TI example.
#[repr(C)]

pub struct ResourceTable {
    base: Header,
    offsets: [usize; NUM_ENTRIES],
    trace: Trace,
    // rpmsg_vdev: Vdev,
    // rpmsg_vring0: VdevVring,
    // rpmsg_vring1: VdevVring,
}

const OFFSET_START: usize = size_of::<Header>() + 4 * NUM_ENTRIES;

#[link_section = ".resource_table"]
#[no_mangle]
pub static RESOURCE_TABLE: ResourceTable = ResourceTable {
    base: Header {
        ver: 1,
        num: NUM_ENTRIES,
        reserved: [0, 0],
    },
    // We don't have an offsetof macro so we have to calculate these by hand
    offsets: [
        OFFSET_START,
        // OFFSET_START + size_of::<Vdev>(),
        // OFFSET_START + size_of::<Vdev>() + size_of::<VdevVring>(),
    ],

    trace: Trace {
        rtype: ResourceType::TRACE,
        /// We must ensure that the tracebuffer structure is linked at this
        /// address. We do this by placing it at the start of the ".tracebuffer"
        /// section. Ideally we'd just take the address of our buffer
        /// but that's now allowed in a static variable definition.
        da: 0x10040000,
        len: 2048,
        reserved: 0,
        name: *b"cm4_log\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    },


    // rpmsg_vdev: Vdev {
    //     rtype: ResourceType::VDEV,
    //     id: 7,
    //     notifyid: 0,
    //     dfeatures: 1,
    //     gfeatures: 0,
    //     config_len: 0,
    //     status: 0,
    //     num_of_vrings: 2,
    //     reserved: [0, 0],
    // },

    // /// vring0 is for rproc-to-Linux comms
    // rpmsg_vring0: VdevVring {
    //     da: 0x10040400,
    //     align: 4,
    //     num: 4,
    //     notifyid: 0,
    //     reserved: 0,
    // },

    // /// vring1 is for Linux-to-rproc comms
    // rpmsg_vring1: VdevVring {
    //     da: 0x10040800,
    //     align: 4,
    //     num: 4,
    //     notifyid: 1,
    //     reserved: 0,
    // },
};

