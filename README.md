# rex

tiny regular expression engine

```
cargo run -- test_cases.txt
```

## Supported operations

- Concatenation: `abc`
- Union: `a|b`
- Kleene closure: `a*`
  - One or more: `a+`
  - Zero or one: `a?`
- Grouping: `(a|b)*`
  - All groups are matching groups
- Escaping: `a\*`
  - Common escape sequences: `\t`, `\n`, `\r`
  - Unicode escape sequences: `\u0061`
    - Unicode characters spanning multiple bytes will fail to interpret
- Charsets: `[abc]`
  - Negation: `[^xyz]`
  - Ranges: `[a-zA-Z]`
- Wildcard: `.*`
