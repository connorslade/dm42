# dm42

Programs for the [Swissmicros DM42](https://www.swissmicros.com/product/dm42) and a preprocessor for creating them.

## Example

This code that makes uses of function definitions, if statements and while loops is compiled down into plain hp 42s instructions.

```cpp
export def sort {
    if { MAT? } {} else {
        "X is not a matrix", AVIEW
        STOP
    }

    STO "A"
    INDEX "A"
    1, 1, STOIJ
    DROPN 2

    while { FC? 76 } {
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

- [x] Finish parser / tokenizer
  - [x] Base
  - [x] Functions
  - [x] If
  - [x] Raw-If
  - [x] While
  - [ ] Do While
- [x] Finish codegen
  - [x] Functions
  - [x] If-statements
  - [x] While loops
  - [x] Do While
- [ ] Future Ideas
  - [ ] For loops
  - [ ] Decent error reporting
  - [ ] Inline functions
  - [ ] vscode lang grammer
  - [ ] Basic documentation
  - [ ] Macros
  - [ ] Imports
  - [ ] CLI
  - [ ] Automatically cut string
