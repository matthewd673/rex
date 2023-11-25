# rex

tiny regular expression engine

```
cargo run -- test_cases.txt
```

## Supported operators

- Concatenation: `abc`
- Union: `a|b`
- Kleene closure: `a*`
- Grouping: `(a|b)*`
- Escape characters: `a\*`
- Charsets: `[abc]`, `[^xyz]` (no ranges yet)
