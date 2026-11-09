mod delivery;
mod store;

pub(crate) use delivery::{DeliveryFixture, Evidence};
pub(crate) use store::Store;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
