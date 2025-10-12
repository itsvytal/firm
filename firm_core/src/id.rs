use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Creates a typed identifier based on an underlying string.
/// This helps differentiate identifiers so that they are not accidentally mixed.
/// By convention, we convert the underlying value to snake_case.
macro_rules! typed_string_id {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
        pub struct $name(pub String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into().to_case(Case::Snake))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

typed_string_id!(EntityId);
typed_string_id!(FieldId);
typed_string_id!(EntityType);

/// Creates a standard composite Entity ID from the entity type and ID.
/// This allows entities of different types to share the same ID.
pub fn compose_entity_id(entity_type: &str, entity_id: &str) -> EntityId {
    EntityId::new(format!(
        "{}.{}",
        entity_type.to_string().to_lowercase(),
        entity_id
    ))
}

/// Decomposes a standard composite Entity ID into its entity type and ID components.
pub fn decompose_entity_id(composite_id: &str) -> (&str, &str) {
    composite_id
        .split_once('.')
        .unwrap_or(("unknown", composite_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preserves_snake_case() {
        let id = EntityId::new("john_doe");
        assert_eq!(id.to_string(), "john_doe");
    }

    #[test]
    fn test_preserves_period() {
        let id = EntityId::new("person.john_doe");
        assert_eq!(id.to_string(), "person.john_doe");
    }

    #[test]
    fn test_converts_to_snake_case() {
        let sentence_case_id = EntityId::new("John Doe");
        assert_eq!(sentence_case_id.to_string(), "john_doe");

        let pascal_case_id = EntityId::new("JohnDoe");
        assert_eq!(pascal_case_id.to_string(), "john_doe");

        let camel_case_id = EntityId::new("johnDoe");
        assert_eq!(camel_case_id.to_string(), "john_doe");
    }

    #[test]
    fn test_preserves_period_when_converted_to_snake_case() {
        let sentence_case_id = EntityId::new("Person.John Doe");
        assert_eq!(sentence_case_id.to_string(), "person.john_doe");

        let pascal_case_id = EntityId::new("Person.JohnDoe");
        assert_eq!(pascal_case_id.to_string(), "person.john_doe");

        let camel_case_id = EntityId::new("person.johnDoe");
        assert_eq!(camel_case_id.to_string(), "person.john_doe");
    }
}
