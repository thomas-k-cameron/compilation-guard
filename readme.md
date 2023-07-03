# Compilation Guard

Say, you have a feature that is behind several different feature guard, and you want to make sure that your user knows whne it has to compile.

This crate can help you with that.

e.g. 

```rust
#![allow(unused_imports, dead_code)]

use compilation_guard::compilation_guard;
#[cfg_attr(all(unstable, feature = "enable-macro"), derive(VeryExpensiveMacro))]
#[cfg_attr(all(not(unstable), feature = "enable-macro"), compilation_guard("lol"))]
struct A;

fn main() {

}
```

If you don't enable the unstable flag, you get this.
```
error: custom attribute panicked
 --> src/main.rs:5:58
  |
5 | #[cfg_attr(all(not(unstable), feature = "enable-macro"), compilation_guard("lol"))]
  |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = help: message: Compilation guard was triggered!
          lol
```
