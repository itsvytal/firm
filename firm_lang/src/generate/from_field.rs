use firm_core::FieldValue;

use super::{GeneratorOptions, from_value};

/// Generate DSL for a field assignment
pub fn generate_field(
    field_name: &str,
    field_value: &FieldValue,
    options: &GeneratorOptions,
) -> String {
    let value_str = from_value::generate_value(field_value, options);
    format!("{} = {}", field_name, value_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use firm_core::FieldValue;

    #[test]
    fn test_generate_simple_field() {
        let options = GeneratorOptions::default();
        let result = generate_field("name", &FieldValue::String("test".to_string()), &options);
        assert_eq!(result, "name = \"test\"");
    }

    #[test]
    fn test_generate_boolean_field() {
        let options = GeneratorOptions::default();
        let result = generate_field("active", &FieldValue::Boolean(true), &options);
        assert_eq!(result, "active = true");
    }
}
