use crate::ac_automaton::ACAutomaton;
use crate::ac_automaton::HybridMatcher;
use crate::mph::MphMatcher;
use crate::{geosite, DomainMatcher, MatchType};
use deepsize::DeepSizeOf;
use std::fs::File;

#[cfg(test)]
fn test_domain_matcher(matcher: &mut impl DomainMatcher) {
    matcher.reverse_insert("163.com", MatchType::Domain(true));
    matcher.reverse_insert("m.126.com", MatchType::Full(true));
    matcher.reverse_insert("3.com", MatchType::Full(true));
    matcher.reverse_insert("google.com", MatchType::SubStr(true));
    matcher.reverse_insert("vgoogle.com", MatchType::SubStr(true));
    matcher.build();
    assert_eq!(matcher.reverse_query("126.com"), false);
    assert_eq!(matcher.reverse_query("mm163.com"), false);
    assert_eq!(matcher.reverse_query("m.163.com"), true); // sub domain
    assert_eq!(matcher.reverse_query("163.com"), true); // sub domain
    assert_eq!(matcher.reverse_query("63.com"), false);
    assert_eq!(matcher.reverse_query("m.126.com"), true); // full match
    assert_eq!(matcher.reverse_query("oogle.com"), false);
    assert_eq!(matcher.reverse_query("vvgoogle.com"), true); // substr
    matcher.clear();
    matcher.reverse_insert("video.google.com", MatchType::Domain(true));
    matcher.reverse_insert("gle.com", MatchType::Domain(true));
    matcher.build();
    assert_eq!(matcher.reverse_query("google.com"), false);
    assert_eq!(matcher.reverse_query("video.google.com.hk"), false); // not sub domain
}

#[test]
fn test_ac_automaton() {
    let mut ac_automaton = ACAutomaton::new(1);
    test_domain_matcher(&mut ac_automaton);
}

#[test]
fn test_hybrid_matcher() {
    let mut hybrid_matcher = HybridMatcher::new(1);
    test_domain_matcher(&mut hybrid_matcher);
}

#[test]
fn test_mph_matcher() {
    let mut mph_matcher = MphMatcher::new(1);
    test_domain_matcher(&mut mph_matcher);
}

#[cfg(test)]
fn test_with_geosite(matcher: &mut impl DomainMatcher) {
    let file = "data/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("open dat file {} failed: {}", file, e);
            return;
        }
    };
    let site_group_list: geosite::SiteGroupList = match protobuf::Message::parse_from_reader(&mut f)
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("dat file {} has invalid format: {}", file, e);
            return;
        }
    };
    for i in site_group_list.site_group.iter() {
        if i.tag.to_uppercase() == "CN" {
            for domain in i.domain.iter() {
                match domain.field_type {
                    geosite::Domain_Type::Plain => {
                        matcher.reverse_insert(domain.get_value(), MatchType::SubStr(true))
                    }
                    geosite::Domain_Type::Domain => {
                        matcher.reverse_insert(domain.get_value(), MatchType::Domain(true))
                    }
                    geosite::Domain_Type::Full => {
                        matcher.reverse_insert(domain.get_value(), MatchType::Full(true))
                    }
                    _ => {}
                }
            }
        }
    }
    matcher.build();
    for i in site_group_list.site_group.iter() {
        for domain in i.domain.iter() {
            match domain.field_type {
                // we leave the regex match type to regex library.
                geosite::Domain_Type::Regex => {}
                _ => {
                    if i.tag.to_uppercase() == "CN" {
                        assert_eq!(matcher.reverse_query(domain.get_value()), true)
                    } else if i.tag.to_uppercase() == "CATEGORY-SCHOLAR-!CN"
                        && !domain.get_value().contains("cn")
                    {
                        assert_eq!(matcher.reverse_query(domain.get_value()), false)
                    }
                }
            }
        }
    }
    assert_eq!(matcher.reverse_query("163.com"), true);
    assert_eq!(matcher.reverse_query("164.com"), false);
}

#[test]
fn test_ac_automaton_with_geosite() {
    let mut ac_automaton = ACAutomaton::new(15000);
    test_with_geosite(&mut ac_automaton);
    println!(
        "Mem size of ac_automaton: {} mb, trie node count: {}",
        ac_automaton.deep_size_of() as f32 / (1024.0 * 1024.0),
        ac_automaton.trie_node_count()
    );
    println!("Hello, DomainMatcher!");
}

#[test]
fn test_hybrid_matcher_with_geosite() {
    let mut hybrid_matcher = HybridMatcher::new(1);
    test_with_geosite(&mut hybrid_matcher);
    println!(
        "Mem size of hybrid matcher: {} mb",
        hybrid_matcher.deep_size_of() as f32 / (1024.0 * 1024.0),
    );
    println!("Hello, Hybrid DomainMatcher!");
}

#[test]
fn test_mph_matcher_with_geosite() {
    let mut mph_matcher = MphMatcher::new(1);
    test_with_geosite(&mut mph_matcher);
    println!(
        "Mem size of Mph matcher: {} mb",
        mph_matcher.deep_size_of() as f32 / (1024.0 * 1024.0),
    );
    println!("Hello, Mph DomainMatcher!");
}
