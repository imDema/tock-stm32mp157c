//! Based on https://github.com/cambridgeconsultants/rust-beagleboardx15-demo
//! (Under MIT License)
//! 
// struct String32 {
//     pub buffer: [u8; 32],
// }

#![allow(missing_docs)]

/// The types of entry you can have in a Resource Table.
#[repr(u32)]
#[derive(Debug)]
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

// const SZ_RT_HEADER: usize = 16;

/// All resource tables start with this header, followed by
/// the offset array.
#[repr(C)]
#[derive(Debug)]
pub struct Header {
    pub ver: u32,
    pub num: usize,
    pub reserved: [u32; 2],
}

// /// This is the structure for `ResourceType::CARVEOUT`.
// #[repr(C)]
// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct Carveout {
//     pub rtype: ResourceType,
//     pub da: usize,
//     pub pa: usize,
//     pub len: usize,
//     pub flags: u32,
//     pub reserved: u32,
//     pub name: String32,
// }

// /// This is the structure for `ResourceType::DEVMEM`.
// #[repr(C)]
// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct Devmem {
//     pub rtype: ResourceType,
//     pub da: usize,
//     pub pa: usize,
//     pub len: usize,
//     pub flags: u32,
//     pub reserved: u32,
//     pub name: String32,
// }

// /// This is the structure for `ResourceType::TRACE`.
// #[repr(C)]
// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct Trace {
//     pub rtype: ResourceType,
//     pub da: usize,
//     pub len: usize,
//     pub reserved: u32,
//     pub name: String32,
// }

// /// This is the structure for `ResourceType::VDEV`. It must be followed by the
// /// appropriate number of `VdevVring` structures.
// #[repr(C)]
// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct Vdev {
//     pub rtype: ResourceType,
//     pub id: u32,
//     pub notifyid: u32,
//     pub dfeatures: u32,
//     pub gfeatures: u32,
//     pub config_len: u32,
//     pub status: u8,
//     pub num_of_vrings: u8,
//     pub reserved: [u8; 2],
// }

// /// The individual vrings follow on from their `Vdev`.
// #[repr(C)]
// #[derive(Debug)]
// #[allow(dead_code)]
// pub struct VdevVring {
//     pub da: usize,
//     pub align: usize,
//     pub num: usize,
//     pub notifyid: u32,
//     pub reserved: u32,
// }

// pub trait AddressMapper {
//     /// Convert a physical address (e.g. an L3/L4 address) to a device address the Cortex-M4 can use.
//     fn pa_to_da(&self, pa: usize) -> Option<usize>;
//     /// Convert a device address the Cortex-M4 can use to a physical address (e.g. an L3/L4 address).
//     fn da_to_pa(&self, da: usize) -> Option<usize>;
// }

// pub trait Region {
//     fn get_pa(&self) -> usize;
//     fn get_da(&self) -> usize;
//     fn get_len(&self) -> usize;
// }

// // Size constants must match those used on host: include/asm-generic/sizes.h

// #[allow(dead_code)]
// pub const SZ_64K: usize = 0x00010000;
// #[allow(dead_code)]
// pub const SZ_128K: usize = 0x00020000;
// #[allow(dead_code)]
// pub const SZ_256K: usize = 0x00040000;
// #[allow(dead_code)]
// pub const SZ_512K: usize = 0x00080000;
// #[allow(dead_code)]
// pub const SZ_1M: usize = 0x00100000;
// #[allow(dead_code)]
// pub const SZ_2M: usize = 0x00200000;
// #[allow(dead_code)]
// pub const SZ_4M: usize = 0x00400000;
// #[allow(dead_code)]
// pub const SZ_8M: usize = 0x00800000;
// #[allow(dead_code)]
// pub const SZ_16M: usize = 0x01000000;
// #[allow(dead_code)]
// pub const SZ_32M: usize = 0x02000000;
// #[allow(dead_code)]
// pub const SZ_64M: usize = 0x04000000;
// #[allow(dead_code)]
// pub const SZ_128M: usize = 0x08000000;
// #[allow(dead_code)]
// pub const SZ_256M: usize = 0x10000000;
// #[allow(dead_code)]
// pub const SZ_512M: usize = 0x20000000;

// impl Region for Carveout {
//     fn get_pa(&self) -> usize {
//         self.pa
//     }
//     fn get_da(&self) -> usize {
//         self.da
//     }
//     fn get_len(&self) -> usize {
//         self.len
//     }
// }

// impl Region for Devmem {
//     fn get_pa(&self) -> usize {
//         self.pa
//     }
//     fn get_da(&self) -> usize {
//         self.da
//     }
//     fn get_len(&self) -> usize {
//         self.len
//     }
// }

// impl Region {
//     /// Convert a physical address (e.g. an L3/L4 address) to a device address the Cortex-M4 can use.
//     pub fn pa_to_da(&self, given_pa: usize) -> Option<usize> {
//         if (given_pa >= self.get_pa()) && (given_pa < (self.get_pa() + self.get_len())) {
//             let offset = given_pa - self.get_pa();
//             Some(self.get_da() + offset)
//         } else {
//             None
//         }
//     }

//     /// Convert a device address the Cortex-M4 can use to a physical address (e.g. an L3/L4 address).
//     pub fn da_to_pa(&self, given_da: usize) -> Option<usize> {
//         if (given_da >= self.get_da()) && (given_da < (self.get_da() + self.get_len())) {
//             let offset = given_da - self.get_da();
//             Some(self.get_pa() + offset)
//         } else {
//             None
//         }
//     }
// }

const NUM_ENTRIES: usize = 0;

/// This resource table structure is processed by the kernel. We can map as
/// many resources as we require, but ensure that the offsets array is
/// calculated correctly. Resource tables are specific to each application,
/// but in this case it closely matches the TI example.
#[repr(C)]
#[derive(Debug)]
pub struct ResourceTable {
    base: Header,
    offsets: [usize; NUM_ENTRIES],
    // rpmsg_vdev: Vdev,
    // rpmsg_vring0: VdevVring,
    // rpmsg_vring1: VdevVring,
    // trace: Trace,
}

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
        // SZ_RT_HEADER,
        // SZ_RT_HEADER + 68,
        // SZ_RT_HEADER + 124,
        // SZ_RT_HEADER + 180,
        // SZ_RT_HEADER + 236,
    ],

    // rpmsg_vdev: Vdev {
    //     rtype: ResourceType::VDEV,
    //     id: vring::VIRTIO_ID_RPMSG,
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
    //     da: 0x60000000,
    //     align: 4096,
    //     num: 256,
    //     notifyid: 1,
    //     reserved: 0,
    // },

    // /// vring1 is for Linux-to-rproc comms
    // rpmsg_vring1: VdevVring {
    //     da: 0x60004000,
    //     align: 4096,
    //     num: 256,
    //     notifyid: 2,
    //     reserved: 0,
    // },

    // trace: Trace {
    //     rtype: ResourceType::TRACE,
    //     /// We must ensure that the tracebuffer structure is linked at this
    //     /// address. We do this by placing it at the start of the ".tracebuffer"
    //     /// section. Ideally we'd just take the address of our buffer
    //     /// but that's now allowed in a static variable definition.
    //     da: 0x9F000000,
    //     len: 16384,
    //     reserved: 0,
    //     name: String32 {
    //         buffer: *b"trace:sysm3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    //     },
    // },
};

