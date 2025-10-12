//! Core data structures and graph operations for Firm.
//!
//! This crate provides the fundamental building blocks for managing
//! business entities, their associated data and their relationships.

pub mod entity;
pub mod field;
pub mod graph;
pub mod id;
pub mod schema;

pub use entity::Entity;
pub use field::{FieldType, FieldValue, ReferenceValue};
pub use id::{EntityId, EntityType, FieldId, compose_entity_id, decompose_entity_id};
pub use schema::EntitySchema;
