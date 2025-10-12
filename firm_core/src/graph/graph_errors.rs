use crate::{EntityId, FieldId};

/// The types of errors you can get when interacting with the graph.
#[derive(Debug, Clone, PartialEq)]
pub enum GraphError {
    EntityAlreadyExists(EntityId),
    EntityNotFound(EntityId),
    FieldNotFound(EntityId, FieldId),
    CyclicReference,
    MaxDepthExceeded,
    NotAFieldReference,
    NotAnEntityReference,
    GraphNotBuilt,
}
