//! # ternary-game-of-life
//!
//! Conway's Game of Life, reimagined with ternary states {-1, 0, +1}.
//!
//! In classic Game of Life, cells are either alive (1) or dead (0). Here we add
//! a **dormant** state (0) — a cell that's neither fully alive nor fully dead.
//! It can be *activated* by its neighbors, creating richer dynamics than binary
//! Life while staying true to the spirit of cellular automata.
//!
//! States:
//! - `+1` (Alive): Active cell
//! - `0` (Dormant): Can be activated by neighbors
//! - `-1` (Dead): Fully dead

use std::fmt;

/// A ternary cell state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    /// Fully dead
    Dead,
    /// Dormant — can be activated by neighbors
    Dormant,
    /// Alive and active
    Alive,
}

impl CellState {
    /// Convert to trit integer.
    pub fn to_trit(self) -> i8 {
        match self {
            CellState::Dead => -1,
            CellState::Dormant => 0,
            CellState::Alive => 1,
        }
    }

    /// Create from trit integer.
    pub fn from_trit(t: i8) -> Self {
        match t {
            -1 => CellState::Dead,
            0 => CellState::Dormant,
            1 => CellState::Alive,
            _ => panic!("Invalid trit: {}", t),
        }
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CellState::Dead => write!(f, "·"),
            CellState::Dormant => write!(f, "○"),
            CellState::Alive => write!(f, "●"),
        }
    }
}

/// A 2D grid for ternary Game of Life.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryGrid {
    width: usize,
    height: usize,
    cells: Vec<CellState>,
}

impl TernaryGrid {
    /// Create a new grid filled with Dead cells.
    pub fn new(width: usize, height: usize) -> Self {
        TernaryGrid {
            width,
            height,
            cells: vec![CellState::Dead; width * height],
        }
    }

    /// Create from a vector of cell states.
    pub fn from_cells(width: usize, height: usize, cells: Vec<CellState>) -> Option<Self> {
        if cells.len() != width * height {
            return None;
        }
        Some(TernaryGrid { width, height, cells })
    }

