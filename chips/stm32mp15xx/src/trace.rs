//! # Trace Buffers
//!
//! Based on Trace buffers implementation by
//! Copyright (c) 2018, Cambridge Consultants Ltd.
//! MIT License.
//!
//! This module is for emitting text to
//! `/sys/kernel/debug/remoteproc/remoteproc0/trace`.

use core::fmt::Write;

use kernel::debug::IoWrite;
use kernel::static_init;
use kernel::hil::uart;
use kernel::utilities::cells::{OptionalCell, TakeCell};

pub const TRACE_BUF_SIZE: usize = 0x1000;

pub struct Trace<'a> {
    buffer: TakeCell<'a, TraceBuffer<'a>>,
    tx_client: OptionalCell<&'a dyn uart::TransmitClient>,
}

/// Represents our tracebuffer. Uses a shared mutable buffer,
/// so only one of these can exist at any one time.
pub struct TraceBuffer<'a> {
    out_idx: usize,
    buffer: &'a mut [u8; TRACE_BUF_SIZE],
}

/// Our output text buffer we share with the kernel. Must must must be linked
/// at address 0x10040000 and the size and address must match that given in
/// the `rt::Trace` part of `resource_trace::RESOURCE_TABLE`.
#[link_section = ".tracebuffer"]
#[no_mangle]
static mut TRACE_BUFFER: [u8; TRACE_BUF_SIZE] = [0u8; TRACE_BUF_SIZE];

/// The first time you call this you'll get Some(t), where t
/// can be passed to `writeln!` and friends. The second
/// time you'll get None, so only call it once!
pub unsafe fn get_trace() -> &'static mut Trace<'static> {
    let trace = Trace {
        tx_client: OptionalCell::empty(),

        buffer: TakeCell::new(static_init!(TraceBuffer, TraceBuffer{
            out_idx: 0,
            buffer: &mut TRACE_BUFFER,
        }))
    };

    static_init!(Trace, trace)
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
        out_idx: used_space,
        buffer: &mut TRACE_BUFFER,
    }
}

impl<'a> Write for TraceBuffer<'a> {
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

impl<'a> IoWrite for TraceBuffer<'a> {
    fn write(&mut self, buf: &[u8]) {
        for &c in buf {
            self.write_char(c as char).unwrap();
        }
    }
}


impl<'a> Write for Trace<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), ::core::fmt::Error> {
        let result = self.buffer.map(|b| b.write_str(s));
        match result {
            Some(Ok(_)) => Ok(()),
            _ => Err(::core::fmt::Error)
        }
    }
}

impl<'a> IoWrite for Trace<'a> {
    fn write(&mut self, buf: &[u8]) {
        for &c in buf {
            self.write_char(c as char).unwrap();
        }
    }
}

impl<'a> uart::Receive<'a> for Trace<'a> {
    fn set_receive_client(&self, _client: &'a dyn uart::ReceiveClient) {}

    fn receive_buffer(
        &self,
        _rx_buffer: &'static mut [u8],
        _rx_len: usize,
    ) -> Result<(), (kernel::ErrorCode, &'static mut [u8])> {
        unimplemented!();
    }

    fn receive_word(&self) -> Result<(), kernel::ErrorCode> {
        unimplemented!();
    }

    fn receive_abort(&self) -> Result<(), kernel::ErrorCode> {
        Ok(())
    }
}

impl<'a> uart::Transmit<'a> for Trace<'a> {
    fn set_transmit_client(&self, client: &'a dyn uart::TransmitClient) {
        self.tx_client.set(client);
    }

    fn transmit_buffer(
        &self,
        tx_data: &'static mut [u8],
        tx_len: usize,
    ) -> Result<(), (kernel::ErrorCode, &'static mut [u8])> {
        if let Some(_) = self.buffer.map(|b| b.write(&tx_data[..tx_len])) {
            self.tx_client.map(|client| {
                client.transmitted_buffer(tx_data, tx_len, Ok(()));
            });
            Ok(())
        } else {
            Err((kernel::ErrorCode::FAIL, tx_data))
        }
    }

    fn transmit_word(&self, _word: u32) -> Result<(), kernel::ErrorCode> {
        // let mut buf = word.to_ne_bytes();
        // self.transmit_buffer(&mut buf, buf.len())
        //     .map_err(|e| e.0)
        unimplemented!();
    }

    fn transmit_abort(&self) -> Result<(), kernel::ErrorCode> {
        unimplemented!();
    }
}

impl<'a> uart::Configure for Trace<'a> {
    fn configure(&self, _params: uart::Parameters) -> Result<(), kernel::ErrorCode> {
        // Do nothing, virtual uart!
        Ok(())
    }
}