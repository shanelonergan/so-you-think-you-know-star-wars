//! The character roster, snapshotted from SWAPI (swapi.info) into
//! `data/characters.json` and compiled into the binary, so a game never waits
//! on the network for anything but the portrait.

use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Character {
    /// SWAPI person id; also names the portrait file.
    pub id: u32,
    pub name: String,
    /// "unknown" when SWAPI doesn't know.
    pub homeworld: String,
    pub species: String,
    /// Title of the first film they appear in.
    pub film: String,
    pub vehicles: u32,
    pub starships: u32,
    pub kind: Kind,
    pub gender: Gender,
    /// How well known they are: 1 famous, 2 fan favourite, 3 deep cut.
    /// A judgment call, made by hand in `data/characters.json`.
    pub tier: u8,
}

/// What they look like at a glance, so multiple choice never pairs a face
/// with names that are obviously the wrong kind of being.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Human,
    Droid,
    Alien,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl Character {
    /// SWAPI portraits, mirrored on GitHub since starwars-visualguide.com shut down.
    pub fn portrait_url(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/vieraboschkova/swapi-gallery/main/static/assets/img/people/{}.jpg",
            self.id
        )
    }
}

pub fn characters() -> &'static [Character] {
    static ROSTER: OnceLock<Vec<Character>> = OnceLock::new();
    ROSTER.get_or_init(|| {
        serde_json::from_str(include_str!("../data/characters.json"))
            .expect("data/characters.json is valid")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roster_loads_with_unique_ids() {
        let all = characters();
        assert_eq!(all.len(), 82);
        let mut ids: Vec<u32> = all.iter().map(|c| c.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), all.len());
    }

    #[test]
    fn every_tier_is_used_and_droids_are_droids() {
        let all = characters();
        for tier in 1..=3 {
            assert!(all.iter().any(|c| c.tier == tier));
        }
        assert!(all.iter().all(|c| (1..=3).contains(&c.tier)));
        let droids: Vec<&str> = all
            .iter()
            .filter(|c| c.kind == Kind::Droid)
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(droids, ["C-3PO", "R2-D2", "R5-D4", "IG-88", "R4-P17"]);
    }
}
