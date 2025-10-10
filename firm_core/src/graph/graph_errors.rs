use crate::{EntityId, FieldId};

#[derive(Debug, Clone)]
pub struct EntityAlreadyExistsError {
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QueryError {
    EntityNotFound(EntityId),
    FieldNotFound(EntityId, FieldId),
    CyclicReference,
    MaxDepthExceeded,
    NotAFieldReference,
    NotAnEntityReference,
    GraphNotBuilt,
}
