use crate::ac_automaton::ACAutomaton;
use crate::mem_hash::MemHash;
use crate::{DomainMatcher, MatchType};
use deepsize::DeepSizeOf;
use std::num::Wrapping;

type RollingHashType = u32;
type Level1HashType = u32;
const PRIMEFK: Wrapping<RollingHashType> = Wrapping(16777619);
const OFFSETS: Wrapping<RollingHashType> = Wrapping(0);

#[derive(DeepSizeOf)]
pub struct MphMatcher {
    ac: ACAutomaton,
    rules: Vec<String>,
    level0: Vec<u32>,
    level0_mask: u32,
    level1: Vec<Level1HashType>,
    level1_mask: Level1HashType,
}

impl DomainMatcher for MphMatcher {
    fn reverse_insert(&mut self, input_string: &str, match_type: MatchType) {
        match match_type {
            MatchType::SubStr(_) => self.ac.reverse_insert(input_string, match_type),
            MatchType::Domain(_) => {
                self.insert_rules(input_string.to_string());
                self.insert_rules(format!(".{}", input_string));
            }
            MatchType::Full(_) => {
                self.insert_rules(input_string.to_string());
            }
        }
    }

    fn reverse_query(&self, query_string: &str) -> bool {
        let mut h = OFFSETS;
        let mut idx = Wrapping(query_string.len() - 1);
        for c in query_string.bytes().rev() {
            h = h * PRIMEFK + Wrapping(c as RollingHashType);
            if c == b'.' {
                if self.lookup(h.0, &query_string[idx.0..]) {
                    return true;
                }
            }
            idx -= Wrapping(1);
        }
        if self.lookup(h.0, query_string) {
            return true;
        } else {
            if !self.ac.empty() {
                self.ac.reverse_query(query_string)
            } else {
                false
            }
        }
    }

    fn build(&mut self) {
        if !self.ac.empty() {
            self.ac.build()
        }
        let size = self.rules.len();
        let level0_size = (size / 4).next_power_of_two();
        let level1_size = size.next_power_of_two();
        self.level0 = vec![0; level0_size];
        self.level1 = vec![0; level1_size];
        self.level0_mask = (level0_size - 1) as u32;
        self.level1_mask = (level1_size - 1) as Level1HashType;
        let mut sparse_bucket: Vec<Vec<u32>> = vec![Vec::new(); level0_size];
        for (idx, rule) in self.rules.iter().enumerate() {
            let mut h = OFFSETS;
            for c in rule.bytes().rev() {
                h = h * PRIMEFK + Wrapping(c as RollingHashType);
            }
            h &= Wrapping(self.level0_mask);
            sparse_bucket[h.0 as usize].push(idx as u32);
        }
        let mut buckets: Vec<(usize, &Vec<u32>)> = Vec::new();
        for (level0_idx, val) in sparse_bucket.iter().enumerate() {
            if !val.is_empty() {
                buckets.push((level0_idx, val));
            }
        }
        buckets.sort_by(|x, y| y.1.len().cmp(&x.1.len()));
        let mut occ = vec![false; level1_size];
        let mut tmp_occ: Vec<usize> = Vec::new();
        for bucket in buckets {
            let mut seed: Level1HashType = 0;
            loop {
                tmp_occ.clear();
                let mut find_seed = true;
                for rule_idx in bucket.1 {
                    let level1_idx =
                        seed.mem_hash(&self.rules[*rule_idx as usize]) & self.level1_mask;
                    if occ[level1_idx as usize] {
                        tmp_occ
                            .iter()
                            .for_each(|level1_idx| occ[*level1_idx] = false);
                        seed += 1;
                        find_seed = false;
                        break;
                    }
                    occ[level1_idx as usize] = true;
                    tmp_occ.push(level1_idx as usize);
                    self.level1[level1_idx as usize] = *rule_idx as Level1HashType;
                }
                if find_seed {
                    self.level0[bucket.0 as usize] = seed as u32;
                    break;
                }
            }
        }
    }
    fn clear(&mut self) {
        self.ac.clear();
        self.rules.clear();
        self.level0.clear();
        self.level1.clear();
        self.level0_mask = 0;
        self.level1_mask = 0;
    }
}

impl MphMatcher {
    pub fn new(size: usize) -> MphMatcher {
        MphMatcher {
            ac: ACAutomaton::new(size),
            rules: Vec::new(),
            level0: Vec::new(),
            level0_mask: 0,
            level1: Vec::new(),
            level1_mask: 0,
        }
    }

    fn insert_rules(&mut self, pattern: String) {
        match self.rules.binary_search(&pattern) {
            Err(pos) => {
                self.rules.insert(pos, pattern);
            }
            _ => {}
        }
    }

    #[inline(always)]
    fn lookup(&self, h: RollingHashType, query_string: &str) -> bool {
        let level0_idx = h & self.level0_mask;
        let seed = self.level0[level0_idx as usize] as Level1HashType;
        let level1_idx = seed.mem_hash(query_string) & self.level1_mask;
        return self.rules[self.level1[level1_idx as usize] as usize] == query_string;
    }
}
