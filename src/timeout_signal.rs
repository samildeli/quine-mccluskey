use std::sync::atomic::{AtomicBool, Ordering};

pub trait TTimeoutSignal: Default + Send + Sync {
    #[must_use]
    fn is_signaled(&self) -> bool;

    #[must_use]
    fn is_not_signaled(&self) -> bool {
        !self.is_signaled()
    }

    fn signal(&self);
}

#[derive(Default)]
pub struct TimeoutSignalNoOp;

#[derive(Default)]
pub struct TimeoutSignalAtomicBool {
    signal: AtomicBool,
}

impl TTimeoutSignal for TimeoutSignalNoOp {
    fn is_signaled(&self) -> bool {
        false
    }

    fn is_not_signaled(&self) -> bool {
        true
    }

    fn signal(&self) {}
}

impl TTimeoutSignal for TimeoutSignalAtomicBool {
    fn is_signaled(&self) -> bool {
        self.signal.load(Ordering::Acquire)
    }

    fn signal(&self) {
        self.signal.store(true, Ordering::Release);
    }
}
