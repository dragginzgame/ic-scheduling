mod maintenance;
mod notifications;
mod store;

pub(crate) use maintenance::{Maintenance, MaintenanceSnapshot, Wake};
pub(crate) use notifications::{Destination, Notification, NotificationSnapshot, Notifications};
pub(crate) use store::Store;

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
