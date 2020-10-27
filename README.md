# Minimal trait E0277 issue example

```rust
$ cargo check
   Compiling typenum v1.12.0
   Compiling version_check v0.9.2
   Compiling generic-array v0.14.4
    Checking min-trait-example v0.1.0 (/home/wouter/tg/min-trait-example)
error[E0277]: the trait bound `N: generic_array::typenum::Unsigned` is not satisfied
 --> src/lib.rs:5:11
  |
5 | ) -> [u8; <N as Unsigned>::USIZE] {
  |           ^^^^^^^^^^^^^^^^^^^^^^ the trait `generic_array::typenum::Unsigned` is not implemented for `N`
  |
  = note: required by `generic_array::typenum::Unsigned::USIZE`
help: consider further restricting this bound
  |
3 | fn convert_generic_array<'a, N: ArrayLength<u8> + Unsigned + generic_array::typenum::Unsigned>(
  |                                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `min-trait-example`.

To learn more, run the command again with --verbose.
```
