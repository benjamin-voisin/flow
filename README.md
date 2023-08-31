# flow
This started as a rewrite of [riverwm-utils](https://github.com/NickHastings/riverwm-utils) in Rust (very cliché, I know). Then I decided to add a few other features as well. See below.

## Features
- Cycle the focused tags, either to next or previous tag.
- Toggle tags. If chosen tag is already focused, toggle to previous tag instead.
- Move directly to urgent tags on an output.

## Installation from source
1. Make sure you've got Rust installed. Either via your distributions package manager or [`rustup`](https://rustup.rs/).
2. `cargo install --git https://github.com/stefur/flow flow`

## Usage
Currently the following commands can be used with flow.

| Command | Arguments | Description | Example |
| --- | --- | --- | --- |
| `cycle-tags` | Direction: `next` or `previous`. Number of available tags: `int`, defaults to `9` if omitted. | Move focused tag to the next or previous tag. | `flow cycle-tags next 6` |
| `toggle-tags` | Tagmask to focus. | Attempt to focus selected tagmask. Toggle previous tags if already focused. | `flow toggle-tags 64` |
| `focus-urgent-tags` | None. | Focus urgent tags on an output. | `flow focus-urgent-tags` |