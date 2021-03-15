## DomainMatcher Goals
Implement a fast algorithm which can solve the following type of matching problem.

### Full 
Full is the type of matcher that the input string must exactly equal to the pattern.

### Substr
Substr is the type of matcher that the input string must contain the pattern as a sub-string.
### Domain
Domain is the type of matcher that the input string must be a sub-domain or itself of the pattern.

## Implementation detail

The DomainMatcher is divided into two parts:

1. `full` and `domain` patterns are matched by Rabin-Karp algorithm & minimal perfect hash table;
2. `substr` patterns are matched by ac automaton;


Matching problem definition:

* a `domain` rule `baidu.com` can be seen as `exact` match `moc.udiab` and `moc.udiab.` when traversing the domain names in reverse order. And  `moc.udiab` and `moc.udiab.`  should not appear in the middle of the string.
* a `full` rule `baidu.com` can be seen as `exact` match `moc.udiab` when traversing the domain names in reverse order. And  `moc.udiab` should not appear in the middle of the string.
* a `substr`  rule `baidu.com` is a matching problem that check if `baidu.com` is substring of the given domain names.  `substr` rules can be matched by `ACAutomaton`.

Through the above definition, we can merge the `full` and `domain` rules together to match. The simplest way is to store these rules in the `HashMap`. However, when we query, we need to calculate the hash value of the same string and its substrings. This additional overhead can be reduced by rolling hash. 

We choose `32bit FNV-prime 16777619` to calculate our rolling hash.

Inspired by ["Hash, displace, and compress" algorithm](http://cmph.sourceforge.net/papers/esa09.pdf), we can design a [minimal perfect hash table](https://en.wikipedia.org/wiki/Perfect_hash_function#Minimal_perfect_hash_function) through two rounds hashes. The first round of hash is rolling hash, which we get directly from the process of traversing the string. The second round of hash is [memhash](https://golang.org/src/runtime/hash64.go). 


In this way, when checking whether the rule is hit, we only need to calculate the hash and compare it once.


````rust
#[inline(always)]
fn lookup(&self, h: RollingHashType, query_string: &str) -> bool {
    let level0_idx = h & self.level0_mask;
    let seed = self.level0[level0_idx as usize] as Level1HashType;
    let level1_idx = seed.mem_hash(query_string) & self.level1_mask;
    return self.rules[self.level1[level1_idx as usize] as usize] == query_string;
}
````
