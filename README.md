# RLox

After finishing my implementation of Lox in _Crafting Interpreters_ here (github.com/jakestambaugh/craftinginterpreters) I want to try it in Rust! I expect to write and re-write things as the idiomatic Rust and well-documented C/Java implementations come into tension.

## Running the test suite

```
dart --snapshot=build/test.dart.snapshot --snapshot-kind=app-jit tool/bin/test.dart rlox
```