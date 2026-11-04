//! Observe the single IC timer without treating its one-shot reset as theft.
#![forbid(unsafe_code)]

#[derive(Clone, Copy)]
pub(super) struct TimerOwnership {
    expected: u64,
}

impl TimerOwnership {
    pub(super) const fn new() -> Self {
        Self { expected: 0 }
    }

    /// Commit only an observed prior state consistent with our last write.
    /// Zero after the deadline can also mean the IC scheduled its one-shot.
    pub(super) const fn record_replacement(&mut self, prior: u64, next: u64, now: u64) -> bool {
        if prior == self.expected || (prior == 0 && self.expected <= now) {
            self.expected = next;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests;
