extern crate test;
use crate::ac_automaton::{ACAutomaton, HybridMatcher};
use crate::geosite::SiteGroupList;
use crate::mph::MphMatcher;
use crate::{geosite, DomainMatcher, MatchType};
use std::fs::File;

#[cfg(test)]
pub fn read_file(matcher: &mut impl DomainMatcher) -> SiteGroupList {
    let file = "data/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            panic!("open dat file {} failed: {}", file, e);
        }
    };
    let geosite_list: geosite::SiteGroupList =
        match protobuf::parse_from_reader::<geosite::SiteGroupList>(&mut f) {
            Ok(v) => v,
            Err(e) => {
                panic!("dat file {} has invalid format: {}", file, e);
            }
        };
    for i in geosite_list.site_group.iter() {
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
            break;
        }
    }
    matcher.build();
    geosite_list
}

#[bench]
fn benchmark_ac_automaton(b: &mut test::Bencher) {
    let mut ac_automaton = ACAutomaton::new(15000);
    let geosite_list = read_file(&mut ac_automaton);
    b.iter(|| {
        for i in geosite_list.site_group.iter() {
            for domain in i.domain.iter() {
                match domain.field_type {
                    geosite::Domain_Type::Regex => {}
                    _ => {
                        ac_automaton.reverse_query(domain.get_value());
                    }
                }
            }
        }
    })
}

#[bench]
fn benchmark_hybrid_matcher(b: &mut test::Bencher) {
    let mut hybrid_matcher = HybridMatcher::new(15000);
    let geosite_list = read_file(&mut hybrid_matcher);
    b.iter(|| {
        for i in geosite_list.site_group.iter() {
            for domain in i.domain.iter() {
                match domain.field_type {
                    geosite::Domain_Type::Regex => {}
                    _ => {
                        hybrid_matcher.reverse_query(domain.get_value());
                    }
                }
            }
        }
    })
}

#[bench]
fn benchmark_mph_matcher(b: &mut test::Bencher) {
    let mut mph_matcher = MphMatcher::new(1);
    let geosite_list = read_file(&mut mph_matcher);
    b.iter(|| {
        for i in geosite_list.site_group.iter() {
            for domain in i.domain.iter() {
                match domain.field_type {
                    geosite::Domain_Type::Regex => {}
                    _ => {
                        mph_matcher.reverse_query(domain.get_value());
                    }
                }
            }
        }
    })
}
