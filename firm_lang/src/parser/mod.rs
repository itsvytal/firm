//! # Parser Module
//!
//! Core parsing functionality for the Firm DSL.
//!
//! This module provides tree-sitter-based parsing capabilities for converting Firm DSL source
//! code into structured, queryable data. It handles entity extraction, field parsing, and
//! comprehensive value type conversion.
//!
//! ## Main Components
//!
//! - [`parse_source`] - Entry point for parsing Firm DSL source code
//! - [`ParsedSource`] - Represents a fully parsed Firm document
//! - [`ParsedEntity`] - Individual entity definitions with type and ID
//! - [`ParsedSchema`] - Schema definitions with field specifications
//! - [`ParsedField`] - Entity fields with parsed values
//! - [`ParsedSchemaField`] - Schema field definitions with name, type, and requirements
//! - [`ParsedValue`] - Strongly-typed values supporting all Firm data types
//!
//! ## Error Handling
//!
//! The parser handles errors at multiple levels:
//!
//! - **Syntax Errors**: Detected by tree-sitter and accessible via `ParsedSource::has_error()`
//! - **Value Parse Errors**: Returned when converting raw text to typed values
//! - **Language Errors**: Issues with parser initialization or language compatibility
//!
//! ## Supported Value Types
//!
//! The parser supports comprehensive value types including primitives (strings, numbers, booleans),
//! structured types (currency, references, lists), and temporal types (dates, datetimes).
//! See [`ParsedValue`] for the complete list of supported types.

mod parsed_entity;
mod parsed_field;
mod parsed_schema;
mod parsed_schema_field;
mod parsed_source;
mod parsed_value;
mod parser_errors;
mod parser_utils;
mod source;

pub use parsed_entity::ParsedEntity;
pub use parsed_field::ParsedField;
pub use parsed_schema::ParsedSchema;
pub use parsed_schema_field::ParsedSchemaField;
pub use parsed_source::ParsedSource;
pub use parsed_value::ParsedValue;
pub use parser_errors::{LanguageError, ValueParseError};
pub use source::parse_source;
