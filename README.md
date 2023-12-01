# rex

tiny regular expression engine

```
cargo run -- test_cases.txt
```

## Supported features

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
    - *Can* have a set of multiple character classes (e.g.: `[\s\w]`)
    - *Can* understand when `-` is meant literally (e.g.: `[\w-]`)
    - *Can* join characters with themselves (e.g.: `[a-a]`)
    - *Cannot* join character classes (e.g.: `[\w-~]`)
    - *Cannot* join characters "out of order" (e.g.: `[a-A]`)

- Common Perl ASCII character classes:
  - Any (Unicode) character: `.*`
  - `\d`: digit (`[0-9]`)
  - `\D`: not digit
  - `\w`: word (`[a-zA-Z0-9_]`)
  - `\W`: not word
  - `\s`: whitespace (`[\n-\r]`)
    - This range includes `\f`, which some versions of Perl do not
  - `\S`: not whitespace
  - `\N`: not newline (`[^\n]`)
