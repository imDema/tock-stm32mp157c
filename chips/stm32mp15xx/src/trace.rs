//! # Trace Buffers
//!
//! Copyright (c) 2018, Cambridge Consultants Ltd.
//! MIT License.
//!
//! This module is for emitting text to
//! `/sys/kernel/debug/remoteproc/remoteproc0/trace`.

use core::fmt::Write;

use kernel::static_init;

const BUF_SIZE: usize = 2048;

/// Represents our tracebuffer. Uses a shared mutable buffer,
/// so only one of these can exist at any one time.
pub struct Trace<'a> {
    out_idx: usize,
    buffer: &'a mut [u8; BUF_SIZE],
}

/// Our output text buffer we share with the kernel. Must must must be linked
/// at address 0x10040000 and the size and address must match that given in
/// the `rt::Trace` part of `resource_trace::RESOURCE_TABLE`.
#[link_section = ".tracebuffer"]
#[no_mangle]
static mut TRACE_BUFFER: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

/// The first time you call this you'll get Some(t), where t
/// can be passed to `writeln!` and friends. The second
/// time you'll get None, so only call it once!
pub fn get_trace() -> &'static mut Trace<'static> {
    let trace = unsafe { static_init!(Trace, Trace {
        out_idx: 0,
        buffer: &mut TRACE_BUFFER,
    }) };
    trace
}

/// Only call this from a panic handler.
pub unsafe fn steal_trace() -> Trace<'static> {
    let mut used_space = 0_usize;
    for (idx, ch) in TRACE_BUFFER.iter().enumerate() {
        if *ch == 0 {
            used_space = idx;
            break;
        }
    }
    Trace {
        out_idx: used_space,
        buffer: &mut TRACE_BUFFER,
    }
}

impl<'a> Write for Trace<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), ::core::fmt::Error> {
        // Can never fit (with the null), so return an error.
        if (s.len() + 1) > self.buffer.len() {
            return Err(::core::fmt::Error);
        }

        let space = self.buffer.len() - self.out_idx;

        // Doesn't fit (with the null), let's wrap to make us some more space.
        if (s.len() + 1) > space {
            self.out_idx = 0;
        }

        for (s, d) in s
            .bytes()
            .zip(self.buffer[self.out_idx..self.out_idx + s.len()].iter_mut())
        {
            *d = s;
        }
        self.out_idx += s.len();
        self.buffer[self.out_idx] = 0;
        Ok(())
    }
}
