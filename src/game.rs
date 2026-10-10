//! Game rules, with no browser or UI types, so they run under `cargo test`.
//! Randomness comes in as a closure returning an index below its argument.

use crate::data::Character;
use serde::{Deserialize, Serialize};

pub const STARTING_LIVES: u32 = 3;
pub const CLOSE_MATCH_POINTS: u32 = 75;
pub const CHOICES: usize = 4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Padawan,
    Master,
}

impl Mode {
    pub fn name(self) -> &'static str {
        match self {
            Mode::Padawan => "Padawan",
            Mode::Master => "Master",
        }
    }

    pub fn points(self) -> u32 {
        match self {
            Mode::Padawan => 50,
            Mode::Master => 100,
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Mode::Padawan => "padawan",
            Mode::Master => "master",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Correct,
    /// Master mode only: a near miss on spelling, worth `CLOSE_MATCH_POINTS`.
    Close,
    Wrong,
}

impl Verdict {
    pub fn points(self, mode: Mode) -> u32 {
        match self {
            Verdict::Correct => mode.points(),
            Verdict::Close => CLOSE_MATCH_POINTS,
            Verdict::Wrong => 0,
        }
    }
}

fn normalize(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

pub fn judge(guess: &str, answer: &str, mode: Mode) -> Verdict {
    if normalize(guess) == normalize(answer) {
        Verdict::Correct
    } else if mode == Mode::Master && is_close_match(guess, answer) {
        Verdict::Close
    } else {
        Verdict::Wrong
    }
}

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut row = vec![0; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        row[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let substitution = prev[j] + usize::from(ca != cb);
            row[j + 1] = substitution.min(prev[j + 1] + 1).min(row[j] + 1);
        }
        std::mem::swap(&mut prev, &mut row);
    }
    prev[b.len()]
}

/// Close but not exact: within 30% of the answer's length, and never more
/// than three edits.
pub fn is_close_match(guess: &str, answer: &str) -> bool {
    let guess = normalize(guess);
    let answer = normalize(answer);
    if guess.is_empty() || guess == answer {
        return false;
    }
    let (g, a) = (guess.chars().count(), answer.chars().count());
    if g.abs_diff(a) > 3 {
        return false;
    }
    let threshold = 3.min(a * 3 / 10);
    levenshtein(&guess, &answer) <= threshold
}

/// Everything true about a character that makes a fair hint.
pub fn hints(c: &Character) -> Vec<String> {
    let mut out = Vec::new();
    if c.homeworld != "unknown" {
        out.push(format!("They come from {}.", c.homeworld));
    }
    out.push(match c.species.as_str() {
        "Droid" => "They're a droid.".to_string(),
        "Human" => "They're human.".to_string(),
        s => format!("Species: {s}."),
    });
    out.push(format!("They first appear in {}.", c.film));
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
    out
}

pub fn shuffle<T>(items: &mut [T], rng: &mut impl FnMut(usize) -> usize) {
    for i in (1..items.len()).rev() {
        items.swap(i, rng(i + 1));
    }
}

/// Deals every character once before any repeats.
#[derive(Clone, Debug, Default)]
pub struct Deck {
    remaining: Vec<usize>,
}

impl Deck {
    pub fn draw(&mut self, size: usize, rng: &mut impl FnMut(usize) -> usize) -> usize {
        if self.remaining.is_empty() {
            self.remaining = (0..size).collect();
            shuffle(&mut self.remaining, rng);
        }
        self.remaining.pop().expect("deck refilled")
    }
}

/// The answer plus three other characters, in random order.
pub fn choices_for(answer: usize, size: usize, rng: &mut impl FnMut(usize) -> usize) -> Vec<usize> {
    let mut others: Vec<usize> = (0..size).filter(|&i| i != answer).collect();
    shuffle(&mut others, rng);
    let mut picked: Vec<usize> = others.into_iter().take(CHOICES - 1).collect();
    picked.push(answer);
    shuffle(&mut picked, rng);
    picked
}

/// "2025-01-22T22:46:54.000Z" → "22 Jan 2025". Anything unexpected is shown as-is.
pub fn short_date(iso: &str) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let parts: Option<(u32, usize, u32)> = (|| {
        let y = iso.get(0..4)?.parse().ok()?;
        let m: usize = iso.get(5..7)?.parse().ok()?;
        let d = iso.get(8..10)?.parse().ok()?;
        Some((y, m, d))
    })();
    match parts {
        Some((y, m @ 1..=12, d)) => format!("{d} {} {y}", MONTHS[m - 1]),
        _ => iso.to_string(),
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
    pub mode: Mode,
    pub date: String,
}

/// Best first; ties go to whoever got there first.
pub fn top_ten(scores: &[HighScore]) -> Vec<HighScore> {
    let mut sorted = scores.to_vec();
    sorted.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.date.cmp(&b.date)));
    sorted.truncate(10);
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::characters;

    fn counter() -> impl FnMut(usize) -> usize {
        let mut n = 7usize;
        move |below| {
            n = n.wrapping_mul(1_103_515_245).wrapping_add(12_345);
            (n >> 8) % below
        }
    }

    #[test]
    fn exact_guesses_ignore_case_and_spacing() {
        assert_eq!(
            judge("  luke   SKYWALKER ", "Luke Skywalker", Mode::Padawan),
            Verdict::Correct
        );
    }

    #[test]
    fn close_spelling_only_counts_in_master_mode() {
        assert_eq!(
            judge("Obi-Wan Kenobee", "Obi-Wan Kenobi", Mode::Master),
            Verdict::Close
        );
        assert_eq!(
            judge("Obi-Wan Kenobee", "Obi-Wan Kenobi", Mode::Padawan),
            Verdict::Wrong
        );
    }

    #[test]
    fn short_names_need_near_exact_spelling() {
        assert!(is_close_match("R2-D3", "R2-D2"));
        assert!(!is_close_match("R3-D3", "R2-D2"));
        assert!(is_close_match("Yod", "Yoda"));
        assert!(!is_close_match("Yo", "Yoda"));
        assert!(!is_close_match("", "Yoda"));
    }

    #[test]
    fn far_guesses_are_wrong() {
        assert_eq!(
            judge("Han Solo", "Leia Organa", Mode::Master),
            Verdict::Wrong
        );
    }

    #[test]
    fn levenshtein_matches_known_distances() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("Padmé", "Padme"), 1);
    }

    #[test]
    fn points_follow_the_original_scoring() {
        assert_eq!(Verdict::Correct.points(Mode::Padawan), 50);
        assert_eq!(Verdict::Correct.points(Mode::Master), 100);
        assert_eq!(Verdict::Close.points(Mode::Master), 75);
    }

    #[test]
    fn hints_skip_unknown_homeworlds_and_zero_counts() {
        let yoda = characters().iter().find(|c| c.name == "Yoda").unwrap();
        let hints = hints(yoda);
        assert!(hints.iter().all(|h| !h.contains("unknown")));
        assert!(hints.iter().all(|h| !h.contains("piloted")));
    }

    #[test]
    fn deck_deals_everyone_before_repeating() {
        let mut rng = counter();
        let mut deck = Deck::default();
        let mut seen: Vec<usize> = (0..82).map(|_| deck.draw(82, &mut rng)).collect();
        seen.sort_unstable();
        assert_eq!(seen, (0..82).collect::<Vec<_>>());
    }

    #[test]
    fn choices_hold_the_answer_once_among_four_distinct() {
        let mut rng = counter();
        for answer in 0..82 {
            let mut c = choices_for(answer, 82, &mut rng);
            assert_eq!(c.len(), 4);
            assert_eq!(c.iter().filter(|&&i| i == answer).count(), 1);
            c.sort_unstable();
            c.dedup();
            assert_eq!(c.len(), 4);
        }
    }

    #[test]
    fn dates_read_short() {
        assert_eq!(short_date("2025-01-22T22:46:54.000Z"), "22 Jan 2025");
        assert_eq!(short_date("whenever"), "whenever");
    }

    #[test]
    fn scores_from_the_react_version_still_parse() {
        let saved =
            r#"[{"name":"Shane","score":450,"mode":"master","date":"2025-01-22T22:46:54.000Z"}]"#;
        let scores: Vec<HighScore> = serde_json::from_str(saved).unwrap();
        assert_eq!(scores[0].mode, Mode::Master);
    }

    #[test]
    fn top_ten_sorts_and_truncates() {
        let scores: Vec<HighScore> = (0..12)
            .map(|i| HighScore {
                name: format!("p{i}"),
                score: i * 10,
                mode: Mode::Padawan,
                date: format!("2025-01-{:02}", i + 1),
            })
            .collect();
        let top = top_ten(&scores);
        assert_eq!(top.len(), 10);
        assert_eq!(top[0].score, 110);
    }
}
