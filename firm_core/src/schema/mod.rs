use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use crate::{EntityType, FieldId, FieldType};

mod builtin;
mod validation;
mod validation_errors;

pub use validation::ValidationResult;
pub use validation_errors::{ValidationError, ValidationErrorType};

/// Defines the mode of a field, either required or optional
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldMode {
    Required,
    Optional,
}

/// Defines the schema for an unnamed field which can be either required or optional.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldSchema {
    pub field_type: FieldType,
    pub field_mode: FieldMode,
}

impl FieldSchema {
    pub fn new(field_type: FieldType, field_mode: FieldMode) -> Self {
        FieldSchema {
            field_type,
            field_mode,
        }
    }

    /// Get the expected field type.
    pub fn expected_type(&self) -> &FieldType {
        &self.field_type
    }

    /// Check if the field is required.
    pub fn is_required(&self) -> bool {
        self.field_mode == FieldMode::Required
    }
}

/// Defines the schema for an entity type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySchema {
    pub entity_type: EntityType,
    pub fields: HashMap<FieldId, FieldSchema>,
}

impl EntitySchema {
    /// Creates a new entity schema with a given name.
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity_type: entity_type,
            fields: HashMap::new(),
        }
    }

    /// Builder method to add a field to the schema.
    pub fn add_field_schema(mut self, id: FieldId, field_schema: FieldSchema) -> Self {
        self.fields.insert(id, field_schema);
        self
    }

    /// Builder method to add a required field to the schema.
    pub fn with_required_field(self, id: FieldId, field_type: FieldType) -> Self {
        self.add_field_schema(id, FieldSchema::new(field_type, FieldMode::Required))
    }

    /// Builder method to add an optional field to the schema.
    pub fn with_optional_field(self, id: FieldId, field_type: FieldType) -> Self {
        self.add_field_schema(id, FieldSchema::new(field_type, FieldMode::Optional))
    }

    /// Builder method to add common metadata fields to the schema.
    pub fn with_metadata(self) -> Self {
        self.with_optional_field(FieldId::new("created_at"), FieldType::DateTime)
            .with_optional_field(FieldId::new("updated_at"), FieldType::DateTime)
            .with_optional_field(FieldId::new("notes"), FieldType::String)
    }
}

impl Display for EntitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.entity_type)?;

        for (field_id, field_schema) in &self.fields {
            writeln!(f, "\n{}", field_id)?;
            writeln!(f, "- Type: {}", field_schema.expected_type())?;
            writeln!(f, "- Required: {}", field_schema.is_required())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_create_new() {
        let schema = EntitySchema::new(EntityType::new("person"))
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("email"), FieldType::String);

        assert_eq!(schema.entity_type, EntityType::new("person"));
        let name_field = &schema.fields[&FieldId::new("name")];
        assert_eq!(name_field.field_type, FieldType::String);
        assert_eq!(name_field.field_mode, FieldMode::Required);

        let email_field = &schema.fields[&FieldId::new("email")];
        assert_eq!(email_field.field_type, FieldType::String);
        assert_eq!(email_field.field_mode, FieldMode::Optional);
    }
}
