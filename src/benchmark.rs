extern crate test;
use crate::ac_automaton::{ACAutomaton, MatchType};
use crate::geosite;
use std::fs::File;

#[cfg(test)]
pub fn read_file() -> geosite::SiteGroupList {
    let file = "data/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            panic!("open dat file {} failed: {}", file, e);
        }
    };
    let geo_ip_list: geosite::SiteGroupList =
        match protobuf::parse_from_reader::<geosite::SiteGroupList>(&mut f) {
            Ok(v) => v,
            Err(e) => {
                panic!("dat file {} has invalid format: {}", file, e);
            }
        };
    geo_ip_list
}

#[bench]
fn benchmark_ac_automaton(b: &mut test::Bencher) {
    let geosite_list = read_file();
    let mut ac_automaton = ACAutomaton::new(15000);
    for i in geosite_list.site_group.iter() {
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
            break;
        }
    }
    ac_automaton.build();
    b.iter(|| {
        for i in geosite_list.site_group.iter() {
            for domain in i.domain.iter() {
                match domain.field_type {
                    geosite::Domain_Type::Regex => {
                        println!("regex domain:{}", domain.get_value());
                    }
                    _ => {
                        ac_automaton.reverse_query(domain.get_value());
                    }
                }
            }
        }
    })
}
