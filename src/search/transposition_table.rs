use std::convert::TryFrom;

pub type ZobristHash = u64;

#[derive(Copy, Clone)]
pub enum MatchType {
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
}

pub struct TranspositionTable {
    data: Vec<Option<TranspositionEntry>>,
    capacity: usize,
}

impl TranspositionTable {
    pub fn with_capacity(capacity: usize) -> TranspositionTable {
        TranspositionTable {
            data: vec![None; capacity],
            capacity: capacity,
        }
    }

    pub fn check(&self, zobrist_hash: ZobristHash, depth: u16) -> Option<(MatchType, i32)>{
        let table_index = self.get_table_index(zobrist_hash);

        match self.data[table_index] {
            None => None,
            Some(entry) => {
                if entry.zobrist_hash == zobrist_hash && entry.depth >= depth {
                    Some((entry.match_type, entry.evaluation))                    
                } else {
                    None
                }
            },
        }
    }

    pub fn update(&mut self, zobrist_hash: ZobristHash, depth: u16, evaluation: i32, match_type: MatchType) {
        let table_index = self.get_table_index(zobrist_hash);
        let entry = TranspositionEntry {
            evaluation: evaluation,
            depth: depth,
            match_type: match_type,
            zobrist_hash,
        };
    
        self.data[table_index] = Some(entry);
    }

    fn get_table_index(&self, zobrist_hash: ZobristHash) -> usize {
        usize::try_from(zobrist_hash % u64::try_from(self.capacity).unwrap()).unwrap()
    }
}

#[derive(Copy, Clone)]
struct TranspositionEntry {
    pub evaluation: i32,
    pub depth: u16,
    pub match_type: MatchType,
    pub zobrist_hash: ZobristHash,
}