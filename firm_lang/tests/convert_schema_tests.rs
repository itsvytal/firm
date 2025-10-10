use firm_core::{
    FieldId,
    EntityType,
    field::FieldType,
    schema::{EntitySchema, FieldSchema},
};

use firm_lang::{convert::SchemaConversionError, parser::parse_source};

#[test]
fn test_convert_simple_schema() {
    let source = r#"
        schema project {
            field {
                name = "title"
                type = "string"
                required = true
            }

            field {
                name = "priority"
                type = "integer"
                required = false
            }
        }
    "#;

    let parsed = parse_source(String::from(source)).unwrap();
    let schemas = parsed.schemas();
    assert_eq!(schemas.len(), 1);

    let schema: EntitySchema = (&schemas[0]).try_into().unwrap();

    assert_eq!(
        schema.entity_type,
        EntityType::new("project")
    );
    assert_eq!(schema.fields.len(), 2);

    // Check required field
    let title_field = &schema.fields[&FieldId("title".to_string())];
    assert_eq!(*title_field, FieldSchema::Required(FieldType::String));

    // Check optional field
    let priority_field = &schema.fields[&FieldId("priority".to_string())];
    assert_eq!(*priority_field, FieldSchema::Optional(FieldType::Integer));
}

#[test]
fn test_convert_schema_with_various_types() {
    let source = r#"
        schema invoice {
            field {
                name = "amount"
                type = "currency"
                required = true
            }

            field {
                name = "paid"
                type = "boolean"
                required = false
            }

            field {
                name = "due_date"
                type = "datetime"
                required = true
            }
        }
    "#;

    let parsed = parse_source(String::from(source)).unwrap();
    let schemas = parsed.schemas();
    assert_eq!(schemas.len(), 1);

    let schema: EntitySchema = (&schemas[0]).try_into().unwrap();

    assert_eq!(
        schema.entity_type,
        EntityType::new("invoice")
    );
    assert_eq!(schema.fields.len(), 3);

    // Check currency field
    let amount_field = &schema.fields[&FieldId("amount".to_string())];
    assert_eq!(*amount_field, FieldSchema::Required(FieldType::Currency));

    // Check boolean field
    let paid_field = &schema.fields[&FieldId("paid".to_string())];
    assert_eq!(*paid_field, FieldSchema::Optional(FieldType::Boolean));

    // Check datetime field
    let due_date_field = &schema.fields[&FieldId("due_date".to_string())];
    assert_eq!(*due_date_field, FieldSchema::Required(FieldType::DateTime));
}

#[test]
fn test_unknown_field_type_error() {
    let source = r#"
        schema test {
            field {
                name = "custom"
                type = "unknown_type"
                required = true
            }
        }
    "#;

    let parsed = parse_source(String::from(source)).unwrap();
    let schemas = parsed.schemas();
    assert_eq!(schemas.len(), 1);

    let result: Result<EntitySchema, SchemaConversionError> = (&schemas[0]).try_into();
    assert!(matches!(
        result,
        Err(SchemaConversionError::UnknownFieldType(_))
    ));
}

#[test]
fn test_convert_multiple_schemas() {
    let source = r#"
        schema project {
            field {
                name = "title"
                type = "string"
                required = true
            }
        }

        schema invoice {
            field {
                name = "amount"
                type = "currency"
                required = true
            }
        }
    "#;

    let parsed = parse_source(String::from(source)).unwrap();
    let schemas = parsed.schemas();
    assert_eq!(schemas.len(), 2);

    // Convert both schemas
    let project_schema: EntitySchema = (&schemas[0]).try_into().unwrap();
    let invoice_schema: EntitySchema = (&schemas[1]).try_into().unwrap();

    assert_eq!(
        project_schema.entity_type,
        EntityType::new("project")
    );
    assert_eq!(
        invoice_schema.entity_type,
        EntityType::new("invoice")
    );

    assert_eq!(project_schema.fields.len(), 1);
    assert_eq!(invoice_schema.fields.len(), 1);
}
