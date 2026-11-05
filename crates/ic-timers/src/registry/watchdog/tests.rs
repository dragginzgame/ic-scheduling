use super::WatchdogControl;

impl WatchdogControl {
    // Seed exhaustion cases before allocating their real callback generations.
    pub(in crate::registry) const fn set_generation_for_test(&mut self, generation: u64) {
        self.generation = generation;
    }
}
