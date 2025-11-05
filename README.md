# Dungeon of the Fallen King

A classic roguelike game written in Rust using Test-Driven Development (TDD).

## About

This is a traditional roguelike dungeon crawler where you descend into the cursed dungeons beneath the Fallen King's castle. Fight monsters, survive as long as you can, and see how deep you can go!

### What is a Roguelike?

A roguelike is a genre of dungeon-crawling video game characterized by:

- **Procedurally generated levels** - Each playthrough creates a new, randomized dungeon
- **Permadeath** - When your character dies, you start over from the beginning
- **Turn-based gameplay** - The game only advances when you take an action
- **Grid-based movement** - Player and entities move on a tile-based grid
- **ASCII graphics** - Uses text characters to represent the game world
- **Tactical combat** - Strategic decision-making in encounters

## Features

### Current Features

- **Procedural dungeon generation** using cellular automata algorithm
- **Turn-based combat** with bump-to-attack mechanics
- **Multiple enemy types**:
  - `g` - Goblin (weak, fast)
  - `o` - Orc (medium strength)
  - `T` - Troll (strong, dangerous)
- **Simple AI** - Enemies chase and attack the player when nearby
- **Health system** with HP tracking
- **Combat messages** - See what's happening in the dungeon
- **8-directional movement** - Full freedom of movement
- **Smooth terminal UI** using crossterm

### Implemented via TDD

This project was built using Test-Driven Development with 22 passing tests covering:

- Position calculations and distance metrics
- Tile properties (walkability, line of sight blocking)
- Map generation and bounds checking
- Entity creation and damage/healing
- Game state management
- Combat resolution

## How to Play

### Installation

```bash
cargo build --release --example play
```

### Running the Game

```bash
cargo run --example play
```

### Controls

**Movement:**
- Arrow keys - Move in 4 directions
- `hjkl` (vi-keys) - Move in 4 directions
- `yubn` - Diagonal movement
- Numpad (1-9) - Full 8-directional movement
- `.` or `5` - Wait/skip turn

**Other:**
- `q` or `Esc` - Quit game

### Objective

Survive as long as possible in the dungeon! Fight monsters, avoid death, and rack up kills. The game ends when your HP reaches 0.

## Project Structure

```
src/
  lib.rs           - Library entry point
  roguelike.rs     - Complete roguelike implementation
    ├─ Position    - 2D position with distance calculations
    ├─ Tile        - Dungeon tiles (Wall, Floor, StairsDown)
    ├─ Map         - 2D grid map with procedural generation
    ├─ Entity      - Player and monster entities
    └─ GameState   - Game logic, combat, and AI

examples/
  play.rs          - Playable terminal game
```

## Development

### Running Tests

```bash
cargo test
```

All 22 tests should pass, covering:
- Position structure and methods
- Tile properties
- Map creation and operations
- Dungeon generation
- Entity management
- Combat system
- Game state

### Building

```bash
cargo build
```

## Technical Details

### Algorithms Used

**Cellular Automata (Dungeon Generation)**
- Starts with random noise (45% floor tiles)
- Applies smoothing rules 4 times
- Creates organic cave-like dungeons
- Ensures borders are always walls

**Chebyshev Distance**
- Used for 8-directional movement distance
- Formula: max(|x1 - x2|, |y1 - y2|)
- Perfect for roguelike grid-based movement

**Simple Chase AI**
- Enemies detect player within 10 tiles
- Move directly towards player position
- Attack when adjacent
- Turn-based: enemies move after player

### Dependencies

- `rand` (0.8) - Random number generation for procedural content
- `crossterm` (0.27) - Cross-platform terminal manipulation

## License

GPL-3.0+

Copyright 2016 André-Patrick Bubel.

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

## Acknowledgments

Named after the classic 1980 game "Rogue", which defined the genre.

Built with Test-Driven Development methodology, writing tests first before implementation.
