use compilation_guard::{CompilationGuard, compilation_guard};
#[cfg_attr(not(all(unstable, feature = "hi")), derive(CompilationGuard("hi")))]
struct A;

fn main() {

}