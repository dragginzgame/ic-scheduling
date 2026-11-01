//! One cohort catalog owns selection, measurement slots and receipt field names.

use crate::report::{CalibrationMetrics, PolicyMetrics};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

// Generate the complete typed collection directly, retaining Serde's strict
// record decoding and the existing flat receipt shape (including duplicate checks).
macro_rules! cohorts {
    ($($variant:ident => $field:ident: $name:literal, $environment:literal;)+) => {
        /// Policy cohorts admitted by the build driver and measurement consumers.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum Cohort { $($variant,)+ }

        impl Cohort {
            pub(crate) const ALL: &'static [Self] = &[$(Self::$variant,)+];

            pub const fn name(self) -> &'static str {
                match self { $(Self::$variant => $name,)+ }
            }

            pub const fn environment(self) -> &'static str {
                match self { $(Self::$variant => $environment,)+ }
            }
        }

        /// Complete policy measurements indexed by cohort, plus calibration.
        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Measurements {
            $(#[serde(rename = $name)] $field: PolicyMetrics,)+
            #[serde(rename = "watchdog-calibration-minimal")]
            pub minimal: CalibrationMetrics,
            #[serde(rename = "watchdog-calibration-representative")]
            pub representative: CalibrationMetrics,
        }

        impl Measurements {
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
    Baseline => baseline: "baseline", "IC_TIMERS_COHORT_BASELINE_WASM";
    Once => once: "once", "IC_TIMERS_COHORT_ONCE_WASM";
    AfterCompletion => after_completion: "after-completion", "IC_TIMERS_COHORT_AFTER_COMPLETION_WASM";
    Watchdog => watchdog: "watchdog", "IC_TIMERS_COHORT_WATCHDOG_WASM";
}
