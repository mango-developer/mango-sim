//! Simulator plugin framework.

use std::{
    any::{Any, TypeId},
    sync::Arc,
};

use downcast_rs::{impl_downcast, DowncastSync};

use crate::{rand::GlobalRng, task::NodeId, time::TimeHandle, SimConfig};

/// Simulator
pub trait Simulator: Any + Send + Sync + DowncastSync {
    /// Create a new simulator.
    ///
    /// This will be called on the first access via [`simulator`].
    fn new(rand: &GlobalRng, time: &TimeHandle, config: &SimConfig) -> Self
    where
        Self: Sized;

    /// Create a node.
    fn create_node(&self, _id: NodeId) {}

    /// Reset a node.
    fn reset_node(&self, _id: NodeId) {}

    /// Delete a node.
    fn delete_node(&self, _id: NodeId) {}
}

impl_downcast!(sync Simulator);

/// Get the simulator.
pub fn simulator<S: Simulator>() -> Arc<S> {
    crate::context::current(|h| {
        let sims = h.sims.lock().unwrap();
        sims[&TypeId::of::<S>()]
            .clone()
            .downcast_arc()
            .ok()
            .unwrap()
    })
}

/// Get the node ID of current task.
pub fn node() -> NodeId {
    crate::context::current_node()
}
