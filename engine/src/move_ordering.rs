use crate::moves::Move;

/// Simple killer-move table: stores up to 2 killers per ply.
pub struct KillerMoves {
    table: Vec<[Option<Move>; 2]>,
}

impl KillerMoves {
    pub fn new(max_ply: usize) -> Self {
        KillerMoves {
            table: vec![[None, None]; max_ply],
        }
    }

    pub fn clear(&mut self) {
        for slot in &mut self.table {
            slot[0] = None;
            slot[1] = None;
        }
    }

    pub fn add(&mut self, ply: usize, m: Move) {
        if ply >= self.table.len() {
            return;
        }
        let entry = &mut self.table[ply];
        if entry[0] != Some(m) {
            entry[1] = entry[0];
            entry[0] = Some(m);
        }
    }

    pub fn is_killer(&self, ply: usize, m: Move) -> bool {
        if ply >= self.table.len() {
            return false;
        }
        let entry = &self.table[ply];
        entry[0] == Some(m) || entry[1] == Some(m)
    }
}

/// History heuristic stub matching search.rs usage.
pub struct HistoryHeuristic;

impl HistoryHeuristic {
    pub fn new() -> Self {
        HistoryHeuristic
    }

    pub fn clear(&mut self) {}

    // search.rs calls: history.add(idx, move_to(m) as usize, depth)
    pub fn add(&mut self, idx: usize, to: usize, depth: i32) {
        let _ = (idx, to, depth);
    }

    // search.rs calls: history.score(idx, move_to(m) as usize)
    pub fn score(&self, idx: usize, to: usize) -> i32 {
        let _ = (idx, to);
        0
    }
}

/// Stub MVV-LVA scoring.
pub fn mvv_lva_score(_m: Move) -> i32 {
    0
}
