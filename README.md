# Rust guitar app

This project is currently mostly for learning rust and leptos.

The idea is to create a guitar learning companion app. For learning and exercising on the guitar.

There are many guitar practice apps, but so far I know of none that are truly free and open source. So i want to build something like that.


## Current status: Early phase

This project is in an early phase and is a bit chaotic currently.

### Current Problems / TODOs

- The git branches are a bit chaotic
  - doing multiple different things on different feature branches
  - gitlab issues do not map to branches clearly
  - I will realign all this rather soon hopefully
- Dead code warnings
  - I wanted to overengineer a music library and not everything is used yet 
  - The warnings hurt me too, trust me



## Prerequisites

```sh
rustup target add wasm32-unknown-unknown

# Trunk is a WASM web application bundler for Rust
cargo install trunk
# For formatting the leptos view! macro
cargo install leptosfmt
```

Tailwind CSS IntelliSense VS Code plugin problem: You might have to downgrade to v0.10.5. See: https://github.com/tailwindlabs/tailwindcss-intellisense/issues/988

## Usage

`trunk serve --open`


## Feature Ideas and Status

### Fretboard map

Feature that is currently being worked on.

Ideas for execrises / features:

- Find notes on the fretboard
- find interval of note
- Show scales
  - Triads
  - Modes

### Circle of Fifths


Display the circle of fifths. 
The first real learning exercise would be to have to fill an empty circle of fifths.
Being given scales and having to drag them in the right spot on the circle of fifths.
Increasing difficulty could be that the CoF has to be filled out in a given order.

Drawing this with CSS is probably very tricky. Using a SVG would be probably good.

### Future Ideas

- Tuner 
  - Audio input?
- Metronome
  - Timing issues?
  - Being able to configure a simple drum beat
- Ear training
  - Have to look into playing audio
- Chord book
  - indication on how well the chord is known
- Chords in a key
- MIDI / Keyboard input for playing notes
  - Like when a scale is selected using keys 1 to 7 to play scale degress

(*cough* scope creep)

## Comparisons / Possible inspirations

[Code pen fretboard example](https://codepen.io/DreySkee/pen/bddpqM)
[JS fretboard](https://github.com/metaescape/js-fretboard)
[fretmap](https://fretmap.app/)

[Fretboard with css tutorial](https://www.youtube.com/watch?v=C6VLedW5Dwk&list=PLXAhCH9FJ8zViqdqhsSP7iyCrVDoUGb3P&index=2)

## Tech Stack Ideas

https://github.com/new-data-services/tailwindcss-animated

