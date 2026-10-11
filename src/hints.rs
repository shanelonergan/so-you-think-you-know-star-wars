//! Hints: true things about a character that narrow it down without giving
//! the name away. Facts come from SWAPI and akabab/starwars-api.

use crate::data::{Character, Kind};

/// Affiliations worth a hint, best first, with how to say them. akabab lists
/// dozens per character, most of them too obscure to help.
const AFFILIATIONS: &[(&str, &str)] = &[
    ("Sith", "They followed the dark side as a Sith."),
    ("Jedi High Council", "They sat on the Jedi High Council."),
    ("Jedi Order", "They were a Jedi."),
    (
        "Alliance to Restore the Republic",
        "They fought for the Rebel Alliance.",
    ),
    ("Galactic Empire", "They served the Galactic Empire."),
    ("Separatist Council", "They sat on the Separatist Council."),
    (
        "Confederacy of Independent Systems",
        "They sided with the Separatists.",
    ),
    (
        "Trade Federation",
        "They were part of the Trade Federation.",
    ),
    ("Techno Union", "They were part of the Techno Union."),
    (
        "InterGalactic Banking Clan",
        "They were part of the InterGalactic Banking Clan.",
    ),
    (
        "Jabba Desilijic Tiure's criminal empire",
        "They worked for Jabba the Hutt.",
    ),
    (
        "Bounty Hunters' Guild",
        "They belonged to the Bounty Hunters' Guild.",
    ),
    ("Red Squadron", "They flew with Red Squadron."),
    ("Rogue Squadron", "They flew with Rogue Squadron."),
    ("Gold Squadron", "They flew with Gold Squadron."),
    ("Green Squadron", "They flew with Green Squadron."),
    ("501st Legion", "They served with the 501st Legion."),
    (
        "Naboo Royal Handmaidens",
        "They were one of Padmé's handmaidens.",
    ),
    (
        "Royal Naboo Security Forces",
        "They served in Naboo's security forces.",
    ),
    ("Royal House of Naboo", "They served Naboo's royal house."),
    (
        "Gungan High Council",
        "They sat on the Gungan High Council.",
    ),
    ("Gungan Grand Army", "They served in the Gungan Grand Army."),
    (
        "Office of the Chancellor",
        "They worked in the Chancellor's office.",
    ),
    // Which era's senate akabab files someone under is unreliable, so neither is named.
    ("Galactic Senate", "They sat in the Senate."),
    ("Imperial Senate", "They sat in the Senate."),
    ("Cloud City", "They made a home on Cloud City."),
    ("Cloud City colony", "They made a home on Cloud City."),
    ("Lars family", "They were part of the Lars family."),
    ("Galactic Republic", "They served the Galactic Republic."),
];

/// Affiliations that make another one redundant or misleading: a Rebel who
/// went through an Imperial academy is still a Rebel.
const OUTRANKED_BY: &[(&str, &[&str])] = &[
    ("Jedi Order", &["Jedi High Council"]),
    // akabab counts anyone who ever brushed the Empire; say it only when
    // nothing more defining applies.
    (
        "Galactic Empire",
        &[
            "Alliance to Restore the Republic",
            "Sith",
            "Royal House of Naboo",
            "Gungan High Council",
            "Jabba Desilijic Tiure's criminal empire",
            "Bounty Hunters' Guild",
            "Confederacy of Independent Systems",
        ],
    ),
    (
        "Confederacy of Independent Systems",
        &["Separatist Council", "Alliance to Restore the Republic"],
    ),
    (
        "Royal House of Naboo",
        &["Naboo Royal Handmaidens", "Royal Naboo Security Forces"],
    ),
    ("Imperial Senate", &["Galactic Senate"]),
    (
        "Galactic Republic",
        &["Jabba Desilijic Tiure's criminal empire"],
    ),
];

const MAX_AFFILIATION_HINTS: usize = 2;
const MAX_PEOPLE_NAMED: usize = 2;

/// Teachers and students players can be expected to know: anyone in the
/// roster, plus these.
const KNOWN_PEOPLE: &[&str] = &[
    "Darth Sidious",
    "Darth Plagueis",
    "Darth Tyranus",
    "Ahsoka Tano",
    "Asajj Ventress",
    "Savage Opress",
    "Quinlan Vos",
    "Ben Solo",
    "Rey",
];

/// Words in a name that would give it away. Titles everyone shares don't.
fn name_words(name: &str) -> Vec<String> {
    words(name)
        .into_iter()
        .filter(|w| w.chars().count() >= 3 && !matches!(w.as_str(), "the" | "darth"))
        .collect()
}

fn words(s: &str) -> Vec<String> {
    s.split(|ch: char| !ch.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect()
}

fn gives_away(text: &str, name: &[String]) -> bool {
    words(text).iter().any(|w| name.contains(w))
}

fn join_names(names: &[&str]) -> String {
    match names {
        [one] => one.to_string(),
        [a, b] => format!("{a} and {b}"),
        _ => names.join(", "),
    }
}

fn article(noun: &str) -> &'static str {
    match noun.chars().next().map(|c| c.to_ascii_lowercase()) {
        Some('a' | 'e' | 'i' | 'o' | 'u') => "an",
        _ => "a",
    }
}

