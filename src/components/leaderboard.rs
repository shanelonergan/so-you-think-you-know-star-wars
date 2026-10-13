use crate::game::{HighScore, short_date, top_ten};
use leptos::prelude::*;

/// The top ten. `highlight` marks a just-saved entry by its date stamp.
#[component]
pub fn Leaderboard(
    scores: RwSignal<Vec<HighScore>>,
    #[prop(optional, into)] highlight: Signal<Option<String>>,
) -> impl IntoView {
    let top = move || scores.with(|s| top_ten(s));
    view! {
        <section class="board" aria-labelledby="board-title">
            <h2 id="board-title" class="board-title">"Top 10 Jedi Masters"</h2>
            <Show
                when=move || !top().is_empty()
                fallback=|| view! {
                    <p class="board-empty">"No scores yet. Finish a game to take the first spot."</p>
                }
            >
                <ol class="board-list">
                    {move || top()
                        .into_iter()
                        .enumerate()
                        .map(|(i, s)| {
                            let mine = highlight.get().as_deref() == Some(s.date.as_str());
                            view! {
                                <li class="board-row" class:mine=mine>
                                    <span class="board-rank">{i + 1}</span>
                                    <span class="board-name">
                                        {s.name.clone()}
                                        <span class=format!("board-mode {}", s.mode.class())>{s.mode.name()}</span>
                                    </span>
                                    <span class="board-score">{s.score}</span>
                                    <span class="board-date">{short_date(&s.date)}</span>
                                </li>
                            }
                        })
                        .collect_view()}
                </ol>
            </Show>
        </section>
    }
}
