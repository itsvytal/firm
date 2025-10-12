use firm_core::{Entity, decompose_entity_id};

use super::{GeneratorOptions, from_field, generator_options::FieldOrder};

/// Generate DSL for a single entity.
pub fn generate_entity(entity: &Entity, options: &GeneratorOptions) -> String {
    let mut output = String::new();
    let (_, entity_id) = decompose_entity_id(&entity.id.0);

    // Entity declaration and open block
    output.push_str(&format!(
        "{} {} {{\n",
        entity.entity_type.to_string().to_lowercase(),
        entity_id
    ));

    // Generate fields
    let field_lines = generate_entity_fields(entity, options);
    for field_line in field_lines {
        output.push_str(&field_line);
    }

    // Close entity block
    output.push_str("}\n");

    output
}

/// Generate DSL for all fields for an entity.
fn generate_entity_fields(entity: &Entity, options: &GeneratorOptions) -> Vec<String> {
    let mut fields: Vec<(String, &firm_core::FieldValue)> = entity
        .fields
        .iter()
        .map(|(field_id, field_value)| (field_id.0.clone(), field_value))
        .collect();

    // Apply field ordering
    apply_field_ordering(&mut fields, &options.field_order);

    // Generate each field
    fields
        .into_iter()
        .map(|(field_name, field_value)| {
            let field_line = from_field::generate_field(&field_name, field_value, options);
            format!("{}{}\n", options.indent_style.indent_string(1), field_line)
        })
        .collect()
}

/// Apply the specified field ordering strategy.
fn apply_field_ordering(fields: &mut Vec<(String, &firm_core::FieldValue)>, order: &FieldOrder) {
    match order {
        FieldOrder::Unordered => {}
        FieldOrder::Alphabetical => {
            fields.sort_by(|a, b| a.0.cmp(&b.0));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate::generator_options::IndentStyle;
    use firm_core::{Entity, EntityId, EntityType, FieldId, FieldValue, ReferenceValue};
    use std::collections::HashMap;

    #[test]
    fn test_generate_simple_person_entity() {
        let mut fields = HashMap::new();
        fields.insert(
            FieldId("name".to_string()),
            FieldValue::String("John Doe".to_string()),
        );
        fields.insert(FieldId("age".to_string()), FieldValue::Integer(42));

        let entity = Entity {
            id: EntityId("person.john_doe".to_string()),
            entity_type: EntityType::new("person"),
            fields,
        };

        let result = generate_entity(&entity, &GeneratorOptions::default());

        let expected = r#"person john_doe {
    age = 42
    name = "John Doe"
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_organization_with_multiple_fields() {
        let mut fields = HashMap::new();
        fields.insert(
            FieldId("name".to_string()),
            FieldValue::String("ACME Corp".to_string()),
        );
        fields.insert(
            FieldId("primary_email".to_string()),
            FieldValue::String("contact@acme.com".to_string()),
        );
        fields.insert(FieldId("active".to_string()), FieldValue::Boolean(true));
        fields.insert(
            FieldId("employee_count".to_string()),
            FieldValue::Integer(150),
        );

        let entity = Entity {
            id: EntityId("organization.acme_corp".to_string()),
            entity_type: EntityType::new("organization"),
            fields,
        };

        let result = generate_entity(&entity, &GeneratorOptions::default());

        let expected = r#"organization acme_corp {
    active = true
    employee_count = 150
    name = "ACME Corp"
    primary_email = "contact@acme.com"
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_entity_with_references() {
        let mut fields = HashMap::new();
        fields.insert(
            FieldId("name".to_string()),
            FieldValue::String("Jane Smith".to_string()),
        );
        fields.insert(
            FieldId("manager".to_string()),
            FieldValue::Reference(ReferenceValue::Entity(EntityId(
                "person.john_doe".to_string(),
            ))),
        );
        fields.insert(
            FieldId("manager_email".to_string()),
            FieldValue::Reference(ReferenceValue::Field(
                EntityId("person.john_doe".to_string()),
                FieldId("email".to_string()),
            )),
        );

        let entity = Entity {
            id: EntityId("person.jane_smith".to_string()),
            entity_type: EntityType::new("person"),
            fields,
        };

        let result = generate_entity(&entity, &GeneratorOptions::default());

        let expected = r#"person jane_smith {
    manager = person.john_doe
    manager_email = person.john_doe.email
    name = "Jane Smith"
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_entity_with_multiline_string() {
        let mut fields = HashMap::new();
        fields.insert(
            FieldId("title".to_string()),
            FieldValue::String("Code Review".to_string()),
        );
        fields.insert(
            FieldId("description".to_string()),
            FieldValue::String(
                "Review the pull request:\n- Check logic\n- Verify tests\n- Approve changes"
                    .to_string(),
            ),
        );

        let entity = Entity {
            id: EntityId("task.code_review".to_string()),
            entity_type: EntityType::new("task"),
            fields,
        };

        let result = generate_entity(&entity, &GeneratorOptions::default());

        let expected = r#"task code_review {
    description = """
    Review the pull request:
    - Check logic
    - Verify tests
    - Approve changes
    """
    title = "Code Review"
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_with_custom_indent() {
        let mut fields = HashMap::new();
        fields.insert(
            FieldId("name".to_string()),
            FieldValue::String("Test".to_string()),
        );

        let entity = Entity {
            id: EntityId("person.test".to_string()),
            entity_type: EntityType::new("person"),
            fields,
        };

        let options = GeneratorOptions {
            indent_style: IndentStyle::Spaces(2),
            ..Default::default()
        };

        let result = generate_entity(&entity, &options);

        let expected = r#"person test {
  name = "Test"
}
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generate_with_tab_indent() {
        let mut fields = HashMap::new();
        fields.insert(
            FieldId("name".to_string()),
            FieldValue::String("Test".to_string()),
        );

        let entity = Entity {
            id: EntityId("person.test".to_string()),
            entity_type: EntityType::new("person"),
            fields,
        };

        let options = GeneratorOptions {
            indent_style: IndentStyle::Tabs,
            ..Default::default()
        };

        let result = generate_entity(&entity, &options);

        let expected = "person test {\n\tname = \"Test\"\n}\n";
        assert_eq!(result, expected);
    }
}
