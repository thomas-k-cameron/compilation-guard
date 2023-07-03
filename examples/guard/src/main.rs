#![allow(unused_imports, dead_code)]

use compilation_guard::compilation_guard;
#[cfg_attr(all(unstable, feature = "enable-macro"), derive(VeryExpensiveMacro))]
#[cfg_attr(all(not(unstable), feature = "enable-macro"), compilation_guard("lol"))]
struct A;

fn main() {

}