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
}
