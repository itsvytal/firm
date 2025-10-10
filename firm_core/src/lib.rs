//! Core data structures and graph operations for Firm.
//!
//! This crate provides the fundamental building blocks for managing
//! business entities and their relationships in a graph structure.

pub mod entity;
pub mod field;
pub mod graph;
pub mod id;
pub mod schema;

pub use entity::{Entity};
pub use field::{FieldType, FieldValue, ReferenceValue};
pub use id::{EntityId, FieldId, EntityType, decompose_entity_id, make_composite_entity_id};
pub use schema::EntitySchema;
