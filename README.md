# Night Light

A small Flappy Bird-inspired game built with Rust and Bevy as a learning project.

## Overview

Night Light is a simple arcade game where the player controls a glowing orb that flies upward with each tap/press, while gravity pulls it back down. The goal is to weave through gaps in oncoming obstacles.

This project was created to learn Rust and Bevy while exploring game development fundamentals such as:

- entity/component architecture
- game loops and systems
- input handling
- basic 2D rendering
- physics-like movement

## Personal milestone

- created the game window using Bevy
- spawned a character as a circle on screen
- made the character move with simple physics
- added keyboard input so the player can jump with Space

## Gameplay

- Press Space to jump upward
- Avoid the pips
- Survive as long as possible

Basically flappy bird

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

## Todos
- obstacle generation
- collision detection
- score tracking
- start/game-over screens
- sound effects and polish
