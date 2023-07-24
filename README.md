# rex

tiny regular expression engine

```
gcc *.c -o rex
./rex
```

## Example
```c
#include <stdio>
#include "regex.h"

int main() {
    RegEx re = compile("(ab|a)*");
    printf("Does 'ababa' match? %d\n", match(re, "ababa"));
    free_RegEx(re);
}
```


## Debugging

1. Build with `-g`

2.
    On macOS:
    ```
    leaks -atExit -- ./rex
    ```