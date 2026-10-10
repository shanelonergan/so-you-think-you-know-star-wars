mod browser;
mod components;
mod data;
mod game;

use components::{GameOver, Intro, Menu, Play, Starfield};
use game::{HighScore, Mode};
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Intro,
    Menu,
    Playing,
    Over,
}

#[component]
fn App() -> impl IntoView {
    let first = if browser::intro_seen() || browser::prefers_reduced_motion() {
        Screen::Menu
    } else {
        Screen::Intro
    };
    let screen = RwSignal::new(first);
    let mode = RwSignal::new(Mode::Padawan);
    let final_score = RwSignal::new(0u32);
    // Bumped on every new game so Play mounts fresh even when the mode repeats.
    let game_id = RwSignal::new(0u32);
    let scores = RwSignal::new(browser::load_scores());

    let start = move |m: Mode| {
        mode.set(m);
        game_id.update(|n| *n += 1);
        screen.set(Screen::Playing);
    };
    let save = Callback::new(move |entry: HighScore| {
        scores.update(|s| s.push(entry));
        scores.with_untracked(|s| browser::save_scores(s));
    });

    view! {
        <Starfield />
        <main class="app">
            {move || match screen.get() {
                Screen::Intro => view! {
                    <Intro on_done=move || {
                        browser::mark_intro_seen();
                        screen.set(Screen::Menu);
                    } />
                }.into_any(),
                Screen::Menu => view! {
                    <Menu
                        scores=scores
                        on_start=start
                        on_intro=move || screen.set(Screen::Intro)
                    />
                }.into_any(),
                Screen::Playing => {
                    let _ = game_id.get();
                    view! {
                        <Play
                            mode=mode.get_untracked()
                            on_over=move |score| {
                                final_score.set(score);
                                screen.set(Screen::Over);
                            }
                            on_quit=move || screen.set(Screen::Menu)
                        />
                    }.into_any()
                }
                Screen::Over => view! {
                    <GameOver
                        score=final_score.get_untracked()
                        mode=mode.get_untracked()
                        scores=scores
                        on_save=save
                        on_again=move || start(mode.get_untracked())
                        on_menu=move || screen.set(Screen::Menu)
                    />
                }.into_any(),
            }}
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
