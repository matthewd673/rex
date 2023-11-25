# rex

tiny regular expression engine

```
cargo run -- test_cases.txt
```

## Supported operators

- Concatenation: `abc`
- Union: `a|b`
- Kleene closure: `a*`
  - One or more: `a+`
  - Zero or one: `a?`
- Grouping: `(a|b)*`
- Escaping: `a\*`
  - Common escape sequences: `\t`, `\n`, `\r`
- Charsets: `[abc]`, `[^xyz]` (no ranges yet)
