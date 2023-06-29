
#[cfg_attr(not(all(unstable, feature = "hi")), derive(CompilationGuard))]
struct A;

fn main() {

}