# dropthe

[![Crates.io](https://img.shields.io/crates/v/dropthe.svg)](https://crates.io/crates/dropthe)
[![docs.rs](https://img.shields.io/docsrs/dropthe)](https://docs.rs/dropthe)

Rust data types and constants for working with the DropThe entertainment and financial dataset. DropThe is a data utility media platform covering 25,000+ movies, 1,800+ entities across crypto, companies, and public figures, with structured metadata, knowledge graph relationships, and editorial scoring.

This crate provides strongly-typed representations of the entity catalog, making it straightforward to build tooling, analysis scripts, or integrations that consume DropThe data in Rust.

## Installation

```toml
[dependencies]
dropthe = "0.1.2"
```

## Quick Start

### Entity Types

Every record in the dataset belongs to one of five entity types. The `EntityType` enum keeps your match arms exhaustive:

```rust
use dropthe::{EntityType, Entity};

fn describe(entity: &Entity) -> &'static str {
    match entity.entity_type {
        EntityType::Movie      => "feature film or documentary",
        EntityType::Series     => "television or streaming series",
        EntityType::Person     => "public figure (director, actor, founder)",
        EntityType::Crypto     => "cryptocurrency or token",
        EntityType::Company    => "publicly traded or private company",
    }
}
```

### Building Entity Records

Use the builder-style constructor to assemble entities with optional metadata:

```rust
use dropthe::{Entity, EntityType};

let entity = Entity::new("inception-2010", "Inception", EntityType::Movie)
    .with_score(92)
    .with_tags(&["sci-fi", "thriller", "nolan"]);

assert_eq!(entity.slug, "inception-2010");
assert_eq!(entity.score, Some(92));
```

### Data Coverage Constants

The `VERTICALS` table lists the editorial verticals with their current catalog sizes. Useful for validation or building progress dashboards:

```rust
use dropthe::VERTICALS;

for (name, count) in VERTICALS {
    println!("{:<14} {:>6} items", name, count);
}
// Tech             1200 items
// Coin             1850 items
// Movies          25400 items
// ...
```

### Scoring Tiers

Entities are ranked into tiers from S (exceptional) down to D (low-data). Parse tiers safely with `from_str`:

```rust
use dropthe::Tier;

let tier: Tier = "A".parse().expect("valid tier");
assert_eq!(tier.min_score(), 70);
```

## Available Data

The DropThe dataset spans entertainment, finance, and technology. The primary verticals are movies and series (sourced from public film databases), cryptocurrencies (live market data), companies (funding and financials), and people (directors, founders, public figures). Each entity carries structured metadata -- release dates, genres, market cap, funding rounds -- plus editorial scoring and knowledge graph links that connect related records across verticals.

## Links

- [DropThe Platform](https://dropthe.org) -- search, browse, and explore the full dataset
- [GitHub Repository](https://github.com/arnaudleroy-studio/dropthe-rust)

## License

MIT
