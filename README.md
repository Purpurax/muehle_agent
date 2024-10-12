# MÜHLE AI

`muehle_agent` is a Rust-based implementation of the classic board game Nine Men's Morris (Mühle in German), featuring an AI opponent powered by a Minimax algorithm with Alpha-Beta pruning. Players can choose different difficulty levels (easy, medium, hard) and can even watch two AIs compete against each other. The game can be played locally or through the web using WebAssembly.

## Features

- **Play Nine Men's Morris:** Enjoy the classic board game against an AI opponent or watch AI vs. AI matches.
- **Adjustable Difficulty:** Choose between easy, medium, and hard difficulty levels.
- **AI Opponent:** The AI uses a Minimax algorithm with Alpha-Beta pruning for efficient decision-making.
- **Cross-Platform:** The game can run locally on a desktop or as a WebAssembly application in the browser.
- **More Information:** [purpurax.de](https://purpurax.de/muehle/)

## Requirements

- **Rust and Cargo:** Make sure you have Cargo installed via [rustup](https://rustup.rs/).
- **Web Server for WebAssembly:**
  - Option 1: Install `basic-http-server` using Cargo: `cargo install basic-http-server`
  - Option 2 (recommended): Install `http-server` using npm: `npm install http-server`

## WebAssembly Setup

1. **Prepare the Assets:**
   - Archive the assets folder into a file named `assets.tar`:
     ```
     tar -cvf assets.tar assets
     ```
   - On Windows, you can use the above command in a terminal that supports `tar`, such as Git Bash.

2. **Add the WebAssembly Target:**
```rustup target add wasm32-unknown-unknown```

3. **Build the Project:**
```cargo build --target wasm32-unknown-unknown --release```

If you do not use the `--release` flag, make sure the `index.html` file is set to load the `.wasm` file from the correct build directory.

4. **Start the Web Server:**
- Using `basic-http-server` from `cargo install basic-http-server`:
  ```
  basic-http-server .
  ```
- Or using `http-server` from `npm install http-server`:
  ```
  http-server
  ```

## Run Locally

To run the game as a standard desktop application simply use:
```cargo run --release```
