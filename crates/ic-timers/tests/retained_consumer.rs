//! Compile the same retained-handle orchestration on native and Wasm.
#![cfg(feature = "metrics")]

use ic_timers::{
    Error, Lifetime, Metrics, Observation, Once, OnceDemand, OnceNext, OnceResult, OnceStatus,
};
use std::{
    cell::Cell,
    future::{Ready, ready},
    rc::Rc,
};

///
/// Engine
/// Minimal consumer that retains one timer for repeated reconciliation.
///

#[derive(Default)]
struct Engine {
    timer: Option<Once>,
}

impl Engine {
    fn reconcile(
        &mut self,
        work: impl FnMut() -> Ready<OnceResult> + 'static,
    ) -> Result<(OnceStatus, Metrics), Error> {
        ic_timers::initialize()?;
        if self.timer.is_none() {
            self.timer = Some(ic_timers::bind_once(
                ["consumer", "engine", "retained"],
                Lifetime::Retained,
                work,
            )?);
        }
        let timer = self.timer.as_ref().expect("retained timer");
        timer.set(OnceDemand::At(0))?;

        Ok((timer.status()?, ic_timers::metrics()?))
    }
}

#[test]
fn native_runtime_refuses_before_admitting_work() {
    let starts = Rc::new(Cell::new(0));
    let callback = || {
        let starts = Rc::clone(&starts);
        move || {
            starts.set(starts.get() + 1);
            ready(OnceResult {
                observation: Observation::Success,
                next: OnceNext::Stop,
            })
        }
    };
    let mut engine = Engine::default();
    assert!(matches!(
        engine.reconcile(callback()),
        Err(Error::Uninitialized)
    ));
    assert!(engine.timer.is_none());
    assert!(matches!(
        ic_timers::bind_once(
            ["consumer", "engine", "retained"],
            Lifetime::Retained,
            callback()
        ),
        Err(Error::Uninitialized)
    ));
    assert!(matches!(ic_timers::metrics(), Err(Error::Uninitialized)));

    let storage = Rc::new(ic_timers::MetricsStorage::default());
    assert_eq!(
        ic_timers::initialize_with_metrics(Rc::clone(&storage)),
        Err(Error::Uninitialized)
    );
    assert_eq!(storage.snapshot(), Metrics::default());
    assert_eq!(starts.get(), 0);
}
