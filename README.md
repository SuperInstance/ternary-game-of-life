# ternary-game-of-life

**Conway's Game of Life, reimagined with ternary states {-1, 0, +1}.**

[![Tests](https://img.shields.io/badge/tests-26%20passing-brightgreen)]()

## Why?

Conway's Game of Life is the canonical introduction to cellular automata. But
binary alive/dead is just the beginning. By adding a **dormant** state — a cell
that's neither fully alive nor fully dead — we create richer dynamics that
mirror the ternary mathematics underlying agent coordination systems.

This makes ternary state machines **playable**. Students can watch {-1, 0, +1}
compete, cooperate, and evolve on a grid — building intuition for multi-state
automata used in real distributed systems.

## The Three States

| State    | Symbol | Trit | Behavior                                    |
|----------|--------|------|---------------------------------------------|
| Dead     | ·      | -1   | Can be stirred to Dormant by neighbors      |
| Dormant  | ○      | 0    | Can be activated to Alive by 3 alive nbrs   |
| Alive    | ●      | +1   | Dies from isolation or overcrowding         |

## Ternary Rules

### Alive (+1) cells:
- **Die** (→ Dead) if fewer than 2 alive neighbors (isolation)
- **Die** (→ Dead) if more than 3 alive neighbors (overcrowding)
- **Survive** with 2–3 alive neighbors

### Dormant (0) cells:
- **Activate** (→ Alive) if exactly 3 alive neighbors
- **Stay Dormant** otherwise (stirred by dormant+alive combos)

### Dead (-1) cells:
- **Stir** (→ Dormant) if exactly 3 alive neighbors (near-miss)
- **Stir** (→ Dormant) if 2 alive + 3+ dormant neighbors (crowd stirring)
- **Stay Dead** otherwise

The key insight: **it takes two steps for a Dead cell to become Alive**
(Dead → Dormant → Alive), creating a natural "warm-up" period that makes
dynamics more interesting than binary Life.

## Quick Start

```rust
use ternary_game_of_life::{TernaryGrid, CellState};

// Create a 10x10 grid
let mut grid = TernaryGrid::new(10, 10);

// Place a 2x2 block (still life)
grid.set(3, 3, CellState::Alive);
grid.set(4, 3, CellState::Alive);
grid.set(3, 4, CellState::Alive);
grid.set(4, 4, CellState::Alive);

// Check if it's stable
assert!(grid.is_still_life());

// Advance one step
let next = grid.step();

// Run 10 steps
let history = grid.steps(10);
```

## Features

### Step Simulation
Advance the grid forward with full ternary rules:
```rust
let next = grid.step();
let history = grid.steps(20);
```

### Oscillator Detection
Find patterns that repeat with a given period:
```rust
if let Some(period) = grid.detect_oscillator() {
    println!("Oscillates with period {}", period);
}
```

### Still-Life Detection
Identify stable subregions:
```rust
let still_lives = grid.find_still_lives(3); // search 3x3 regions
```

### Glider Detection
Detect patterns that translate across the grid:
```rust
let movement = TernaryGrid::detect_glider(&initial, &after_steps);
if let Some((dx, dy)) = movement {
    println!("Pattern moved by ({}, {})", dx, dy);
}
```

### Population Statistics
Track how the three populations evolve:
```rust
let stats = grid.population();
println!("Alive: {}  Dormant: {}  Dead: {}", 
    stats.alive, stats.dormant, stats.dead);
println!("Activity ratio: {:.1}%", stats.activity_ratio() * 100.0);
```

### Rendering
Visualize the grid with Unicode symbols:
```rust
println!("{}", grid.render());
// ··●··
// ··●··
// ··●··
```

## Educational Value

Part of the **Loom** educational platform for making agent coordination
accessible. Ternary Game of Life teaches:

1. **Multi-state automata** — not just binary, but ternary state machines
2. **Emergent behavior** — complex patterns from simple rules
3. **Population dynamics** — tracking ratios, growth, decay
4. **Pattern recognition** — oscillators, still-lives, gliders
5. **Ternary transitions** — the two-step Dead→Dormant→Alive path mirrors
   real-world system warmup and cooldown

## Differences from Classic Life

| Feature              | Classic Life          | Ternary Life                  |
|----------------------|-----------------------|-------------------------------|
| States               | 2 (alive, dead)       | 3 (alive, dormant, dead)      |
| Birth rule           | 3 neighbors → alive   | 3 nbrs → dormant (then alive) |
| Warm-up period       | None                  | Two-step activation           |
| Dormant state        | N/A                   | Can be stirred by neighbors   |
| Pattern diversity    | Binary patterns       | Richer ternary dynamics       |

## API Overview

| Type             | Description                          |
|------------------|--------------------------------------|
| `CellState`      | Dead, Dormant, or Alive              |
| `TernaryGrid`    | 2D grid with ternary Life rules      |
| `PopulationStats`| Counts and ratios for each state     |

## License

MIT
