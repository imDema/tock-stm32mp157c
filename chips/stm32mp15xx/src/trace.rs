//! # Trace Buffers
//!
//! Based on https://github.com/cambridgeconsultants/rust-beagleboardx15-demo (MIT License)
//!
//! This module is for emitting text to
//! `/sys/kernel/debug/remoteproc/remoteproc0/trace`.

use core::fmt::Write;

use kernel::debug::IoWrite;
use kernel::static_init;

pub const TRACE_BUF_SIZE: usize = 0x1000;

/// Represents our tracebuffer. Uses a shared mutable buffer,
/// so only one of these can exist at any one time.
pub struct TraceBuffer<'a> {
    pos: usize,
    buffer: &'a mut [u8; TRACE_BUF_SIZE],
}

/// Our output text buffer we share with the kernel. Must must must be linked
/// at address 0x10040000 and the size and address must match that given in
/// the `rt::Trace` part of `resource_trace::RESOURCE_TABLE`.
#[link_section = ".tracebuffer"]
#[no_mangle]
static mut TRACE_BUFFER: [u8; TRACE_BUF_SIZE] = [0u8; TRACE_BUF_SIZE];

pub unsafe fn get_trace() -> &'static mut TraceBuffer<'static> {
    static_init!(TraceBuffer, TraceBuffer{
        pos: 0,
        buffer: &mut TRACE_BUFFER,
    })
}

/// Only call this from a panic handler.
pub unsafe fn steal_trace() -> TraceBuffer<'static> {
    let mut used_space = 0_usize;
    for (idx, ch) in TRACE_BUFFER.iter().enumerate() {
        if *ch == 0 {
            used_space = idx;
            break;
        }
    }
    TraceBuffer {
        pos: used_space,
        buffer: &mut TRACE_BUFFER,
    }
}

impl<'a> Write for TraceBuffer<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), ::core::fmt::Error> {
        let byte_len = s.as_bytes().len();
        let space = self.buffer.len() - self.pos;

        // Doesn't fit (with the null), let's wrap to make us some more space.
        if (byte_len + 1) > space {
            self.pos = 0;
        }

        // Truncates output if it doesn't fit
        let mut written = 0;
        for (s, d) in s
            .bytes()
            .filter(|&c| c != b'\0')
            .zip(self.buffer[self.pos..].iter_mut())
        {
            *d = s;
            written += 1;
        }
        self.pos = (self.buffer.len() - 1).min(self.pos + written);
        self.buffer[self.pos] = 0;
        Ok(())
    }
}

impl<'a> IoWrite for TraceBuffer<'a> {
    fn write(&mut self, buf: &[u8]) {
        for &c in buf {
            self.write_char(c as char).unwrap();
        }
    }
}
