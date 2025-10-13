# Business-as-code
Firm is a text-based work management system for technologists.

Firm lets you define your business entities (people, organizations, projects, tasks) in a declarative DSL, then represent it as a queryable graph that you can manage with a CLI, version control, and high-level automations.

## Features
- **Map your business:** Strategy to execution in version-controlled plain text.
- **Work management:** Project and task tracking in the command line.
- **Local-first:** Everything important in one place (that you own).
- **Rich relationships:** Link strategies, customers, projects, and tasks.
- **Automatable:** Hook into the workspace using the Firm CLI or Rust package.

## Installation
The Firm CLI is available to download via Github Releases. Install scripts are provided for desktop platforms to make that process easy.

### Linux and macOS
Run the following command to download and run the install script:

```bash
curl -fsSL https://raw.githubusercontent.com/42futures/firm/main/install.sh | sudo bash
```

## Getting started
Firm operates on a "workspace", which is a directory containing all your `.firm` DSL files. The Firm CLI processes every file in this workspace to build a unified, queryable graph of your business.

The first step is to add an entity to your workspace. You can do this either by using the CLI or by writing the DSL manually.

### Add entities with the CLI
Use `firm add` to interactively generate new entities. Out of the box, Firm supports a set of pre-built entity schemas for org mapping, customer relations and work management. The CLI will prompt you for the necessary info and generate corresponding DSL automatically.

```bash
$ firm add
```
```
Adding new entity

> Type: organization
> ID: megacorp
> Name: Megacorp Ltd.
> Email: mega@corp.com
> Urls: ["corp.com"]

Writing generated DSL to file my_workspace/generated/organization.firm
```

### Write DSL manually
Alternatively, you can create a `.firm` file and write the DSL yourself. This gives you more control and is ideal for defining multiple entities at once.

```firm
organization megacorp {
  name = "Megacorp Ltd."
  email = "mega@corp.com"
  urls = ["corp.com"]
}
```

Both of these methods achieve the same result: a new entity defined in your Firm workspace.

### Querying the workspace
Once you have entities in your workspace, you can query them using the CLI. By default, Firm will "pretty" output entities but can also be configured for JSON, allowing for downstream automations.

#### Listing entities
Use `firm list` to see all entities of a specific type.

```bash
$ firm list task
```
```
Found 7 entities with type 'task'

ID: task.design_homepage
Name: Design new homepage
Is completed: false
Assignee ref: person.jane_doe

...
```

#### Getting an entity
To view the full details of a single entity, use `firm get` followed by the entity's type and ID.

```bash
$ firm get person john_doe
```
```
Found 'person' entity with ID 'john_doe'

ID: person.john_doe
Name: John Doe
Email: john@doe.com
```

#### Exploring relationships
The power of Firm lies in its ability to build a graph of your business. Use `firm related` to explore connections to/from any entity.

```bash
$ firm related contact john_doe
```
```
Found 1 relationships for 'contact' entity with ID 'john_doe'

ID: interaction.megacorp_intro
Type: Call
Subject: Initial discussion about Project X
Interaction date: 2025-09-30 09:45:00 +02:00
Initiator ref: person.jane_smith
Primary contact ref: contact.john_doe
```

#### What's next
You've seen the basic commands for interacting with a Firm workspace. The project is a work-in-progress, and you can expect to see more sophisticated features added over time, including a more powerful query engine and tools for running business workflows directly from the CLI.

## Using Firm as a library
Beyond the CLI, you can integrate Firm's core logic directly into your own software using the `firm_core` and `firm_lang` Rust packages. This allows you to build custom tools, automations, and integrations on top of Firm.

First, add the Firm crates to your `Cargo.toml`:

```toml
[dependencies]
firm_core = { git = "https://github.com/42futures/firm.git" }
firm_lang = { git = "https://github.com/42futures/firm.git" }
```

You can then load a workspace, build the entity graph, and query it programmatically:

```rust
use firm_lang::workspace::Workspace;
use firm_core::EntityGraph;

// Load workspace from a directory
let mut workspace = Workspace::new();
workspace.load_directory("./my_workspace")?;
let build = workspace.build()?;

// Build the graph from the workspace entities
let mut graph = EntityGraph::new();
graph.add_entities(build.entities)?;
graph.build();

// Query the graph for a specific entity
let lead = graph.get_entity(&EntityId::new("lead.ai_validation_project"))?;

// Traverse a relationship to another entity
let contact_ref = lead.get_field(FieldId::new("contact_ref"))?;
let contact = contact_ref.resolve_entity_reference(&graph)?;
```

This gives you full access to the underlying data structures, providing a foundation for building custom business automations.

## Architecture

Firm is organized as a Rust workspace with three crates:

### `firm-core`
Core data structures and graph operations.

- Entity data model
- Typed fields with references
- Relationship graph with query capabilities
- Entity schemas and validation

