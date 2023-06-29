# Compilation Guard

Say, you have a feature that is behind several different feature guard, and you want to make sure that your user knows whne it has to compile.

This crate can help you with that.

e.g. 

```rust
use compilation_guard::{CompilationGuard, compilation_guard};
/// You want both --cfg and crate level feature to be enabled.
#[cfg_attr(all(unstable, feature = "enable-macro"), derive(VeryExpensiveMacro))]
#[cfg_attr(all(not(unstable), feature = "enable-macro"), derive(CompilationGuard("hi")))]
struct A;
```