# Hololive Schedule (Rust)

A lightweight desktop viewer for the [Hololive schedule](https://schedule.hololive.tv/lives), written in Rust with [Slint](https://slint.dev) for the GUI. Fetches the live schedule straight from Hololive's public JSON API, groups streams by day, and shows streamer, time, thumbnail, title, and a red **LIVE** badge for streams that are currently on air.

## Features

- **Live data** — pulls from `https://schedule.hololive.tv/api/list/7` (the same endpoint the official site's frontend uses), refreshed on demand with the refresh button.
- **Live status** — streams that are currently broadcasting get a red **LIVE** pill next to the streamer's name, driven by the API's `isLive` field.
- **Titles** — each row shows the stream title exactly as YouTube exposes it, no second request needed.
- **Thumbnails** — downloaded on a background thread pool, so the UI never blocks. Six workers, one shared HTTP client, connection reuse.
- **Name translation** — Japanese and short-form English names are mapped to full English names (for example "角巻わため" becomes "Tsunomaki Watame", "Gigi" becomes "Gigi Murin"). Unknown names pass through unchanged.
- **Click to open** — clicking any thumbnail opens the stream in your default browser.
- **Japanese clock** — a live "Current Japanese Time" label in the header.
- **Native rendering** — one binary, no runtime, no .NET.

## Requirements

- Rust 1.75+ (edition 2021). Installing Rust on Windows (MSVC target) or macOS already brings the C toolchain the build needs, so no extra setup is required.
- On Linux, you will typically need the standard development packages: `build-essential`, `pkg-config`, `libxkbcommon-dev`, `libfontconfig1-dev`, `libgl1-mesa-dev` (or your distro's equivalents).

## Build and run

    cargo run --release

The resulting binary is `target/release/hololive-schedule` (or `hololive-schedule.exe` on Windows). It is fully self-contained — copy it anywhere and it runs.

For development:

    cargo run

## Project layout

    hololive-schedule/
    |-- Cargo.toml
    |-- build.rs             (invokes slint-build to compile ui/app.slint)
    |-- ui/
    |   `-- app.slint        (declarative UI: window, layout, live badge)
    `-- src/
        |-- main.rs          (app lifecycle, timers, message pump)
        |-- worker.rs        (background fetch + bounded thumbnail pool)
        |-- schedule.rs      (JSON API types, fetch, parse, image decode)
        `-- translate.rs     (JP/EN/ID/STARS name lookup table)

### What goes where

| File | Responsibility |
|---|---|
| `main.rs` | Create the window, wire callbacks, drive Slint's timers, apply messages to the model |
| `worker.rs` | One background thread per refresh; parse first, then fan out image downloads across six workers |
| `schedule.rs` | Deserialize the API response into `StreamRow`, download and decode thumbnails |
| `translate.rs` | `translate(name)` — pure data, one `match` arm per streamer |
| `ui/app.slint` | Everything visual; no logic |

## How it works

1. On startup, `worker::spawn` fires a background thread.
2. That thread calls `schedule::fetch_streams()`, which GETs `https://schedule.hololive.tv/api/list/7` and deserializes the JSON into a flat `Vec<StreamRow>`.
3. The rows are sent to the UI as a single `Msg::Rows` — they render immediately with names, times, titles, and LIVE badges.
4. Six worker threads then pull thumbnail URLs from a shared queue and send `Msg::Image(index, pixels)` for each, which the UI applies to the corresponding row's `pic` field.
5. A 16 ms timer in `main.rs` drains the message channel and applies every queued update in one pass, so the UI never falls behind.

## API reference

The app relies on a single public endpoint, undocumented but stable since 2019 and used by the official web frontend:

    GET https://schedule.hololive.tv/api/list/7

It returns a JSON payload shaped like this:

    {
      "dateGroupList": [
        {
          "displayDate": "10.28",
          "videoList": [
            {
              "displayDate": "19:00",
              "isLive": true,
              "url": "https://www.youtube.com/watch?v=...",
              "thumbnail": "https://img.youtube.com/vi/.../mqdefault.jpg",
              "title": "...",
              "name": "角巻わため"
            }
          ]
        }
      ]
    }

Only these six fields are used. Thumbnails point at YouTube's CDN; the schedule site itself only aggregates metadata.

If the endpoint ever changes, the only file affected is `schedule.rs` — the deserializer structs are tiny and self-contained.

## Extending

- **Add a streamer to the name map** — one line in `translate.rs`.
- **Change the API window** — `api/list/7` becomes `api/list/N` in `schedule.rs` (the server accepts roughly 1–7 days).
- **Add UI** — edit `ui/app.slint`; the Rust side only needs to change if you're adding a new data field, which means one new entry in `StreamData` and one line in `to_ui()`.
- **Tune thumbnail concurrency** — `worker_count` in `worker.rs` (default 6; higher means faster on wide pipes, lower means gentler on the CDN).

## Dependencies

| Crate | Purpose |
|---|---|
| `slint` | GUI toolkit (window, layout, images, timer) |
| `reqwest` (blocking, rustls) | HTTP client — one per thread pool, connection reuse |
| `serde` + `serde_json` | API response deserialization |
| `image` | Decode JPEG/PNG/WebP thumbnails |
| `webbrowser` | Open stream URLs in the default browser |
| `chrono` + `chrono-tz` | "Current Japanese Time" label |
| `slint-build` (build dep) | Compile `ui/app.slint` into Rust at build time |

## AI assistance

This project is a Rust rework of an earlier C# WinForms implementation. The rewrite — including the Slint UI, the JSON API client, the bounded thumbnail worker pool, the message pump, and the name translation table — was produced with the assistance of an AI coding assistant (Anthropic's Claude). Every line was reviewed, compiled, and iterated on by a human; AI was used as a pair programmer, not an autopilot.

If you're looking at this as a reference for "how to write Rust," keep that in mind: the code prioritizes clarity over idiom density, and some decisions (such as the hand-rolled `match` in `translate.rs` rather than a `phf` map) are deliberate simplicity choices rather than best-practice claims.

## License

GPL-3.0-or-later — see `LICENSE.md`.

## Disclaimer

Unofficial fan project. Not affiliated with Cover Corp. or Hololive Production. All stream metadata is fetched from publicly accessible endpoints belonging to the schedule site and YouTube. If you redistribute the binary, keep the attributions.
