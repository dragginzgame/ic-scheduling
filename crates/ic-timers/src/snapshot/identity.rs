//! Bounded, deterministically ordered timer identity.

use std::{fmt, sync::Arc};
use thiserror::Error;

/// Maximum encoded length of one timer identity component.
///
/// The limit is measured in UTF-8 bytes so storage and metrics cardinality do
/// not depend on Unicode scalar width.
pub const MAX_TIMER_IDENTITY_COMPONENT_BYTES: usize = 64;

/// The identity component whose validation failed.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TimerIdentityField {
    /// Scheduling owner, such as a framework or application.
    Owner,
    /// Functional area within the owner.
    Subsystem,
    /// Stable logical timer name.
    Name,
}

impl fmt::Display for TimerIdentityField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Owner => "owner",
            Self::Subsystem => "subsystem",
            Self::Name => "name",
        })
    }
}

/// Invalid structured timer identity.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum TimerIdentityError {
    /// One identity component was empty.
    #[error("invalid timer {field}: identity component must not be empty")]
    Empty {
        /// Identity component that failed validation.
        field: TimerIdentityField,
    },
    /// One identity component exceeded the UTF-8 byte bound.
    #[error(
        "invalid timer {field}: identity component is {actual_bytes} bytes; maximum is {max_bytes}"
    )]
    TooLong {
        /// Identity component that failed validation.
        field: TimerIdentityField,
        /// Encoded length of the rejected value.
        actual_bytes: usize,
        /// Maximum supported encoded length.
        max_bytes: usize,
    },
    /// One identity component had leading or trailing whitespace.
    #[error("invalid timer {field}: identity component has leading or trailing whitespace")]
    SurroundingWhitespace {
        /// Identity component that failed validation.
        field: TimerIdentityField,
    },
    /// One identity component contained a control character.
    #[error(
        "invalid timer {field}: identity component contains control character {character:?} at byte {byte_index}"
    )]
    ControlCharacter {
        /// Identity component that failed validation.
        field: TimerIdentityField,
        /// UTF-8 byte offset of the rejected character.
        byte_index: usize,
        /// Rejected control character.
        character: char,
    },
}

/// Stable, low-cardinality identity for one logical canister timer.
///
/// Derived ordering is lexicographic by owner, subsystem, then name.
/// Clones share immutable component storage and retain value-based equality.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimerIdentity {
    components: Arc<IdentityComponents>,
}

#[derive(Eq, Hash, Ord, PartialEq, PartialOrd)]
struct IdentityComponents {
    owner: Box<str>,
    subsystem: Box<str>,
    name: Box<str>,
}

impl TimerIdentity {
    /// Validate all three identity components.
    ///
    /// # Errors
    ///
    /// Returns [`TimerIdentityError`] when any component is empty, exceeds
    /// the byte limit, has surrounding whitespace, or contains a control character.
    pub fn try_new(
        owner: impl AsRef<str>,
        subsystem: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> Result<Self, TimerIdentityError> {
        Ok(Self {
            components: Arc::new(IdentityComponents {
                owner: validate_component(TimerIdentityField::Owner, owner)?,
                subsystem: validate_component(TimerIdentityField::Subsystem, subsystem)?,
                name: validate_component(TimerIdentityField::Name, name)?,
            }),
        })
    }

    /// Return the scheduling owner.
    #[must_use]
    pub fn owner(&self) -> &str {
        &self.components.owner
    }

    /// Return the functional subsystem.
    #[must_use]
    pub fn subsystem(&self) -> &str {
        &self.components.subsystem
    }

    /// Return the stable logical timer name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.components.name
    }
}

impl fmt::Debug for TimerIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TimerIdentity")
            .field("owner", &self.owner())
            .field("subsystem", &self.subsystem())
            .field("name", &self.name())
            .finish()
    }
}

fn validate_component(
    field: TimerIdentityField,
    value: impl AsRef<str>,
) -> Result<Box<str>, TimerIdentityError> {
    let value = value.as_ref();
    if value.is_empty() {
        return Err(TimerIdentityError::Empty { field });
    }
    if value.len() > MAX_TIMER_IDENTITY_COMPONENT_BYTES {
        return Err(TimerIdentityError::TooLong {
            field,
            actual_bytes: value.len(),
            max_bytes: MAX_TIMER_IDENTITY_COMPONENT_BYTES,
        });
    }
    if value.trim() != value {
        return Err(TimerIdentityError::SurroundingWhitespace { field });
    }
    if let Some((byte_index, character)) = value.char_indices().find(|(_, ch)| ch.is_control()) {
        return Err(TimerIdentityError::ControlCharacter {
            field,
            byte_index,
            character,
        });
    }

    Ok(value.into())
}
