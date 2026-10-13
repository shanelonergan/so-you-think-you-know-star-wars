//! The few browser APIs the game touches. Every storage call fails soft:
//! private windows and blocked storage just mean nothing is remembered.

use crate::game::HighScore;
use leptos::prelude::window;

/// Same key as the React version, so existing high scores carry over.
const SCORES_KEY: &str = "star-wars-quiz-highscores";
const INTRO_SEEN_KEY: &str = "star-wars-quiz-intro-seen";

/// A random index below `below`.
pub fn random_index(below: usize) -> usize {
    ((js_sys::Math::random() * below as f64) as usize).min(below.saturating_sub(1))
}

pub fn load_scores() -> Vec<HighScore> {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item(SCORES_KEY).ok().flatten())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

pub fn save_scores(scores: &[HighScore]) {
    let (Some(storage), Ok(json)) = (
        window().local_storage().ok().flatten(),
        serde_json::to_string(scores),
    ) else {
        return;
    };
    let _ = storage.set_item(SCORES_KEY, &json);
}

pub fn now_iso() -> String {
    js_sys::Date::new_0().to_iso_string().into()
}

pub fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .is_some_and(|m| m.matches())
}

/// The opening crawl plays once per browser session.
pub fn intro_seen() -> bool {
    window()
        .session_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item(INTRO_SEEN_KEY).ok().flatten())
        .is_some()
}

pub fn mark_intro_seen() {
    if let Some(s) = window().session_storage().ok().flatten() {
        let _ = s.set_item(INTRO_SEEN_KEY, "1");
    }
}
