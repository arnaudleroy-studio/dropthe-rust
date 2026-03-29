//! # dropthe
//!
//! Rust data types and constants for working with the DropThe entertainment
//! and financial dataset. Covers movies, series, crypto, companies, and people.
//!
//! Homepage: <https://dropthe.org>

use std::str::FromStr;

/// Library version.
pub const VERSION: &str = "0.1.2";

/// Base URL for the DropThe platform.
pub const BASE_URL: &str = "https://dropthe.org";

/// Editorial verticals and their approximate catalog sizes.
pub const VERTICALS: &[(&str, u32)] = &[
    ("Tech", 1_200),
    ("Coin", 1_850),
    ("Movies", 25_400),
    ("Series", 4_300),
    ("Companies", 2_100),
    ("People", 6_800),
    ("Culture", 980),
    ("Travel", 1_400),
    ("Gear", 760),
    ("Gaming", 540),
];

/// The five entity types in the DropThe knowledge graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Movie,
    Series,
    Person,
    Crypto,
    Company,
}

impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Self::Movie => "movies",
            Self::Series => "series",
            Self::Person => "people",
            Self::Crypto => "cryptocurrencies",
            Self::Company => "companies",
        };
        f.write_str(label)
    }
}

/// Scoring tiers from S (exceptional) to D (low-data).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    S,
    A,
    B,
    C,
    D,
}

impl Tier {
    /// Minimum score required to reach this tier.
    pub fn min_score(self) -> u8 {
        match self {
            Tier::S => 90,
            Tier::A => 70,
            Tier::B => 50,
            Tier::C => 30,
            Tier::D => 0,
        }
    }
}

impl FromStr for Tier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" | "s" => Ok(Tier::S),
            "A" | "a" => Ok(Tier::A),
            "B" | "b" => Ok(Tier::B),
            "C" | "c" => Ok(Tier::C),
            "D" | "d" => Ok(Tier::D),
            other => Err(format!("unknown tier: {}", other)),
        }
    }
}

/// A single entity in the DropThe dataset.
#[derive(Debug, Clone)]
pub struct Entity {
    /// URL-safe identifier (e.g. `"inception-2010"`).
    pub slug: String,
    /// Display name.
    pub name: String,
    /// Entity classification.
    pub entity_type: EntityType,
    /// Editorial score (0-100), if assigned.
    pub score: Option<u8>,
    /// Free-form tags for categorization.
    pub tags: Vec<String>,
}

impl Entity {
    /// Create a new entity with required fields.
    pub fn new(slug: &str, name: &str, entity_type: EntityType) -> Self {
        Self {
            slug: slug.to_owned(),
            name: name.to_owned(),
            entity_type,
            score: None,
            tags: Vec::new(),
        }
    }

    /// Set the editorial score.
    pub fn with_score(mut self, score: u8) -> Self {
        self.score = Some(score);
        self
    }

    /// Attach tags.
    pub fn with_tags(mut self, tags: &[&str]) -> Self {
        self.tags = tags.iter().map(|t| t.to_string()).collect();
        self
    }

    /// Determine the scoring tier for this entity.
    pub fn tier(&self) -> Option<Tier> {
        self.score.map(|s| {
            if s >= 90 { Tier::S }
            else if s >= 70 { Tier::A }
            else if s >= 50 { Tier::B }
            else if s >= 30 { Tier::C }
            else { Tier::D }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_builder() {
        let e = Entity::new("bitcoin", "Bitcoin", EntityType::Crypto)
            .with_score(88)
            .with_tags(&["defi", "layer-1"]);
        assert_eq!(e.tier(), Some(Tier::A));
        assert_eq!(e.tags.len(), 2);
    }

    #[test]
    fn tier_parsing() {
        assert_eq!("S".parse::<Tier>(), Ok(Tier::S));
        assert!("X".parse::<Tier>().is_err());
    }

    #[test]
    fn verticals_not_empty() {
        assert!(VERTICALS.len() >= 5);
    }
}
