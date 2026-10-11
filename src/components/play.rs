use super::sabers::Sabers;
use crate::browser::random_index;
use crate::data::characters;
use crate::game::{Deck, Mode, STARTING_LIVES, Verdict, choices_for, judge, tier_name};
use crate::hints::hints;
use leptos::ev;
use leptos::html;
use leptos::prelude::*;
use std::time::Duration;

const AFTER_CORRECT: Duration = Duration::from_millis(1400);
const AFTER_REVEAL: Duration = Duration::from_millis(2200);
const AFTER_LAST_LIFE: Duration = Duration::from_millis(1600);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Outcome {
    Correct,
    Close,
    GaveUp,
    OutOfLives,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tone {
    Good,
    Close,
    Bad,
    Neutral,
}

impl Tone {
    fn class(self) -> &'static str {
        match self {
            Tone::Good => "feedback good",
            Tone::Close => "feedback close",
            Tone::Bad => "feedback bad",
            Tone::Neutral => "feedback neutral",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Round {
    number: u32,
    answer: usize,
    choices: Vec<usize>,
    /// Padawan choices already tried and wrong.
    wrong: Vec<usize>,
    hint: Option<String>,
    outcome: Option<Outcome>,
    message: Option<(Tone, String)>,
    portrait: Portrait,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Portrait {
    Loading,
    Ready,
    Missing,
}

impl Round {
    fn deal(number: u32, deck: &mut Deck) -> Self {
        let all = characters();
        let mut rng = random_index;
        let answer = deck.draw(all, number, &mut rng);
        Round {
            number,
            answer,
            choices: choices_for(answer, all, &mut rng),
            wrong: Vec::new(),
            hint: None,
            outcome: None,
            message: None,
            portrait: Portrait::Loading,
        }
    }

    fn settled(&self) -> bool {
        self.outcome.is_some()
    }
}

#[component]
pub fn Play(
    mode: Mode,
    on_over: impl Fn(u32) + Copy + Send + Sync + 'static,
    on_quit: impl Fn() + Copy + Send + Sync + 'static,
) -> impl IntoView {
    let deck = StoredValue::new(Deck::default());
    let score = RwSignal::new(0u32);
    let lives = RwSignal::new(STARTING_LIVES);
    let round = RwSignal::new(
        deck.try_update_value(|d| Round::deal(1, d))
            .expect("deck is live"),
    );
    let guess = RwSignal::new(String::new());
    let input_ref = NodeRef::<html::Input>::new();

    let character = move || &characters()[round.with(|r| r.answer)];

    let next = move || {
        let Some(n) = round.try_with_untracked(|r| r.number) else {
            return;
        };
        if let Some(fresh) = deck.try_update_value(|d| Round::deal(n + 1, d)) {
            round.set(fresh);
            guess.set(String::new());
        }
    };
    // Moves on after a pause, unless the player has already moved on (Skip) or left.
    let next_after = move |delay: Duration| {
        let n = round.with_untracked(|r| r.number);
        set_timeout(
            move || {
                if round.try_with_untracked(|r| r.number) == Some(n) {
                    next();
                }
            },
            delay,
        );
    };

    let answer_with = move |guessed: &str, choice: Option<usize>| {
        if round.with_untracked(Round::settled) || lives.get_untracked() == 0 {
            return;
        }
        let name = characters()[round.with_untracked(|r| r.answer)]
            .name
            .clone();
        match judge(guessed, &name, mode) {
            verdict @ (Verdict::Correct | Verdict::Close) => {
                score.update(|s| *s += verdict.points(mode));
                round.update(|r| {
                    let close = verdict == Verdict::Close;
                    r.outcome = Some(if close {
                        Outcome::Close
                    } else {
                        Outcome::Correct
                    });
                    r.message = Some(if close {
                        (
                            Tone::Close,
                            format!(
                                "Close enough. It's spelled {name}. +{}",
                                verdict.points(mode)
                            ),
                        )
                    } else {
                        (Tone::Good, format!("Correct. +{}", verdict.points(mode)))
                    });
                });
                next_after(AFTER_CORRECT);
            }
            Verdict::Wrong => {
                lives.update(|l| *l = l.saturating_sub(1));
                guess.set(String::new());
                if lives.get_untracked() == 0 {
                    round.update(|r| {
                        r.outcome = Some(Outcome::OutOfLives);
                        r.message = Some((Tone::Bad, format!("Out of lives. That was {name}.")));
                    });
                    let final_score = score.get_untracked();
                    set_timeout(
                        move || {
                            if round.try_with_untracked(|_| ()).is_some() {
                                on_over(final_score);
                            }
                        },
                        AFTER_LAST_LIFE,
                    );
                } else {
                    round.update(|r| {
                        if let Some(c) = choice {
                            r.wrong.push(c);
                        }
                        r.message = Some((Tone::Bad, "Not them. Try again.".to_string()));
                    });
                }
            }
        }
    };

    let give_up = move || {
        if round.with_untracked(Round::settled) {
            return;
        }
        let name = characters()[round.with_untracked(|r| r.answer)]
            .name
            .clone();
        round.update(|r| {
            r.outcome = Some(Outcome::GaveUp);
            r.message = Some((Tone::Neutral, format!("That was {name}.")));
        });
        next_after(AFTER_REVEAL);
    };

    let show_hint = move || {
        round.update(|r| {
            if r.hint.is_none() && !r.settled() {
                let options = hints(&characters()[r.answer], characters());
                r.hint = Some(options[random_index(options.len())].clone());
            }
        });
    };

    // Keyboard: 1–4 pick a choice in Padawan mode.
    let keys = window_event_listener(ev::keydown, move |e| {
        if mode != Mode::Padawan || e.ctrl_key() || e.meta_key() || e.alt_key() {
            return;
        }
        if let Some(slot) = e
            .key()
            .parse::<usize>()
            .ok()
            .filter(|k| (1..=4).contains(k))
        {
            let picked = round.with_untracked(|r| {
                r.choices
                    .get(slot - 1)
                    .copied()
                    .filter(|c| !r.wrong.contains(c))
            });
            if let Some(c) = picked {
                answer_with(&characters()[c].name, Some(c));
            }
        }
    });
    on_cleanup(move || keys.remove());

    // Master mode: the cursor waits in the name field every round.
    Effect::new(move |_| {
        let _ = round.with(|r| r.number);
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    let settled = move || round.with(Round::settled);
    // Memos, so the portrait and the choice buttons re-render only when a new
    // round deals them, not on every hint or wrong answer.
    let number = Memo::new(move |_| round.with(|r| r.number));
    let portrait_missing = Memo::new(move |_| round.with(|r| r.portrait == Portrait::Missing));
    let choices = Memo::new(move |_| round.with(|r| r.choices.clone()));
    let revealed = move || round.with(|r| r.outcome);

    view! {
        <div class=format!("play {}", mode.class())>
            <header class="hud">
                <button class="text-button hud-quit" on:click=move |_| on_quit()>"Quit"</button>
                <p class="hud-mode">{mode.name()}" mode"</p>
                <Sabers lives=lives.into() mode=mode />
                <p class="hud-score" aria-label=move || format!("Score {}", score.get())>
                    <span class="hud-score-value">{move || score.get()}</span>
                    <span class="hud-score-unit">"pts"</span>
                </p>
            </header>

            <div class="table">
                <figure
                    class="cardback"
                    class:loading=move || round.with(|r| r.portrait == Portrait::Loading)
                    class:won=move || matches!(revealed(), Some(Outcome::Correct | Outcome::Close))
                    class:lost=move || matches!(revealed(), Some(Outcome::GaveUp | Outcome::OutOfLives))
                >
                    <figcaption class="cardback-top">
                        <span class="cardback-round">"Round "{move || round.with(|r| r.number)}</span>
                        <span class="cardback-tier">{move || tier_name(character().tier)}</span>
                    </figcaption>
                    <div class="bubble">
                        {move || {
                            let _ = number.get();
                            let c = &characters()[round.with_untracked(|r| r.answer)];
                            if portrait_missing.get() {
                                view! {
                                    <p class="bubble-missing">"The portrait didn't load. Use a hint, or skip this one."</p>
                                }.into_any()
                            } else {
                                view! {
                                    <img
                                        src=c.portrait_url()
                                        alt="A Star Wars character. Who is it?"
                                        on:load=move |_| {
                                            if round.with_untracked(|r| r.portrait == Portrait::Loading) {
                                                round.update(|r| r.portrait = Portrait::Ready);
                                            }
                                        }
                                        on:error=move |_| round.update(|r| r.portrait = Portrait::Missing)
                                    />
                                }.into_any()
                            }
                        }}
                    </div>
                    <p class="nameplate" aria-live="polite">
                        {move || if revealed().is_some() { character().name.clone() } else { "Who is this?".to_string() }}
                    </p>
                </figure>

                <section class="console" aria-label="Your answer">
                    <p class="hint" class:shown=move || round.with(|r| r.hint.is_some())>
                        {move || round.with(|r| r.hint.clone())
                            .unwrap_or_else(|| "Stuck? A hint costs nothing.".to_string())}
                    </p>

                    {match mode {
                        Mode::Padawan => view! {
                            <div class="choices">
                                {move || choices.get()
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, c)| {
                                        let name = characters()[c].name.clone();
                                        let is_wrong = move || round.with(|r| r.wrong.contains(&c));
                                        let is_answer = move || round.with(|r| r.answer == c);
                                        view! {
                                            <button
                                                class="choice"
                                                class:wrong=is_wrong
                                                class:right=move || is_answer() && revealed().is_some()
                                                disabled=move || is_wrong() || settled()
                                                on:click={
                                                    let guessed = name.clone();
                                                    move |_| answer_with(&guessed, Some(c))
                                                }
                                            >
                                                <kbd class="choice-key">{i + 1}</kbd>
                                                <span>{name.clone()}</span>
                                            </button>
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }.into_any(),
                        Mode::Master => view! {
                            <form class="typed" on:submit=move |e: ev::SubmitEvent| {
                                e.prevent_default();
                                let g = guess.get_untracked();
                                if !g.trim().is_empty() {
                                    answer_with(&g, None);
                                }
                            }>
                                <label class="visually-hidden" for="guess">"Character name"</label>
                                <input
                                    id="guess"
                                    node_ref=input_ref
                                    type="text"
                                    autocomplete="off"
                                    autocapitalize="words"
                                    spellcheck="false"
                                    placeholder="Their name"
                                    prop:value=move || guess.get()
                                    on:input=move |e| guess.set(event_target_value(&e))
                                    disabled=settled
                                />
                                <button class="primary" type="submit" disabled=move || settled() || guess.with(|g| g.trim().is_empty())>
                                    "Guess"
                                </button>
                            </form>
                        }.into_any(),
                    }}

                    <p class="feedback-line" aria-live="polite">
                        {move || round.with(|r| r.message.clone()).map(|(tone, text)| view! {
                            <span class=tone.class()>{text}</span>
                        })}
                    </p>

                    <div class="actions">
                        <button
                            class="text-button"
                            on:click=move |_| show_hint()
                            disabled=move || settled() || round.with(|r| r.hint.is_some())
                        >"Show a hint"</button>
                        <button class="text-button" on:click=move |_| give_up() disabled=settled>"Give up"</button>
                        <button
                            class="text-button"
                            on:click=move |_| next()
                            disabled=move || revealed() == Some(Outcome::OutOfLives)
                        >"Skip"</button>
                    </div>
                </section>
            </div>
        </div>
    }
}
