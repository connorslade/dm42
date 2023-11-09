# dm42

Programs for the [Swissmicros DM42](https://www.swissmicros.com/product/dm42) and a preprocessor for creating them.

## Example

This code that makes uses of function definitions, if statements and while loops is compiled down into plain hp 42s instructions.

```cpp
export def sort {
    if { MAT? } == { 0 } {
        "X is not a matrix", AVIEW
        RTN
    }

    STO "A"
    INDEX "A"
    1, 1, STOIJ
    DROPN 2

    while { FC? 76 } == { 1 } {
        [MIN]
        DROP
        RCLIJ
        DROP
        R<>R
        DROPN 2
        I+
    }

    RCL "A"
}
```

## Todo

- [ ] Finish parser / tokenizer
  - [x] Base
  - [x] Functions
  - [x] If
  - [ ] Raw-If
  - [x] While
- [ ] Finish codegen
  - [ ] Functions
  - [ ] If-statements
  - [ ] While loops
- [ ] Future Ideas
  - [ ] For loops