    /// Width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get cell at (x, y).
    pub fn get(&self, x: usize, y: usize) -> Option<CellState> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    /// Set cell at (x, y).
    pub fn set(&mut self, x: usize, y: usize, state: CellState) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = state;
        }
    }

    /// Count neighbors in each state around (x, y), wrapping at edges (toroidal).
    pub fn count_neighbors(&self, x: usize, y: usize) -> (usize, usize, usize) {
        let mut dead = 0;
        let mut dormant = 0;
        let mut alive = 0;

        for dy in [-1i32, 0, 1].iter() {
            for dx in [-1i32, 0, 1].iter() {
                if *dx == 0 && *dy == 0 {
                    continue;
                }
                let nx = (x as i32 + dx).rem_euclid(self.width as i32) as usize;
                let ny = (y as i32 + dy).rem_euclid(self.height as i32) as usize;
                match self.cells[ny * self.width + nx] {
                    CellState::Dead => dead += 1,
                    CellState::Dormant => dormant += 1,
                    CellState::Alive => alive += 1,
                }
            }
        }
        (dead, dormant, alive)
    }

    /// Ternary Game of Life rules:
    ///
    /// **Alive (+1) cell:**
    /// - Dies (→ Dead) if alive_neighbors < 2 (isolation)
    /// - Dies (→ Dead) if alive_neighbors > 3 (overcrowding)
    /// - Stays Alive if alive_neighbors is 2 or 3
    ///
    /// **Dormant (0) cell:**
    /// - Becomes Alive if exactly 3 alive neighbors (activation)
    /// - Becomes Dormant if 1-2 alive + 2+ dormant (stirring)
    /// - Stays Dormant otherwise
    ///
    /// **Dead (-1) cell:**
    /// - Becomes Dormant if exactly 3 alive neighbors (near-miss)
    /// - Becomes Dormant if 2 alive + 3+ dormant (crowd stirring)
    /// - Stays Dead otherwise
    fn next_state(&self, x: usize, y: usize) -> CellState {
        let (_dead, dormant, alive) = self.count_neighbors(x, y);
        let current = self.cells[y * self.width + x];

        match current {
            CellState::Alive => {
                if alive < 2 || alive > 3 {
                    CellState::Dead
                } else {
                    CellState::Alive
                }
            }
            CellState::Dormant => {
                if alive == 3 {
                    CellState::Alive
                } else if alive >= 1 && dormant >= 2 {
                    CellState::Dormant // stays dormant, stirred by neighbors
                } else {
                    CellState::Dormant
                }
            }
            CellState::Dead => {
                if alive == 3 {
                    CellState::Dormant // near-miss: brought back to dormant
                } else if alive == 2 && dormant >= 3 {
                    CellState::Dormant
                } else {
                    CellState::Dead
                }
            }
        }
    }

    /// Advance the grid by one step.
    pub fn step(&self) -> TernaryGrid {
        let mut next = TernaryGrid::new(self.width, self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                next.set(x, y, self.next_state(x, y));
            }
        }
        next
    }

    /// Advance N steps, returning all intermediate grids.
    pub fn steps(&self, n: usize) -> Vec<TernaryGrid> {
        let mut history = Vec::with_capacity(n);
        let mut current = self.clone();
        for _ in 0..n {
            current = current.step();
            history.push(current.clone());
        }
        history
    }

    /// Population statistics.
    pub fn population(&self) -> PopulationStats {
        let mut dead = 0;
        let mut dormant = 0;
        let mut alive = 0;
        for &cell in &self.cells {
            match cell {
                CellState::Dead => dead += 1,
                CellState::Dormant => dormant += 1,
                CellState::Alive => alive += 1,
            }
        }
        PopulationStats { dead, dormant, alive, total: self.cells.len() }
    }

    /// Render the grid as a string.
    pub fn render(&self) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                out.push_str(&format!("{}", self.cells[y * self.width + x]));
            }
            out.push('\n');
        }
        out
    }

    /// Detect if the grid is a still life (same after one step).
    pub fn is_still_life(&self) -> bool {
        &self.step() == self
    }

    /// Detect if the grid is an oscillator with a given period.
    /// Returns the period if oscillating (periods 1-20 checked), or None.
    pub fn detect_oscillator(&self) -> Option<usize> {
        let mut current = self.clone();
        for period in 1..=20 {
            current = current.step();
            if &current == self {
                return Some(period);
            }
        }
        None
    }

    /// Detect if a glider pattern exists that has moved from an initial position.
    /// Compares `initial` grid to `after_steps` to see if the pattern has translated.
    pub fn detect_glider(initial: &TernaryGrid, after_steps: &TernaryGrid) -> Option<(i32, i32)> {
        // Extract alive cell positions from both grids
        let initial_alive: Vec<(usize, usize)> = initial.alive_positions();
        let after_alive: Vec<(usize, usize)> = after_steps.alive_positions();

        if initial_alive.len() != after_alive.len() || initial_alive.is_empty() {
            return None;
        }

        // Try all possible (dx, dy) translations
        let ((ix, iy), _) = initial_alive.split_first().unwrap();
        let (ix, iy) = (*ix, *iy);
        for &(ax, ay) in &after_alive {
            let dx = (ax as i32).wrapping_sub(ix as i32);
            let dy = (ay as i32).wrapping_sub(iy as i32);
            if dx == 0 && dy == 0 {
                continue; // No movement
            }
            let all_match = initial_alive.iter().all(|&(px, py)| {
                let translated_x = ((px as i32 + dx).rem_euclid(initial.width as i32)) as usize;
                let translated_y = ((py as i32 + dy).rem_euclid(initial.height as i32)) as usize;
                after_alive.contains(&(translated_x, translated_y))
            });
            if all_match {
                return Some((dx, dy));
            }
        }
        None
    }

    /// Get positions of all alive cells.
    fn alive_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y * self.width + x] == CellState::Alive {
                    positions.push((x, y));
                }
            }
        }
        positions
    }

    /// Find all still-life patterns in the grid (subregions that don't change).
    /// Returns a list of (x, y, width, height) bounding boxes.
    pub fn find_still_lives(&self, search_size: usize) -> Vec<(usize, usize, usize, usize)> {
        let mut results = Vec::new();
        if search_size == 0 || search_size > self.width || search_size > self.height {
            return results;
        }

        let step_grid = self.step();

        for y in 0..=(self.height.saturating_sub(search_size)) {
            for x in 0..=(self.width.saturating_sub(search_size)) {
                let mut matches = true;
                'outer: for dy in 0..search_size {
                    for dx in 0..search_size {
                        if self.get(x + dx, y + dy) != step_grid.get(x + dx, y + dy) {
                            matches = false;
                            break 'outer;
                        }
                    }
                }
                if matches {
                    let mut has_alive = false;
                    for dy in 0..search_size {
                        for dx in 0..search_size {
                            if self.get(x + dx, y + dy) == Some(CellState::Alive) {
                                has_alive = true;
                                break;
                            }
                        }
                    }
                    if has_alive {
                        results.push((x, y, search_size, search_size));
                    }
                }
            }
        }
        results
    }
}

