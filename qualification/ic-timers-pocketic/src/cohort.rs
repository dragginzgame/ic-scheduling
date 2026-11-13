//! One cohort catalog owns selection, measurement slots and receipt field names.

use crate::report::{CalibrationMetrics, PolicyMetrics};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

// Generate the complete typed collection directly, retaining Serde's strict
// record decoding and the existing flat receipt shape (including duplicate checks).
macro_rules! cohorts {
    ($($(#[$doc:meta])* $variant:ident => $field:ident: $name:literal, $environment:literal;)+) => {
        /// Policy cohorts admitted by the build driver and measurement consumers.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Cohort { $($(#[$doc])* $variant,)+ }

        impl Cohort {
            /// Canonical execution order shared by the driver and async measurements.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Stable label used in receipt fields and build selections.
            #[must_use]
            pub const fn name(self) -> &'static str {
                match self { $(Self::$variant => $name,)+ }
            }

            /// Environment variable carrying this cohort's exact artifact path.
            #[must_use]
            pub const fn environment(self) -> &'static str {
                match self { $(Self::$variant => $environment,)+ }
            }
        }

        /// Complete policy measurements indexed by cohort, plus calibration.
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Measurements {
            $(#[serde(rename = $name)] $field: PolicyMetrics,)+
            /// Watchdog calibration with minimal application work.
            #[serde(rename = "watchdog-calibration-minimal")]
            pub minimal: CalibrationMetrics,
            /// Watchdog calibration with representative application work.
            #[serde(rename = "watchdog-calibration-representative")]
            pub representative: CalibrationMetrics,
        }

        impl Measurements {
            /// Measure every policy in canonical order and initialize calibration slots.
            #[must_use]
            pub fn from_fn(mut measure: impl FnMut(Cohort) -> PolicyMetrics) -> Self {
                Self {
                    $($field: measure(Cohort::$variant),)+
                    minimal: CalibrationMetrics::default(),
                    representative: CalibrationMetrics::default(),
                }
            }
        }

        impl Index<Cohort> for Measurements {
            type Output = PolicyMetrics;
            fn index(&self, cohort: Cohort) -> &Self::Output {
                match cohort { $(Cohort::$variant => &self.$field,)+ }
            }
        }

        impl IndexMut<Cohort> for Measurements {
            fn index_mut(&mut self, cohort: Cohort) -> &mut Self::Output {
                match cohort { $(Cohort::$variant => &mut self.$field,)+ }
            }
        }
    };
}

cohorts! {
    /// Runtime/protocol baseline with no timer registration.
    Baseline => baseline: "baseline", "IC_TIMERS_COHORT_BASELINE_WASM";
    /// One-shot ordinary timer policy.
    Once => once: "once", "IC_TIMERS_COHORT_ONCE_WASM";
    /// Ordinary timer recurrence relative to completion.
    AfterCompletion => after_completion: "after-completion", "IC_TIMERS_COHORT_AFTER_COMPLETION_WASM";
    /// Synchronous watchdog scheduler and dispatched work.
    Watchdog => watchdog: "watchdog", "IC_TIMERS_COHORT_WATCHDOG_WASM";
}
