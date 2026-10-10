use super::leaderboard::Leaderboard;
use crate::browser::now_iso;
use crate::game::{HighScore, Mode, top_ten};
use leptos::ev;
use leptos::prelude::*;

const NAME_LIMIT: usize = 20;

#[component]
pub fn GameOver(
    score: u32,
    mode: Mode,
    scores: RwSignal<Vec<HighScore>>,
    on_save: Callback<HighScore>,
    on_again: impl Fn() + Copy + Send + Sync + 'static,
    on_menu: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    // The saved entry's date stamp, which also highlights it on the board.
    let saved = RwSignal::new(None::<String>);

    // Would this score make the top ten right now?
    let makes_board = scores.with_untracked(|s| {
        let top = top_ten(s);
        top.len() < 10 || top.last().is_some_and(|last| score > last.score)
    });
    let headline = match (score, makes_board) {
        (0, _) => "No points this time",
        (_, true) => "That's a top ten score",
        _ => "Game over",
    };

    let save = move |e: ev::SubmitEvent| {
        e.prevent_default();
        let trimmed = name.with_untracked(|n| n.trim().to_string());
        if trimmed.is_empty() || saved.get_untracked().is_some() {
            return;
        }
        let date = now_iso();
        on_save.run(HighScore {
            name: trimmed,
            score,
            mode,
            date: date.clone(),
        });
        saved.set(Some(date));
    };

    view! {
        <div class="over">
            <header class="over-head">
                <h1 class="over-title">{headline}</h1>
                <p class="over-score">
                    <span class="over-score-value">{score}</span>
                    <span class="over-score-unit">"points as a "{mode.name()}</span>
                </p>
            </header>

            <Show when=move || { score > 0 }>
                <Show
                    when=move || saved.get().is_none()
                    fallback=|| view! { <p class="over-saved">"Saved to the leaderboard."</p> }
                >
                    <form class="typed over-form" on:submit=save>
                        <label class="over-label" for="player-name">"Your name for the leaderboard"</label>
                        <input
                            id="player-name"
                            type="text"
                            autocomplete="nickname"
                            maxlength=NAME_LIMIT
                            placeholder="Name"
                            prop:value=move || name.get()
                            on:input=move |e| name.set(event_target_value(&e))
                        />
                        <button class="primary" type="submit" disabled=move || name.with(|n| n.trim().is_empty())>
                            "Save score"
                        </button>
                    </form>
                </Show>
            </Show>

            <div class="over-actions">
                <button class=format!("mode {} compact", mode.class()) on:click=move |_| on_again()>
                    <span class="mode-name">"Play again as "{mode.name()}</span>
                </button>
                <button class="text-button" on:click=move |_| on_menu()>"Change mode"</button>
            </div>

            <Leaderboard scores=scores highlight=saved />
        </div>
    }
}
