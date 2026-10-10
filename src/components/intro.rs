use leptos::ev;
use leptos::prelude::*;
use std::time::Duration;

/// How long the full crawl runs before the menu takes over on its own.
const CRAWL: Duration = Duration::from_millis(20_000);

/// The opening crawl. Any key, click or tap skips it.
#[component]
pub fn Intro(on_done: impl Fn() + Copy + Send + Sync + 'static) -> impl IntoView {
    let finished = StoredValue::new(false);
    let finish = move || {
        if finished.try_get_value() == Some(false) {
            finished.set_value(true);
            on_done();
        }
    };
    set_timeout(finish, CRAWL);
    let keys = window_event_listener(ev::keydown, move |_| finish());
    on_cleanup(move || keys.remove());

    view! {
        <section class="intro" on:click=move |_| finish() aria-label="Opening crawl">
            <p class="intro-long-ago">"A long time ago in a galaxy far,"<br />"far away…"</p>
            <h1 class="intro-logo">
                <span>"So you think"</span>
                <span>"you know"</span>
                <span class="intro-logo-big">"Star Wars"</span>
            </h1>
            <div class="crawl-stage">
                <div class="crawl">
                    <p class="crawl-episode">"Episode ∞"</p>
                    <h2 class="crawl-title">"The Quiz"</h2>
                    <p>
                        "It is a period of fierce debate. Fans striking from hidden "
                        "group chats have claimed to know every face in every cantina, "
                        "every pilot in every squadron."
                    </p>
                    <p>
                        "Eighty-two faces from the first six films stand ready to test "
                        "that claim. A challenger has three lives, a few hints, and "
                        "nothing but memory to name them all."
                    </p>
                    <p>
                        "Choose your path. A Padawan picks from four names. A Master "
                        "must type the name, spelled close enough to count…"
                    </p>
                </div>
            </div>
            <button class="intro-skip" on:click=move |ev| {
                ev.stop_propagation();
                finish();
            }>"Skip intro"</button>
        </section>
    }
}
