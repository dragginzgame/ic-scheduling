//! Bounded callback environments; a slot is reusable only after ownership returns.
#![forbid(unsafe_code)]

pub(super) struct Slots<T, const LIMIT: usize>(Vec<Option<T>>);

impl<T, const LIMIT: usize> Slots<T, LIMIT> {
    pub(super) const fn new() -> Self {
        Self(Vec::new())
    }

    pub(super) fn insert(&mut self, value: T) -> Result<usize, T> {
        let index = self
            .0
            .iter()
            .position(Option::is_none)
            .unwrap_or(self.0.len());
        if index == LIMIT {
            return Err(value);
        }
        if index == self.0.len() {
            self.0.push(Some(value));
        } else {
            self.0[index] = Some(value);
        }
        Ok(index)
    }

    pub(super) fn take(&mut self, index: usize) -> Option<T> {
        self.0.get_mut(index).and_then(Option::take)
    }
}

#[cfg(test)]
mod tests;
