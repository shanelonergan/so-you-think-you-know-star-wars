use leptos::prelude::*;

/// Three layers of stars drawn as box-shadows on 1px dots. Seeded, so the sky
/// is the same on every visit.
#[component]
pub fn Starfield() -> impl IntoView {
    let mut seed: u32 = 0x5747_4152; // "SWAR"
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed as f64 / u32::MAX as f64
    };
    let mut layer = |count: usize, max_alpha: f64| -> String {
        (0..count)
            .map(|_| {
                let (x, y, a) = (
                    next() * 100.0,
                    next() * 100.0,
                    0.25 + next() * (max_alpha - 0.25),
                );
                // A few stars run warm or blue, like a real sky.
                let tint = match (next() * 10.0) as u8 {
                    0 => "255,226,190",
                    1 => "190,215,255",
                    _ => "255,255,255",
                };
                format!("{x:.2}vw {y:.2}vh rgba({tint},{a:.2})")
            })
            .collect::<Vec<_>>()
            .join(",")
    };
    let small = layer(260, 0.7);
    let medium = layer(70, 0.9);
    let large = layer(18, 1.0);

    view! {
        <div class="starfield" aria-hidden="true">
            <span class="star star-s" style:box-shadow=small></span>
            <span class="star star-m" style:box-shadow=medium></span>
            <span class="star star-l" style:box-shadow=large></span>
        </div>
    }
}
