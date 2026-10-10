use super::leaderboard::Leaderboard;
use crate::game::{CLOSE_MATCH_POINTS, HighScore, Mode};
use leptos::prelude::*;

#[component]
pub fn Menu(
    scores: RwSignal<Vec<HighScore>>,
    on_start: impl Fn(Mode) + Copy + Send + Sync + 'static,
    on_intro: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div class="menu">
            <header class="menu-head">
                <h1 class="logo">
                    <span>"So you think you know"</span>
                    <span class="logo-big">"Star Wars?"</span>
                </h1>
                <p class="menu-lede">"Name the character in the portrait. Three wrong answers and you're out."</p>
            </header>

            <div class="modes">
                <button class="mode padawan" on:click=move |_| on_start(Mode::Padawan)>
                    <span class="mode-name">"Play as Padawan"</span>
                    <span class="mode-rule">"Pick from four names. "{Mode::Padawan.points()}" points each."</span>
                </button>
                <button class="mode master" on:click=move |_| on_start(Mode::Master)>
                    <span class="mode-name">"Play as Master"</span>
                    <span class="mode-rule">
                        "Type the name. "{Mode::Master.points()}" points, or "
                        {CLOSE_MATCH_POINTS}" if the spelling is close."
                    </span>
                </button>
            </div>

            <Leaderboard scores=scores />

            <button class="text-button menu-intro" on:click=move |_| on_intro()>"Watch the intro"</button>
        </div>
    }
}
