use crate::{EntitySchema, EntityType, FieldId, FieldType};

impl EntitySchema {
    /// Creates all built-in schemas.
    pub fn all_builtin() -> Vec<EntitySchema> {
        vec![
            // Core entities
            EntitySchema::person(),
            EntitySchema::organization(),
            EntitySchema::industry(),
            // Customer relations
            EntitySchema::account(),
            EntitySchema::channel(),
            EntitySchema::lead(),
            EntitySchema::contact(),
            EntitySchema::interaction(),
            EntitySchema::opportunity(),
            // Work management
            EntitySchema::strategy(),
            EntitySchema::objective(),
            EntitySchema::key_result(),
            EntitySchema::project(),
            EntitySchema::task(),
            EntitySchema::review(),
            // Resources
            EntitySchema::file_asset(),
        ]
    }

    // Fundamental
    pub fn person() -> Self {
        Self::new(EntityType::new("person"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("email"), FieldType::String)
            .with_optional_field(FieldId::new("phone"), FieldType::String)
            .with_optional_field(FieldId::new("urls"), FieldType::List)
    }

    pub fn organization() -> Self {
        Self::new(EntityType::new("organization"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("address"), FieldType::String)
            .with_optional_field(FieldId::new("email"), FieldType::String)
            .with_optional_field(FieldId::new("phone"), FieldType::String)
            .with_optional_field(FieldId::new("urls"), FieldType::List)
            .with_optional_field(FieldId::new("vat_id"), FieldType::String)
            .with_optional_field(FieldId::new("industry_ref"), FieldType::Reference)
    }

    pub fn industry() -> Self {
        Self::new(EntityType::new("industry"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("sector"), FieldType::String)
            .with_optional_field(FieldId::new("classification_code"), FieldType::String)
            .with_optional_field(FieldId::new("classification_system"), FieldType::String)
    }

    // Customer relations
    pub fn account() -> Self {
        Self::new(EntityType::new("account"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("organization_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("status"), FieldType::String)
    }

    pub fn channel() -> Self {
        Self::new(EntityType::new("channel"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("type"), FieldType::String)
            .with_optional_field(FieldId::new("description"), FieldType::String)
    }

    pub fn lead() -> Self {
        Self::new(EntityType::new("lead"))
            .with_metadata()
            .with_required_field(FieldId::new("source_ref"), FieldType::Reference)
            .with_required_field(FieldId::new("status"), FieldType::String)
            .with_optional_field(FieldId::new("person_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("account_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("score"), FieldType::Integer)
    }

    pub fn contact() -> Self {
        Self::new(EntityType::new("contact"))
            .with_metadata()
            .with_optional_field(FieldId::new("source_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("person_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("account_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("role"), FieldType::String)
            .with_optional_field(FieldId::new("status"), FieldType::String)
    }

    pub fn interaction() -> Self {
        Self::new(EntityType::new("interaction"))
            .with_metadata()
            .with_required_field(FieldId::new("type"), FieldType::String)
            .with_required_field(FieldId::new("subject"), FieldType::String)
            .with_required_field(FieldId::new("initiator_ref"), FieldType::Reference)
            .with_required_field(FieldId::new("primary_contact_ref"), FieldType::Reference)
            .with_required_field(FieldId::new("interaction_date"), FieldType::DateTime)
            .with_optional_field(FieldId::new("outcome"), FieldType::String)
            .with_optional_field(FieldId::new("secondary_contacts_ref"), FieldType::List)
            .with_optional_field(FieldId::new("channel_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("opportunity_ref"), FieldType::Reference)
    }

    pub fn opportunity() -> Self {
        Self::new(EntityType::new("opportunity"))
            .with_metadata()
            .with_required_field(FieldId::new("source_ref"), FieldType::Reference)
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("status"), FieldType::String)
            .with_optional_field(FieldId::new("value"), FieldType::Currency)
            .with_optional_field(FieldId::new("probability"), FieldType::Integer)
    }

    // Work management
    pub fn strategy() -> Self {
        Self::new(EntityType::new("strategy"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("description"), FieldType::String)
            .with_optional_field(FieldId::new("source_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("status"), FieldType::String)
            .with_optional_field(FieldId::new("start_date"), FieldType::DateTime)
            .with_optional_field(FieldId::new("end_date"), FieldType::DateTime)
    }

    pub fn objective() -> Self {
        Self::new(EntityType::new("objective"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("description"), FieldType::String)
            .with_optional_field(FieldId::new("strategy_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("status"), FieldType::String)
            .with_optional_field(FieldId::new("start_date"), FieldType::DateTime)
            .with_optional_field(FieldId::new("end_date"), FieldType::DateTime)
    }

    pub fn key_result() -> Self {
        Self::new(EntityType::new("key_result"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("objective_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("start_value"), FieldType::Float)
            .with_optional_field(FieldId::new("target_value"), FieldType::Float)
            .with_optional_field(FieldId::new("current_value"), FieldType::Float)
            .with_optional_field(FieldId::new("unit"), FieldType::String)
    }

    pub fn project() -> Self {
        Self::new(EntityType::new("project"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("status"), FieldType::String)
            .with_optional_field(FieldId::new("description"), FieldType::String)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("objective_refs"), FieldType::List)
            .with_optional_field(FieldId::new("due_date"), FieldType::DateTime)
    }

    pub fn task() -> Self {
        Self::new(EntityType::new("task"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_optional_field(FieldId::new("description"), FieldType::String)
            .with_optional_field(FieldId::new("source_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("assignee_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("due_date"), FieldType::DateTime)
            .with_optional_field(FieldId::new("is_completed"), FieldType::Boolean)
            .with_optional_field(FieldId::new("completed_at"), FieldType::DateTime)
    }

    pub fn review() -> Self {
        Self::new(EntityType::new("review"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("date"), FieldType::DateTime)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("source_refs"), FieldType::List)
            .with_optional_field(FieldId::new("attendee_refs"), FieldType::List)
    }

    // Resources
    pub fn file_asset() -> Self {
        Self::new(EntityType::new("file_asset"))
            .with_metadata()
            .with_required_field(FieldId::new("name"), FieldType::String)
            .with_required_field(FieldId::new("path"), FieldType::Path)
            .with_optional_field(FieldId::new("description"), FieldType::String)
            .with_optional_field(FieldId::new("source_ref"), FieldType::Reference)
            .with_optional_field(FieldId::new("owner_ref"), FieldType::Reference)
    }
}
