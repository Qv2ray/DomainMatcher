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

1. `full` and `domain` patterns are matched by Rabin-Karp algorithm;
2. `substr` patterns are matched by ac automaton;


Matching problem definition:

* a `domain` rule `baidu.com` can be seen as `exact` match `moc.udiab` and `moc.udiab.` when traversing the domain names in reverse order. And  `moc.udiab` and `moc.udiab.`  should not appear in the middle of the string.
* a `full` rule `baidu.com` can be seen as `exact` match `moc.udiab` when traversing the domain names in reverse order. And  `moc.udiab` should not appear in the middle of the string.
* a `substr`  rule `baidu.com` is a matching problem that check if `baidu.com` is substring of the given domain names.  `substr` rules can be matched by `ACAutomaton`.

Through the above definition, we can merge the `full` and `domain` rules together to match. The simplest way is to store these rules in the `HashMap`. However, when we query, we need to calculate the hash value of the same string and its substrings. This additional overhead can be reduced by rolling hash. 

In this way, our `HashMap` needs to store `KeyValue<RollingHashType,String>`. 

Of course, it should be noted that the rolling hash will collide with a small probability, and we need to open hashing. The key-value pair becomes `KeyValue<RollingHashType,Vec<String>>`.

We choose `32bit fnv-prime 16777619` to calculate our rolling hash.

