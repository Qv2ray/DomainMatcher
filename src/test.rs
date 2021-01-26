extern crate test;
use crate::ac_automaton::ACAutomaton;
use crate::ac_automaton::HybridMatcher;
use crate::ac_automaton::MatchType;
use crate::geosite;
use deepsize::DeepSizeOf;
use std::fs::File;

#[test]
fn test_ac_automaton_with_geosite() {
    let file = "data/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open dat file {} failed: {}", file, e);
            return;
        }
    };
    let site_group_list: geosite::SiteGroupList =
        match protobuf::parse_from_reader::<geosite::SiteGroupList>(&mut f) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("dat file {} has invalid format: {}", file, e);
                return;
            }
        };
    let mut ac_automaton = ACAutomaton::new(15000);
    for i in site_group_list.site_group.iter() {
        if i.tag.to_uppercase() == "CN" {
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
        }
    }
    ac_automaton.build();
    for i in site_group_list.site_group.iter() {
        for domain in i.domain.iter() {
            match domain.field_type {
                // we leave the regex match type to regex library.
                geosite::Domain_Type::Regex => {}
                _ => {
                    if i.tag.to_uppercase() == "CN" {
                        assert_eq!(ac_automaton.reverse_query(domain.get_value()), true)
                    } else if i.tag.to_uppercase() == "CATEGORY-SCHOLAR-!CN"
                        && !domain.get_value().contains("cn")
                    {
                        assert_eq!(ac_automaton.reverse_query(domain.get_value()), false)
                    }
                }
            }
        }
    }
    println!(
        "Mem size of ac_automaton: {} mb, trie node count: {}",
        ac_automaton.deep_size_of() as f32 / (1024.0 * 1024.0),
        ac_automaton.trie_node_count()
    );
    assert_eq!(ac_automaton.reverse_query("163.com"), true);
    assert_eq!(ac_automaton.reverse_query("164.com"), false);
    println!("Hello, DomainMatcher!");
}

#[test]
fn test_hybrid_matcher_with_geosite() {
    let file = "data/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open dat file {} failed: {}", file, e);
            return;
        }
    };
    let site_group_list: geosite::SiteGroupList =
        match protobuf::parse_from_reader::<geosite::SiteGroupList>(&mut f) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("dat file {} has invalid format: {}", file, e);
                return;
            }
        };
    let mut ac_automaton = HybridMatcher::new(1);
    for i in site_group_list.site_group.iter() {
        if i.tag.to_uppercase() == "CN" {
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
        }
    }
    ac_automaton.build();
    for i in site_group_list.site_group.iter() {
        for domain in i.domain.iter() {
            match domain.field_type {
                // we leave the regex match type to regex library.
                geosite::Domain_Type::Regex => {}
                _ => {
                    if i.tag.to_uppercase() == "CN" {
                        assert_eq!(ac_automaton.reverse_query(domain.get_value()), true)
                    } else if i.tag.to_uppercase() == "CATEGORY-SCHOLAR-!CN"
                        && !domain.get_value().contains("cn")
                    {
                        assert_eq!(ac_automaton.reverse_query(domain.get_value()), false)
                    }
                }
            }
        }
    }
    println!(
        "Mem size of hybrid matcher: {} mb",
        ac_automaton.deep_size_of() as f32 / (1024.0 * 1024.0),
    );
    assert_eq!(ac_automaton.reverse_query("163.com"), true);
    assert_eq!(ac_automaton.reverse_query("164.com"), false);
    println!("Hello, Hybrid DomainMatcher!");
}
