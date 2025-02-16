# Rust guitar app


## Prerequisites

```
rustup target add wasm32-unknown-unknown

cargo install trunk
cargo install leptosfmt
```

You might have to downgrade the Tailwind CSS IntelliSense VS Code plugin to v0.10.5. See: https://github.com/tailwindlabs/tailwindcss-intellisense/issues/988

## Usage

`trunk serve --open`


# Pitch

I play guitar and I want to learn rust. There are many guitar practice apps, but so far I know of none that are truly free and open source. So i want to build something like that.

This project is mostly for learning. Using rust, creating a GUI with rust.

## Scope

For now i want to display the circle of fifths. 

The first real learning exercise would be to have to fill an empty circle of fifths.

Increasing difficulty could be that the CoF has to be filled out in a given order.

### Future Ideas

- Tuner
- Metronome
- Fretboard trainer

(*cough* scope overload)

## Challenges

A big challenge will probably be to have a GUI that is multi platform. I'd like to have the app portable so it can be used on mobile and PC. 

## Comparisons

TODO link other comparable software

## Possible inspirations

[Code pen fretboard example](https://codepen.io/DreySkee/pen/bddpqM)
[JS fretboard](https://github.com/metaescape/js-fretboard)

https://www.youtube.com/watch?v=C6VLedW5Dwk&list=PLXAhCH9FJ8zViqdqhsSP7iyCrVDoUGb3P&index=2


## Brainstroming

- Circle of Fifths trainer
  - Noten/Tonarten auf den Kreis ziehen
- Fretboard trainer
  - Find notes on the fretboard
  - Show scales
    - Triads
- Ear training
- Tuner
- Metronome
  - Maybe a simple drum machine
- Knowledge base
  - Chords
- Some info about achords in a key?
