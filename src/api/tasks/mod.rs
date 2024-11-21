//! Represent tasks.
//! Notice this module only provides a database layer.

use crate::api::common::o2o;

pub mod data;
pub mod tasks;
pub mod manager;

#[flutter_rust_bridge::frb(non_opaque)]
/// A union task struct.
#[derive(o2o::o2o)]
#[map_owned(wlist_native::tasks::Task)]
pub enum FTask {
    /// Represents a refresh task.
    Refresh(#[map(o2o::map(~))] tasks::FRefreshTask),

}