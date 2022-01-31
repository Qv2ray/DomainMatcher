use criterion::{criterion_group, criterion_main, Criterion};
use domain_matcher::ac_automaton::{ACAutomaton, HybridMatcher};
use domain_matcher::geosite::SiteGroupList;
use domain_matcher::mph::MphMatcher;
use domain_matcher::{geosite, DomainMatcher, MatchType};
use std::fs::File;

pub fn read_file(matcher: &mut impl DomainMatcher) -> SiteGroupList {
    let file = "data/geosite.dat";
    let mut f = match File::open(&file) {
        Ok(f) => f,
        Err(e) => {
            panic!("open dat file {} failed: {}", file, e);
        }
    };
    let geosite_list: geosite::SiteGroupList = match protobuf::Message::parse_from_reader(&mut f) {
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

fn benchmark_ac_automaton(c: &mut Criterion) {
    let mut ac_automaton = ACAutomaton::new(15000);
    let geosite_list = read_file(&mut ac_automaton);
    c.bench_function("benchmark_ac_automaton", |b| {
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
    });
}

fn benchmark_hybrid_matcher(c: &mut Criterion) {
    let mut hybrid_matcher = HybridMatcher::new(15000);
    let geosite_list = read_file(&mut hybrid_matcher);
    c.bench_function("benchmark_hybrid_matcher", |b| {
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
    });
}

fn benchmark_mph_matcher(c: &mut Criterion) {
    let mut mph_matcher = MphMatcher::new(1);
    let geosite_list = read_file(&mut mph_matcher);
    c.bench_function("benchmark_mph_matcher", |b| {
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
    });
}
criterion_group!(
    benches,
    benchmark_mph_matcher,
    benchmark_hybrid_matcher,
    benchmark_ac_automaton
);
criterion_main!(benches);
