# So You Think You Know Star Wars?

Name the Star Wars character in the portrait. Three wrong answers and you're out.

🎮 [Play now](https://youthinkyouiknowstarwars.netlify.app)

- **Padawan**: pick from four names, 50 points each.
- **Master**: type the name, 100 points, or 75 if the spelling is close.
- One free hint per character, plus Give up and Skip.
- Every character comes up once before any repeats.
- The top ten scores are kept in your browser.

## How it's built

Written in Rust with [Leptos](https://leptos.dev), compiled to WebAssembly and bundled by [Trunk](https://trunkrs.dev).
The only non-Rust code is the HTML shell and the stylesheet.

| Path | What it holds |
| --- | --- |
| `src/game.rs` | The rules: judging guesses, scoring, hints, dealing rounds. No browser code, so `cargo test` covers it. |
| `src/data.rs` | The 82 characters, compiled in from `data/characters.json`. |
| `src/browser.rs` | localStorage, sessionStorage, randomness, reduced motion. |
| `src/components/` | The intro crawl, menu, game, game-over screen and leaderboard. |
| `style.css` | All styling. |

`data/characters.json` is a snapshot of [SWAPI](https://swapi.info): name, homeworld, species, first film, and vehicle and starship counts. SWAPI doesn't change, so a snapshot is all a game needs.
Portraits load from the [swapi-gallery](https://github.com/vieraboschkova/swapi-gallery) mirror of the old Star Wars Visual Guide images.

## Development

You need Rust (with the `wasm32-unknown-unknown` target, which `rust-toolchain.toml` installs) and [Trunk](https://trunkrs.dev/guide/getting-started/installation.html).

```bash
trunk serve --open        # dev server with reload
cargo test                # game rules
trunk build --release     # production build into dist/
```

Netlify builds with `netlify.toml`.
