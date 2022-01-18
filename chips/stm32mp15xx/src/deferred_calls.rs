use core::convert::Into;
use core::convert::TryFrom;

/// A type of task to defer a call for
#[derive(Copy, Clone)]
pub enum DeferredCallTask {
    // Flash = 0,
}

impl TryFrom<usize> for DeferredCallTask {
    type Error = ();

    fn try_from(value: usize) -> Result<DeferredCallTask, ()> {
        match value {
            // 0 => Ok(DeferredCallTask::Flash),
            _ => Err(()),
        }
    }
}

impl Into<usize> for DeferredCallTask {
    fn into(self) -> usize {
        self as usize
    }
}
