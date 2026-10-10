use crate::game::{Mode, STARTING_LIVES};
use leptos::prelude::*;

/// Lives as lightsabers: each one lost retracts its blade.
#[component]
pub fn Sabers(lives: Signal<u32>, mode: Mode) -> impl IntoView {
    let label = move || match lives.get() {
        1 => "1 life left".to_string(),
        n => format!("{n} lives left"),
    };
    view! {
        <div class=format!("sabers {}", mode.class()) role="img" aria-label=label>
            {(0..STARTING_LIVES)
                .map(move |i| view! {
                    <span class="saber" class:lost=move || { i >= lives.get() }>
                        <span class="saber-hilt"></span>
                        <span class="saber-blade"></span>
                    </span>
                })
                .collect_view()}
        </div>
    }
}
