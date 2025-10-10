//! # Firm Language Parser
//!
//! DSL parsing and generation for Firm business entities.
//!
//! This crate provides parsing and generation capabilities for the Firm DSL, a declarative syntax
//! for defining business relationships that compiles to a queryable graph. The grammar is defined
//! in [tree-sitter-firm](https://github.com/42futures/tree-sitter-firm), and this library handles
//! the semantic layer for converting between Firm source code and structured entity models.
//!
//! ## Features
//!
//! - **Syntax Parsing**: Tree-sitter-based parsing with comprehensive error detection
//! - **Entity Extraction**: Extract entity definitions with type and ID information
//! - **Field Parsing**: Parse entity fields with full value extraction and type support
//! - **Value Types**: Support for all major data types including strings, numbers, booleans,
//!   currency, references, lists, dates, and datetimes
//!
//! ## Supported Value Types
//!
//! The parser supports a comprehensive set of value types:
//!
//! - **Strings**: Single-line (`"text"`) and multi-line (`"""text"""`)
//! - **Numbers**: Integers (`42`) and floats (`42.5`)
//! - **Booleans**: `true` and `false`
//! - **Currency**: Amount with currency code (`100.50 USD`)
//! - **References**: Entity (`contact.john`) and field (`contact.john.name`)
//! - **Lists**: Arrays of values (`["item1", "item2", 42]`)
//! - **Dates**: ISO format (`2024-03-20`)
//! - **DateTimes**: With optional timezone (`2024-03-20 at 14:30 UTC-5`)
//!
//! ## Error Handling
//!
//! The library provides comprehensive error handling for both parsing and value extraction:
//!
//! - Syntax errors are detected during parsing via `ParsedSource::has_error()`
//! - Value parsing errors are returned as `ValueParseError` variants
//! - Language initialization errors are handled via `LanguageError`

/// Parser module containing all parsing functionality for Firm DSL.
///
/// This module provides the core parsing capabilities including entity extraction,
/// field parsing, and value type conversion. All parsing is done using tree-sitter
/// for robust syntax analysis.
pub mod parser;

pub mod convert;

pub mod generate;

pub mod workspace;
