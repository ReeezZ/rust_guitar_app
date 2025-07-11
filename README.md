# Rust Guitar App

This project is currently mostly for learning Rust and Leptos.

The idea is to create a guitar learning companion app for learning and practicing on the guitar.

There are many guitar practice apps, but so far I know of none that are truly free and open source. So I want to build something like that.

---

## Current Status: Early Phase

This project is in an early phase and is a bit chaotic currently. Currently this is more a tech demo to myself to try various approaches.

---

## Prerequisites

### For VS Code Users

1. **Install VS Code**:
- Download and install [Visual Studio Code](https://code.visualstudio.com/).

2. **Install the Dev Containers Extension**:
- Open VS Code.
- Go to the Extensions view (`Ctrl+Shift+X` or `Cmd+Shift+X` on macOS).
- Search for "Dev Containers" and install the extension.

3. **Install Docker**:
- Download and install [Docker Desktop](https://www.docker.com/products/docker-desktop/).
- Ensure Docker is running.

4. **Clone the Repository**:
```sh
git clone https://github.com/your-username/leptos_stuff.git
cd leptos_stuff
```

5. **Open the Project in VS Code:**
- Open the project folder in VS Code.

6. **Reopen in Devcontainer:**

- Press Ctrl+Shift+P (or Cmd+Shift+P on macOS) to open the Command Palette.
- Select "Dev Containers: Rebuild and Reopen in Container".
- Wait for the container to build and start.

7. **(Optional) Verify the Setup:**

- Open a terminal in VS Code (`Ctrl+ or Cmd+ on macOS).
- Run the following commands to verify the setup:

```sh
rustc --version
cargo --version
trunk --version
```


8. **Run the Project**:

- Use Trunk to serve the project:
  - `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --open`


### For Non-VS Code Users

This project is designed to work with VS Code's devcontainer setup. If you're not using VS Code, you can still set up the environment manually:

1. **Install Rust**:
    
- Install Rust using [rustup](vscode-file://vscode-app/c:/Users/mario/AppData/Local/Programs/Microsoft%20VS%20Code/resources/app/out/vs/code/electron-sandbox/workbench/workbench.html).
- Add the WebAssembly target:
  
  rustup target add wasm32-unknown-unknown
        
2. **Install Trunk**:

- Install Trunk, the WASM bundler:  
  - `cargo install trunk`
        
3. **Install Leptosfmt**:
- Install Leptosfmt for formatting Leptos macros:
  - `cargo install leptosfmt`
        
4. **Install Node.js**:
   
- Install Node.js (LTS version) from [nodejs.org](vscode-file://vscode-app/c:/Users/mario/AppData/Local/Programs/Microsoft%20VS%20Code/resources/app/out/vs/code/electron-sandbox/workbench/workbench.html).


5. **Install Tailwind CSS**:

- Install Tailwind CSS globally:
  - `npm install -g tailwindcss`
    
2. **Run the Project**:

- Use Trunk to serve the project:
  - `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' trunk serve --open`
        

Note: You may need to manually configure caching for Rust dependencies and build artifacts.

---

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
[Fretonomy](https://www.fretonomy.com/)

## Tech Stack Ideas

https://github.com/new-data-services/tailwindcss-animated


---

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

---

## License

This project is licensed under GNU General Public License v3.0

[LICENSE file](./LICENSE)

