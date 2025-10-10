use std::fmt;

/// Represents a problem while converting a ParsedEntity.
#[derive(Debug)]
pub enum EntityConversionError {
    MissingEntityType,
    MissingEntityId,
    MissingFieldId,
    InvalidFieldValue,
}

impl fmt::Display for EntityConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EntityConversionError::MissingEntityType => {
                write!(f, "Entity is missing required type")
            }
            EntityConversionError::MissingEntityId => {
                write!(f, "Entity is missing required id")
            }
            EntityConversionError::MissingFieldId => {
                write!(f, "Entity field is missing required id")
            }
            EntityConversionError::InvalidFieldValue => {
                write!(f, "Entity field contains an invalid value")
            }
        }
    }
}

/// Represents a problem converting a ParsedSchema.
#[derive(Debug)]
pub enum SchemaConversionError {
    MissingSchemaName,
    MissingFieldName,
    MissingFieldType,
    UnknownFieldType(String),
    InvalidFieldDefinition,
}

impl fmt::Display for SchemaConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaConversionError::MissingSchemaName => {
                write!(f, "Schema is missing required name")
            }
            SchemaConversionError::MissingFieldName => {
                write!(f, "Schema field is missing required name")
            }
            SchemaConversionError::MissingFieldType => {
                write!(f, "Schema field is missing required type")
            }
            SchemaConversionError::UnknownFieldType(field_type) => {
                write!(f, "Unknown field type: '{}'", field_type)
            }
            SchemaConversionError::InvalidFieldDefinition => {
                write!(f, "Schema field definition is invalid")
            }
        }
    }
}
