use crate::ac_automaton::ACAutomaton;
use crate::ac_automaton::MatchType;

mod ac_automaton;

fn main() {
    // initiallize a 1 node ac_automaton and force it expand capacity at runtime.
    {
        let mut ac_automaton = ACAutomaton::new(1);
        ac_automaton.reverse_insert("163.com", MatchType::Domain(true));
        ac_automaton.reverse_insert("m.126.com", MatchType::Full(true));
        ac_automaton.reverse_insert("3.com", MatchType::Full(true));
        ac_automaton.reverse_insert("google.com", MatchType::SubStr(true));
        ac_automaton.reverse_insert("vgoogle.com", MatchType::SubStr(true));
        ac_automaton.build();
        assert_eq!(ac_automaton.reverse_query("126.com"), false);
        assert_eq!(ac_automaton.reverse_query("mm163.com"), false);
        assert_eq!(ac_automaton.reverse_query("m.163.com"), true); // sub domain
        assert_eq!(ac_automaton.reverse_query("163.com"), true); // sub domain
        assert_eq!(ac_automaton.reverse_query("63.com"), false);
        assert_eq!(ac_automaton.reverse_query("m.126.com"), true); // full match
        assert_eq!(ac_automaton.reverse_query("oogle.com"), false);
        assert_eq!(ac_automaton.reverse_query("vvgoogle.com"), true); // substr
    }
    {
        let mut ac_automaton_2 = ACAutomaton::new(1);
        ac_automaton_2.reverse_insert("video.google.com", MatchType::Domain(true));
        ac_automaton_2.reverse_insert("gle.com", MatchType::Domain(true));
        assert_eq!(ac_automaton_2.reverse_query("google.com"), false); // substr
    }
    println!("Hello, DomainMatcher!");
}