/// Everything true about a character that makes a fair hint.
pub fn hints(c: &Character, roster: &[Character]) -> Vec<String> {
    let name = name_words(&c.name);
    let mut out = Vec::new();

    if c.homeworld != "unknown" {
        out.push(format!("They come from {}.", c.homeworld));
    }

    match (c.kind, &c.droid_class, &c.manufacturer) {
        (Kind::Droid, Some(class), Some(maker)) => {
            let class = class.to_lowercase();
            out.push(format!(
                "They're {} {class} made by {maker}.",
                article(&class)
            ));
        }
        (Kind::Droid, Some(class), None) => {
            let class = class.to_lowercase();
            out.push(format!("They're {} {class}.", article(&class)));
        }
        (Kind::Droid, None, _) => out.push("They're a droid.".to_string()),
        (Kind::Human, ..) => out.push("They're human.".to_string()),
        (Kind::Alien, ..) => out.push(format!("Species: {}.", c.species)),
    }

    out.push(format!("They first appear in {}.", c.film));

    let has = |a: &str| c.affiliations.iter().any(|x| x == a);
    let affiliation_hints: Vec<String> = AFFILIATIONS
        .iter()
        .filter(|(a, _)| has(a))
        .filter(|(a, _)| {
            !OUTRANKED_BY
                .iter()
                .any(|(weaker, stronger)| weaker == a && stronger.iter().any(|s| has(s)))
        })
        .filter(|(a, text)| !gives_away(a, &name) && !gives_away(text, &name))
        .map(|(_, text)| text.to_string())
        .collect();
    // The Republic is only worth saying when there's nothing more specific.
    let specific = affiliation_hints.len() > 1;
    out.extend(
        affiliation_hints
            .into_iter()
            .filter(|t| !(specific && t.contains("Galactic Republic")))
            .take(MAX_AFFILIATION_HINTS),
    );

    let known = |p: &&String| {
        (roster.iter().any(|r| &r.name == *p) || KNOWN_PEOPLE.contains(&p.as_str()))
            && !gives_away(p, &name)
    };
    let masters: Vec<&str> = c
        .masters
        .iter()
        .filter(known)
        .take(MAX_PEOPLE_NAMED)
        .map(String::as_str)
        .collect();
    if !masters.is_empty() {
        out.push(format!("They were trained by {}.", join_names(&masters)));
    }
    let apprentices: Vec<&str> = c
        .apprentices
        .iter()
        .filter(known)
        .take(MAX_PEOPLE_NAMED)
        .map(String::as_str)
        .collect();
    if !apprentices.is_empty() {
        out.push(format!("They trained {}.", join_names(&apprentices)));
    }

    match c.vehicles {
        0 => {}
        1 => out.push("They've piloted one vehicle.".to_string()),
        n => out.push(format!("They've piloted {n} vehicles.")),
    }
    match c.starships {
        0 => {}
        1 => out.push("They've flown one starship.".to_string()),
        n => out.push(format!("They've flown {n} starships.")),
    }

    out.retain(|h| !gives_away(h, &name));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::characters;

    fn hints_for(name: &str) -> Vec<String> {
        let all = characters();
        hints(all.iter().find(|c| c.name == name).unwrap(), all)
    }

    #[test]
    fn no_hint_says_any_part_of_the_name() {
        for c in characters() {
            let name = name_words(&c.name);
            for h in hints(c, characters()) {
                assert!(!gives_away(&h, &name), "{}: {h}", c.name);
            }
        }
    }

    #[test]
    fn everyone_gets_at_least_two_hints() {
        for c in characters() {
            assert!(hints(c, characters()).len() >= 2, "{}", c.name);
        }
    }

    #[test]
    fn hints_skip_unknown_homeworlds_and_zero_counts() {
        let yoda = hints_for("Yoda");
        assert!(
            yoda.iter()
                .all(|h| !h.contains("unknown") && !h.contains("piloted"))
        );
    }

    #[test]
    fn affiliations_teachers_and_droid_models_show_up() {
        assert!(
            hints_for("Mace Windu").contains(&"They sat on the Jedi High Council.".to_string())
        );
        assert!(
            hints_for("Luke Skywalker")
                .contains(&"They were trained by Obi-Wan Kenobi and Yoda.".to_string())
        );
        assert!(
            hints_for("R2-D2")
                .contains(&"They're an astromech droid made by Industrial Automaton.".to_string())
        );
        assert!(
            hints_for("Greedo").contains(&"They were part of the Trade Federation.".to_string())
        );
    }

    #[test]
    fn rebels_from_imperial_academies_read_as_rebels() {
        let wedge = hints_for("Wedge Antilles");
        assert!(wedge.iter().any(|h| h.contains("Rebel Alliance")));
        assert!(wedge.iter().all(|h| !h.contains("Galactic Empire")));
    }

    /// For reviewing the wording: `cargo test print_every_hint -- --ignored --nocapture`.
    #[test]
    #[ignore]
    fn print_every_hint() {
        for c in characters() {
            println!("{}", c.name);
            for h in hints(c, characters()) {
                println!("    {h}");
            }
        }
    }

    #[test]
    fn force_ghost_lessons_are_not_training() {
        assert!(hints_for("Yoda").iter().all(|h| !h.contains("trained by")));
    }

    #[test]
    fn the_empire_is_mentioned_only_when_it_defines_them() {
        for name in [
            "Padmé Amidala",
            "Jar Jar Binks",
            "Jabba Desilijic Tiure",
            "C-3PO",
        ] {
            assert!(
                hints_for(name)
                    .iter()
                    .all(|h| !h.contains("Galactic Empire") && !h.contains("Separatists")),
                "{name}"
            );
        }
        assert!(
            hints_for("Wilhuff Tarkin").contains(&"They served the Galactic Empire.".to_string())
        );
    }

    #[test]
    fn obscure_teachers_are_left_out() {
        let palpatine = hints_for("Palpatine");
        assert!(
            palpatine
                .iter()
                .all(|h| !h.contains("Nikto") && !h.contains("Vergere"))
        );
    }
}
