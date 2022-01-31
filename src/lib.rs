pub mod ac_automaton;
#[cfg(feature = "pb")]
pub mod geosite;
mod mem_hash;
pub mod mph;
#[cfg(all(test, feature = "pb"))]
mod test;

#[derive(Copy, Clone)]
pub enum MatchType {
    Domain(bool),
    SubStr(bool),
    Full(bool),
}

impl From<bool> for MatchType {
    fn from(v: bool) -> Self {
        MatchType::Full(v)
    }
}
pub trait DomainMatcher {
    fn reverse_insert(&mut self, input_string: &str, match_type: MatchType);
    fn reverse_query(&self, query_string: &str) -> bool;
    fn build(&mut self);
    fn clear(&mut self);
}
