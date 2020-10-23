mod ac_automaton;
mod geosite;
use crate::ac_automaton::ACAutomaton;
use crate::ac_automaton::MatchType;
use std::fs::File;

fn main() {
    let file = "src/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open dat file {} failed: {}", file, e);
            return;
        }
    };
    let site_group_list:geosite::SiteGroupList =
        match protobuf::parse_from_reader::<geosite::SiteGroupList>(&mut f) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("dat file {} has invalid format: {}", file, e);
                return
            }
        };
    let mut ac_automaton =ACAutomaton::new(15000);
    for i in site_group_list.site_group.iter(){
        if i.tag.to_uppercase()=="CN" {
            for domain in i.domain.iter() {
                match domain.field_type {
                    geosite::Domain_Type::Plain => {
                        ac_automaton.reverse_insert(domain.get_value(), MatchType::SubStr(true))
                    }
                    geosite::Domain_Type::Domain => {
                        ac_automaton.reverse_insert(domain.get_value(), MatchType::Domain(true))
                    }
                    geosite::Domain_Type::Full => {
                        ac_automaton.reverse_insert(domain.get_value(), MatchType::Full(true))
                    }
                    _ => {}
                }
            }
            break
        }
    }
    ac_automaton.build();
    println!("Mem size of ac_automaton: {} mb, trie node count: {}", ac_automaton.runtime_memory_size() as f32/(1024.0*1024.0), ac_automaton.trie_node_count());
    assert_eq!(ac_automaton.reverse_query("163.com"),true);
    assert_eq!(ac_automaton.reverse_query("164.com"),false);
    println!("Hello, DomainMatcher!");
}
