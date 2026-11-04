//! Deterministic native provider and provider-boundary tests.

use super::*;
use std::{future::Future, time::Duration};

pub(crate) use fake::{
    TimerHandle, advance_instructions, canister_version, clear_tasks, clear_timer,
    discard_next_due, discard_timer_at, grow_memory_pages, instruction_counter, memory_pages,
    reset, run_next_due, set_time, set_timer, time_ns, timer_count, trap,
};

pub(crate) fn try_set_timer(
    delay: Duration,
    task: impl Future<Output = ()> + 'static,
) -> Result<TimerHandle, AdmissionError> {
    if let Some((remaining, error)) = ADMISSION_FAULT.take() {
        if remaining == 0 {
            return Err(error);
        }
        ADMISSION_FAULT.set(Some((remaining - 1, error)));
    }
    queue::deadline(time_ns(), delay)?;
    Ok(set_timer(delay, task))
}

thread_local! {
    pub(crate) static ADMISSION_FAULT: std::cell::Cell<Option<(usize, AdmissionError)>> = const { std::cell::Cell::new(None) };
}

mod fake {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        collections::BTreeMap,
        pin::Pin,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
        task::{Context, Poll, Wake, Waker},
    };

    type Task = Pin<Box<dyn Future<Output = ()>>>;

    struct ScheduledTask {
        deadline_ns: u64,
        task: Task,
        started: bool,
        ready: Arc<TaskReady>,
    }

    struct TaskReady(AtomicBool);

    impl Wake for TaskReady {
        fn wake(self: Arc<Self>) {
            self.wake_by_ref();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.0.store(true, Ordering::Relaxed);
        }
    }

    #[must_use = "bind or clear the platform timer handle"]
    #[derive(Debug, Eq, PartialEq)]
    pub(crate) struct TimerHandle(u64);

    thread_local! {
        static NOW_NS: Cell<u64> = const { Cell::new(0) };
        static CANISTER_VERSION: Cell<u64> = const { Cell::new(0) };
        static NEXT_HANDLE: Cell<u64> = const { Cell::new(0) };
        static INSTRUCTIONS: Cell<u64> = const { Cell::new(0) };
        static WASM_MEMORY_PAGES: Cell<u64> = const { Cell::new(1) };
        static STABLE_MEMORY_PAGES: Cell<u64> = const { Cell::new(0) };
        static TASKS: RefCell<BTreeMap<u64, ScheduledTask>> = const {
            RefCell::new(BTreeMap::new())
        };
    }

    pub(crate) fn set_timer(
        delay: Duration,
        task: impl Future<Output = ()> + 'static,
    ) -> TimerHandle {
        advance_instructions(5);
        let delay_ns = u64::try_from(delay.as_nanos()).unwrap_or(u64::MAX);
        let deadline_ns = time_ns().saturating_add(delay_ns);
        let handle = NEXT_HANDLE.with(|next| {
            let handle = next.get().saturating_add(1);
            next.set(handle);
            handle
        });
        TASKS.with(|tasks| {
            tasks.borrow_mut().insert(
                handle,
                ScheduledTask {
                    deadline_ns,
                    task: Box::pin(task),
                    started: false,
                    ready: Arc::new(TaskReady(AtomicBool::new(false))),
                },
            );
        });
        TimerHandle(handle)
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "Clearing consumes the linear handle."
    )]
    pub(crate) fn clear_timer(handle: TimerHandle) {
        advance_instructions(2);
        let removed = TASKS.with(|tasks| tasks.borrow_mut().remove(&handle.0));
        // Match provider removal: consumer future destructors run after the
        // task-map borrow ends and may inspect or schedule other timers.
        drop(removed);
    }

    pub(crate) fn time_ns() -> u64 {
        NOW_NS.with(Cell::get)
    }

    pub(crate) fn canister_version() -> u64 {
        CANISTER_VERSION.with(Cell::get)
    }

    pub(crate) fn instruction_counter() -> u64 {
        INSTRUCTIONS.with(Cell::get)
    }

    pub(crate) fn memory_pages() -> MemoryPageExtent {
        MemoryPageExtent::new(
            WASM_MEMORY_PAGES.with(Cell::get),
            STABLE_MEMORY_PAGES.with(Cell::get),
        )
    }

    pub(crate) fn trap(message: &str) -> ! {
        panic!("{message}")
    }

    pub(crate) fn advance_instructions(amount: u64) {
        INSTRUCTIONS.with(|instructions| {
            instructions.set(instructions.get().saturating_add(amount));
        });
    }

    pub(crate) fn grow_memory_pages(wasm: u64, stable: u64) {
        WASM_MEMORY_PAGES.with(|pages| pages.set(pages.get().saturating_add(wasm)));
        STABLE_MEMORY_PAGES.with(|pages| pages.set(pages.get().saturating_add(stable)));
    }

    pub(crate) fn reset(now_ns: u64, canister_version: u64) {
        NOW_NS.with(|now| now.set(now_ns));
        CANISTER_VERSION.with(|version| version.set(canister_version));
        NEXT_HANDLE.with(|next| next.set(0));
        INSTRUCTIONS.with(|instructions| instructions.set(0));
        WASM_MEMORY_PAGES.with(|pages| pages.set(1));
        STABLE_MEMORY_PAGES.with(|pages| pages.set(0));
        clear_tasks();
    }

    /// Drop queued futures before native TLS teardown, with no task-map borrow.
    pub(crate) fn clear_tasks() {
        let tasks = TASKS.with(|tasks| std::mem::take(&mut *tasks.borrow_mut()));
        drop(tasks);
    }

    pub(crate) fn set_time(now_ns: u64) {
        NOW_NS.with(|now| now.set(now_ns));
    }

    pub(crate) fn timer_count() -> usize {
        TASKS.with(|tasks| tasks.borrow().values().filter(|task| !task.started).count())
    }

    pub(crate) fn run_next_due() -> bool {
        let now_ns = time_ns();
        let next_handle = next_due_handle(now_ns);
        let Some(next_handle) = next_handle else {
            return false;
        };
        let Some(mut scheduled) = TASKS.with(|tasks| tasks.borrow_mut().remove(&next_handle))
        else {
            return false;
        };

        scheduled.started = true;
        scheduled.ready.0.store(false, Ordering::Relaxed);
        let waker = Waker::from(Arc::clone(&scheduled.ready));
        let mut context = Context::from_waker(&waker);
        if matches!(scheduled.task.as_mut().poll(&mut context), Poll::Pending) {
            TASKS.with(|tasks| {
                tasks.borrow_mut().insert(next_handle, scheduled);
            });
        }
        true
    }

    pub(crate) fn discard_next_due() -> bool {
        let Some(next_handle) = next_due_handle(time_ns()) else {
            return false;
        };
        let removed = TASKS.with(|tasks| tasks.borrow_mut().remove(&next_handle));
        removed.is_some()
    }

    pub(crate) fn discard_timer_at(deadline_ns: u64) -> bool {
        let handle = TASKS.with_borrow(|tasks| {
            tasks
                .iter()
                .find(|(_, task)| !task.started && task.deadline_ns == deadline_ns)
                .map(|(handle, _)| *handle)
        });
        let Some(handle) = handle else {
            return false;
        };
        let removed = TASKS.with_borrow_mut(|tasks| tasks.remove(&handle));
        removed.is_some()
    }

    fn next_due_handle(now_ns: u64) -> Option<u64> {
        TASKS.with(|tasks| {
            tasks
                .borrow()
                .iter()
                .filter(|(_, scheduled)| {
                    if scheduled.started {
                        scheduled.ready.0.load(Ordering::Relaxed)
                    } else {
                        scheduled.deadline_ns <= now_ns
                    }
                })
                .min_by_key(|(handle, scheduled)| (scheduled.deadline_ns, **handle))
                .map(|(handle, _)| *handle)
        })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::rc::Rc;

        struct ScheduleOnDrop(Rc<Cell<bool>>);

        impl Drop for ScheduleOnDrop {
            fn drop(&mut self) {
                assert_eq!(timer_count(), 0);
                let _handle = set_timer(Duration::ZERO, async {});
                self.0.set(true);
            }
        }

        #[test]
        fn removed_futures_can_schedule_from_their_destructors() {
            for (discard, started) in [(false, false), (false, true), (true, false)] {
                reset(0, 0);
                let dropped = Rc::new(Cell::new(false));
                let captured = ScheduleOnDrop(Rc::clone(&dropped));
                let handle = set_timer(Duration::ZERO, async move {
                    std::future::pending::<()>().await;
                    drop(captured);
                });
                if started {
                    assert!(run_next_due());
                }
                if discard {
                    assert!(discard_next_due());
                } else {
                    clear_timer(handle);
                }
                assert!(dropped.get());
                assert_eq!(timer_count(), 1);
                assert!(run_next_due());
                assert_eq!(timer_count(), 0);
                clear_tasks();
            }
        }
    }
}
