use log::debug;

use super::{EntitySchema, ValidationError};
use crate::Entity;

pub type ValidationResult = Result<(), Vec<ValidationError>>;

impl EntitySchema {
    /// Validates an entity against the schema.
    pub fn validate(&self, entity: &Entity) -> ValidationResult {
        debug!(
            "Validating entity: '{}' for schema: '{}'",
            entity.id, self.entity_type
        );

        let mut errors = Vec::new();

        // Check the entity type against the schema
        if entity.entity_type != self.entity_type {
            errors.push(ValidationError::mismatched_entity_type(
                &entity.id,
                &self.entity_type,
                &entity.entity_type,
            ))
        }

        // Check each field in the schema
        for (field_name, field_schema) in &self.fields {
            match entity.fields.get(field_name) {
                // Entity has the field: Check that it has desired type
                Some(field_value) => {
                    let expected_type = field_schema.expected_type();
                    if !field_value.is_type(expected_type) {
                        errors.push(ValidationError::mismatched_field_type(
                            &entity.id,
                            field_name,
                            expected_type,
                            &field_value.get_type(),
                        ));
                    }
                }
                // Entity does not have the field: Check if it's required
                None => {
                    if field_schema.is_required() {
                        errors.push(ValidationError::missing_field(&entity.id, field_name));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            debug!(
                "Entity '{}' failed validation with {} errors",
                entity.id,
                errors.len()
            );
            Err(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::ValidationErrorType;
    use crate::{
        EntityId, EntityType, FieldId,
        field::{FieldType, FieldValue},
    };
    use assert_matches::assert_matches;

    #[test]
    fn test_validate_ok() {
        let schema = EntitySchema::new(EntityType::new("person"))
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("email"), FieldType::String);

        let entity = Entity::new(EntityId::new("test_person"), EntityType::new("person"))
            .with_field(
                FieldId::new("name"),
                FieldValue::String(String::from("John Doe")),
            );

        let result = schema.validate(&entity);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_error_mismatched_entity_types() {
        let schema = EntitySchema::new(EntityType::new("test_a"));
        let entity = Entity::new(EntityId::new("test"), EntityType::new("test_b"));

        let result = schema.validate(&entity);

        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);

        assert_matches!(
            &errors[0].error_type,
            ValidationErrorType::MismatchedEntityType { expected, actual } if expected == &EntityType::new("test_a") && actual == &EntityType::new("test_b")
        );
    }

    #[test]
    fn test_validate_error_missing_field() {
        let schema = EntitySchema::new(EntityType::new("person"))
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("email"), FieldType::String);

        let entity = Entity::new(EntityId::new("test_person"), EntityType::new("person"))
            .with_field(
                FieldId::new("name"),
                FieldValue::String(String::from("John Doe")),
            );

        let result = schema.validate(&entity);

        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);

        assert_matches!(
            &errors[0].error_type,
            ValidationErrorType::MissingRequiredField { required } if required == &FieldId::new("email")
        );
    }

    #[test]
    fn test_validate_error_mismatched_field_types() {
        let schema = EntitySchema::new(EntityType::new("person"))
            .with_required_field(FieldId::new("is_nice"), FieldType::Boolean);

        let entity = Entity::new(EntityId::new("test_person"), EntityType::new("person"))
            .with_field(
                FieldId::new("is_nice"),
                FieldValue::String("Sure".to_string()),
            );

        let result = schema.validate(&entity);

        assert!(result.is_err());

        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);

        assert_matches!(
            &errors[0].error_type,
            ValidationErrorType::MismatchedFieldType { expected, actual } if expected == &FieldType::Boolean && actual == &FieldType::String
        );
    }
}
