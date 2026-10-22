# Night Light

A small Flappy Bird-inspired game built with Rust and Bevy as a learning project.

## Overview

Night Light is a simple arcade game where the player controls a glowing orb that flies upward with each tap/press, while gravity pulls it back down. The goal is to weave through gaps in oncoming obstacles without crashing.

This project was created to learn Rust and Bevy while exploring game development fundamentals such as:

- entity/component architecture
- game loops and systems
- input handling
- basic 2D rendering
- physics-like movement

## Gameplay

- Press Space to flap upward
- Avoid the obstacles
- Survive as long as possible

## Requirements

- Rust and Cargo installed
- A system capable of running Bevy applications

## Run the game

```bash
git clone https://github.com/Caj64bits/night-light.git
cd night-light
cargo run
```

## Project structure

```text
night-light/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── player.rs
├── Cargo.lock
└── README.md
```

## Dependencies

- Rust
- Bevy 0.20.0

## Notes

This is a lightweight learning project and is intentionally simple. It is a great starting point for experimenting with Bevy and expanding the game with features like:

- obstacle generation
- collision detection
- score tracking
- start/game-over screens
- sound effects and polish

## License

This project does not currently include a license file.
