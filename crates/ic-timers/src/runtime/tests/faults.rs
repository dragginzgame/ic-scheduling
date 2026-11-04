//! Native scenario fault selection; the runtime/platform retain injection mechanics.

use super::{PROVIDER_INSTALL_FAULT_AFTER, inject_provider_install_fault_after, platform};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ProviderFault {
    None,
    Admission {
        after: usize,
        error: platform::AdmissionError,
    },
    Binding {
        after: u64,
    },
}

impl ProviderFault {
    pub(super) const ADMISSION: Self = Self::Admission {
        after: 0,
        error: platform::AdmissionError::Capacity,
    };
    pub(super) const BINDING: Self = Self::Binding { after: 0 };

    pub(super) fn inject(self) {
        match self {
            Self::None => {}
            Self::Admission { after, error } => platform::ADMISSION_FAULT.set(Some((after, error))),
            Self::Binding { after } => inject_provider_install_fault_after(after),
        }
    }

    pub(super) fn clear() {
        platform::ADMISSION_FAULT.set(None);
        PROVIDER_INSTALL_FAULT_AFTER.with(|fault| fault.set(None));
    }
}