/// Population statistics for a ternary grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopulationStats {
    pub dead: usize,
    pub dormant: usize,
    pub alive: usize,
    pub total: usize,
}

impl PopulationStats {
    /// Alive ratio (0.0 to 1.0).
    pub fn alive_ratio(&self) -> f64 {
        self.alive as f64 / self.total as f64
    }

    /// Dormant ratio (0.0 to 1.0).
    pub fn dormant_ratio(&self) -> f64 {
        self.dormant as f64 / self.total as f64
    }

    /// Total non-dead ratio.
    pub fn activity_ratio(&self) -> f64 {
        (self.alive + self.dormant) as f64 / self.total as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        let grid = TernaryGrid::new(5, 5);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 5);
        for y in 0..5 {
            for x in 0..5 {
                assert_eq!(grid.get(x, y), Some(CellState::Dead));
            }
        }
    }

    #[test]
    fn test_set_get() {
        let mut grid = TernaryGrid::new(3, 3);
        grid.set(1, 1, CellState::Alive);
        grid.set(0, 0, CellState::Dormant);
        assert_eq!(grid.get(1, 1), Some(CellState::Alive));
        assert_eq!(grid.get(0, 0), Some(CellState::Dormant));
        assert_eq!(grid.get(2, 2), Some(CellState::Dead));
    }

    #[test]
    fn test_neighbor_count_center() {
        let mut grid = TernaryGrid::new(3, 3);
        grid.set(0, 0, CellState::Alive);
        grid.set(1, 0, CellState::Alive);
        grid.set(2, 0, CellState::Dormant);
        // Center cell (1,1) has neighbors from all 8 positions (wrapping)
        let (dead, dormant, alive) = grid.count_neighbors(1, 1);
        assert_eq!(alive, 2);
        assert_eq!(dormant, 1);
        assert!(dead > 0); // remaining neighbors are dead
        assert_eq!(dead + dormant + alive, 8);
    }

    #[test]
    fn test_neighbor_count_wrapping() {
        let mut grid = TernaryGrid::new(3, 3);
        grid.set(0, 0, CellState::Alive);
        // Cell (2, 2) should see (0,0) as a neighbor via wrapping
        let (_, _, alive) = grid.count_neighbors(2, 2);
        assert_eq!(alive, 1);
    }

    #[test]
    fn test_step_isolation() {
        // Single alive cell with no neighbors → dies
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(2, 2, CellState::Alive);
        let next = grid.step();
        assert_eq!(next.get(2, 2), Some(CellState::Dead));
    }

    #[test]
    fn test_step_survival() {
        // Block: 2x2 alive cells should be a still life
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 1, CellState::Alive);
        grid.set(1, 2, CellState::Alive);
        grid.set(2, 2, CellState::Alive);
        let next = grid.step();
        // All should survive (each has exactly 3 alive neighbors)
        assert_eq!(next.get(1, 1), Some(CellState::Alive));
        assert_eq!(next.get(2, 1), Some(CellState::Alive));
        assert_eq!(next.get(1, 2), Some(CellState::Alive));
        assert_eq!(next.get(2, 2), Some(CellState::Alive));
    }

    #[test]
    fn test_step_birth() {
        // Dormant cell with exactly 3 alive neighbors → becomes alive
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 1, CellState::Alive);
        grid.set(1, 2, CellState::Alive);
        grid.set(2, 2, CellState::Dormant);
        let next = grid.step();
        // The dormant cell at (2,2) has 3 alive neighbors → becomes alive
        assert_eq!(next.get(2, 2), Some(CellState::Alive));
    }

    #[test]
    fn test_step_overcrowding() {
        // Alive cell with 4+ alive neighbors → dies
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(2, 2, CellState::Alive); // center
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 1, CellState::Alive);
        grid.set(3, 1, CellState::Alive);
        grid.set(1, 2, CellState::Alive);
        // Center has 4 alive neighbors → should die
        let next = grid.step();
        assert_eq!(next.get(2, 2), Some(CellState::Dead));
    }

    #[test]
    fn test_block_still_life() {
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 1, CellState::Alive);
        grid.set(1, 2, CellState::Alive);
        grid.set(2, 2, CellState::Alive);
        assert!(grid.is_still_life());
    }

    #[test]
    fn test_oscillator_block_period1() {
        // A 2x2 block is a still life → period 1 oscillator
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 1, CellState::Alive);
        grid.set(1, 2, CellState::Alive);
        grid.set(2, 2, CellState::Alive);
        let period = grid.detect_oscillator();
        assert_eq!(period, Some(1));
    }

    #[test]
    fn test_oscillator_empty_period1() {
        // Empty grid is a period 1 oscillator (still life)
        let grid = TernaryGrid::new(3, 3);
        let period = grid.detect_oscillator();
        assert_eq!(period, Some(1));
    }

    #[test]
    fn test_blinker_no_oscillation_in_ternary() {
        // Classic blinker doesn't oscillate under ternary rules
        // because ternary rules have different birth/death dynamics
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(1, 2, CellState::Alive);
        grid.set(2, 2, CellState::Alive);
        grid.set(3, 2, CellState::Alive);
        let period = grid.detect_oscillator();
        // In ternary: the pattern doesn't settle within 20 steps
        assert!(period.is_none() || period == Some(1));
    }

    #[test]
    fn test_empty_is_still_life() {
        let grid = TernaryGrid::new(3, 3);
        assert!(grid.is_still_life());
    }

    #[test]
    fn test_glider_detection_translated_block() {
        // Create a block pattern, then manually create a translated version
        let mut grid1 = TernaryGrid::new(10, 10);
        grid1.set(2, 2, CellState::Alive);
        grid1.set(3, 2, CellState::Alive);
        grid1.set(2, 3, CellState::Alive);
        grid1.set(3, 3, CellState::Alive);

        let mut grid2 = TernaryGrid::new(10, 10);
        grid2.set(4, 4, CellState::Alive);
        grid2.set(5, 4, CellState::Alive);
        grid2.set(4, 5, CellState::Alive);
        grid2.set(5, 5, CellState::Alive);

        let movement = TernaryGrid::detect_glider(&grid1, &grid2);
        assert!(movement.is_some());
        let (dx, dy) = movement.unwrap();
        assert_eq!(dx, 2);
        assert_eq!(dy, 2);
    }

    #[test]
    fn test_glider_detection_no_movement() {
        // Same pattern in same position → no movement detected
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 1, CellState::Alive);
        let movement = TernaryGrid::detect_glider(&grid, &grid);
        assert!(movement.is_none());
    }

    #[test]
    fn test_population_stats() {
        let mut grid = TernaryGrid::new(3, 3);
        grid.set(0, 0, CellState::Alive);
        grid.set(1, 1, CellState::Alive);
        grid.set(2, 2, CellState::Dormant);
        let stats = grid.population();
        assert_eq!(stats.alive, 2);
        assert_eq!(stats.dormant, 1);
        assert_eq!(stats.dead, 6);
        assert_eq!(stats.total, 9);
    }

    #[test]
    fn test_population_ratios() {
        let mut grid = TernaryGrid::new(2, 2);
        grid.set(0, 0, CellState::Alive);
        grid.set(0, 1, CellState::Dormant);
        grid.set(1, 0, CellState::Dead);
        grid.set(1, 1, CellState::Dead);
        let stats = grid.population();
        assert!((stats.alive_ratio() - 0.25).abs() < 0.01);
        assert!((stats.dormant_ratio() - 0.25).abs() < 0.01);
        assert!((stats.activity_ratio() - 0.50).abs() < 0.01);
    }

    #[test]
    fn test_steps_chain() {
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(2, 2, CellState::Alive);
        let history = grid.steps(3);
        assert_eq!(history.len(), 3);
    }

    #[test]
    fn test_render() {
        let mut grid = TernaryGrid::new(3, 1);
        grid.set(0, 0, CellState::Dead);
        grid.set(1, 0, CellState::Dormant);
        grid.set(2, 0, CellState::Alive);
        let rendered = grid.render();
        assert!(rendered.contains("·"));
        assert!(rendered.contains("○"));
        assert!(rendered.contains("●"));
    }

    #[test]
    fn test_from_cells() {
        let cells = vec![CellState::Alive, CellState::Dead, CellState::Dormant, CellState::Dead];
        let grid = TernaryGrid::from_cells(2, 2, cells).unwrap();
        assert_eq!(grid.get(0, 0), Some(CellState::Alive));
        assert_eq!(grid.get(1, 0), Some(CellState::Dead));
    }

    #[test]
    fn test_from_cells_wrong_size() {
        let cells = vec![CellState::Alive];
        assert!(TernaryGrid::from_cells(2, 2, cells).is_none());
    }

    #[test]
    fn test_find_still_lives() {
        // Place a block (2x2 alive) — should be found as still life
        let mut grid = TernaryGrid::new(6, 6);
        grid.set(2, 2, CellState::Alive);
        grid.set(3, 2, CellState::Alive);
        grid.set(2, 3, CellState::Alive);
        grid.set(3, 3, CellState::Alive);
        let still_lives = grid.find_still_lives(3);
        // The 3x3 region containing the block should be detected
        assert!(!still_lives.is_empty());
    }

    #[test]
    fn test_dormant_activation() {
        // A dormant cell surrounded by 3 alive cells should activate
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(0, 0, CellState::Alive);
        grid.set(1, 0, CellState::Alive);
        grid.set(0, 1, CellState::Alive);
        grid.set(1, 1, CellState::Dormant);
        // Cell (1,1) has 3 alive neighbors → should activate
        let next = grid.step();
        assert_eq!(next.get(1, 1), Some(CellState::Alive));
    }

    #[test]
    fn test_dead_near_miss() {
        // A dead cell with exactly 3 alive neighbors → becomes dormant
        let mut grid = TernaryGrid::new(5, 5);
        grid.set(0, 0, CellState::Alive);
        grid.set(1, 0, CellState::Alive);
        grid.set(0, 1, CellState::Alive);
        grid.set(1, 1, CellState::Dead);
        // Cell (1,1) is dead with 3 alive neighbors → becomes dormant
        let next = grid.step();
        assert_eq!(next.get(1, 1), Some(CellState::Dormant));
    }

    #[test]
    fn test_out_of_bounds() {
        let grid = TernaryGrid::new(3, 3);
        assert_eq!(grid.get(5, 5), None);
    }

    #[test]
    fn test_cell_state_trit_roundtrip() {
        assert_eq!(CellState::from_trit(CellState::Dead.to_trit()), CellState::Dead);
        assert_eq!(CellState::from_trit(CellState::Dormant.to_trit()), CellState::Dormant);
        assert_eq!(CellState::from_trit(CellState::Alive.to_trit()), CellState::Alive);
    }
}