### `firm-lang`
DSL parsing and generation.

- Tree-sitter-based parser for `.firm` files
- Conversion between DSL and entities
- Workspace support for multi-file projects
- DSL generation from entities

Grammar is defined in [tree-sitter-firm](https://github.com/42futures/tree-sitter-firm).

### `firm-cli`

Command-line interface, making the Firm workspace interactive.

## Core concepts
Firm's data model is built on a few key concepts. Each concept is accessible declaratively through the `.firm` DSL for human-readable definitions, and programmatically through the Rust packages for building your own automations.

### Entities
Entities are the fundamental business objects in your workspace, like people, organizations, or projects. Each entity has a unique ID, a type, and a collection of fields.

**In the DSL**, you define an entity with its type and ID, followed by its fields in a block:

```firm
person john_doe {
    name = "John Doe"
    email = "john@doe.com"
}
```

**In Rust**, this corresponds to an `Entity` struct:

```rust
let person = Entity::new(EntityId::new("john_doe"), EntityType::new("person"))
    .with_field(FieldId::new("name"), "John Doe")
    .with_field(FieldId::new("email"), "john@doe.com");
```

### Fields
Fields are typed key-value pairs attached to an entity. Firm supports a rich set of types:

- `String`
- `Integer`
- `Float`
- `Boolean`
- `Currency`
- `DateTime`
- `List` of other values
- `Reference` to other fields or entities
- `Path` to a local file

**In the DSL**, the syntax maps directly to these types:

```firm
my_task design_homepage {
    title = "Design new homepage"        // String
    priority = 1                         // Integer
    completed = false                    // Boolean
    budget = 5000.00 USD                 // Currency
    due_date = 2024-12-01 at 17:00 UTC   // DateTime
    tags = ["ui", "ux"]                  // List
    assignee = person.jane_doe           // Reference
    deliverable = path"homepage.zip"     // Path
}
```

**In Rust**, these are represented by the `FieldValue` enum:

```rust
// A reference to another entity
let assignee = FieldValue::Reference(
    ReferenceValue::Entity(
        EntityId::new("person.jane_doe")
    )
);
```

### Relationships and the entity graph
The power of Firm comes from connecting entities. You create relationships using `Reference` fields.

When Firm processes your workspace, it builds the **entity graph** representing of all your entities (as nodes) and their relationships (as directed edges). This graph is what allows for traversal and querying.

**In the DSL**, creating a relationship is as simple as referencing another entity's ID.

```firm
contact john_at_acme {
    person_ref = person.john_doe
    organization_ref = organization.acme_corp
}
```

**In Rust**, you build the graph by loading entities and calling the `.build()` method, which resolves all references into queryable links.

```rust
let mut graph = EntityGraph::new();
graph.add_entities(workspace.build()?.entities)?;
graph.build(); // Builds relationships from references

// Now you can traverse the graph
let contact = graph.get_entity(&EntityId::new("contact.john_at_acme"))?;
let person_ref = contact.get_field(FieldId::new("person_ref"))?;
let person = person_ref.resolve_entity_reference(&graph)?;
```

### Schemas

Schemas allow you to define and enforce a structure for your entities, ensuring data consistency. You can specify which fields are required or optional and what their types should be.

**In the DSL**, you can define a schema that other entities can adhere to:

```firm
schema custom_project {
    field {
        name = "title"
        type = "string"
        required = true
    }
    field {
        name = "budget"
        type = "currency"
        required = false
    }
}

custom_project my_project {
    title  = "My custom project"
    budget = 42000 EUR
}
```

**In Rust**, you can define schemas programmatically to validate entities.

```rust
let schema = EntitySchema::new(EntityType::new("project"))
    .with_required_field(FieldId::new("title"), FieldType::String)
    .with_optional_field(FieldId::new("budget"), FieldType::Currency);

schema.validate(&some_project_entity)?;
```

## Built-in entities

Firm includes schemas for a range of built-in entities like Person, Organization, and Industry.

Firm's entity taxonomy is built on the [REA model (Resources, Events, Agents)](https://en.wikipedia.org/wiki/Resources,_Events,_Agents) with inspiration from [Schema.org](https://schema.org/Person), designed for flexible composition and efficient queries.

Every entity maps to a Resource (thing with value), an Event (thing that happens), or an Agent (thing that acts).

We separate objective reality from business relationships:

- **Fundamental entities** represent things that exist independently (`Person`, `Organization`, `Document`)
- **Contextual entities** represent your business relationships and processes (`Contact`, `Lead`, `Project`)

Entities reference each other rather than extending. One `Person` can be referenced by multiple `Contact`, `Employee`, and `Partner` entities simultaneously.

When the entity graph is built, all `Reference` values automatically create directed edges between entities. This enables traversal queries like "find all Tasks for Opportunities whose Contacts work at Organization X" without complex joins.
