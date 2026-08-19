use crate::moves::Move;

#[derive(Copy, Clone, PartialEq)]
pub enum Bound {
    Exact,
    Alpha,
    Beta,
}

#[derive(Copy, Clone)]
pub struct TTEntry {
    pub key: u64,
    pub depth: i32,
    pub score: i32,
    pub bound: Bound,
    pub best_move: Option<Move>,
}

pub struct TranspositionTable {
    pub table: Vec<TTEntry>,
    pub mask: usize,
}

impl TranspositionTable {
    pub fn new(size_mb: usize) -> Self {
        let entries = (size_mb * 1024 * 1024) / std::mem::size_of::<TTEntry>();
        let size = entries.next_power_of_two();
        let mask = size - 1;

        Self {
            table: vec![
                TTEntry {
                    key: 0,
                    depth: -1,
                    score: 0,
                    bound: Bound::Exact,
                    best_move: None,
                };
                size
            ],
            mask,
        }
    }

    #[inline]
    fn index(&self, key: u64) -> usize {
        (key as usize) & self.mask
    }

    pub fn probe(&self, key: u64, depth: i32, alpha: i32, beta: i32) -> Option<(i32, Option<Move>)> {
        let idx = self.index(key);
        let entry = self.table[idx];

        if entry.key != key {
            return None;
        }

        if entry.depth < depth {
            return None;
        }

        match entry.bound {
            Bound::Exact => Some((entry.score, entry.best_move)),
            Bound::Alpha if entry.score <= alpha => Some((entry.score, entry.best_move)),
            Bound::Beta if entry.score >= beta => Some((entry.score, entry.best_move)),
            _ => None,
        }
    }

    pub fn store(
        &mut self,
        key: u64,
        depth: i32,
        score: i32,
        bound: Bound,
        best_move: Option<Move>,
    ) {
        let idx = self.index(key);

        // Replace only if deeper or empty
        if self.table[idx].depth <= depth {
            self.table[idx] = TTEntry {
                key,
                depth,
                score,
                bound,
                best_move,
            };
        }
    }
}
