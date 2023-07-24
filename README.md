# rex

tiny regular expression engine

```
gcc *.c -o rex
./rex
```

## Debugging

1. Build with `-g`

2.
    On macOS:
    ```
    leaks -atExit -- ./rex
    ```