use proc_macro::{self, TokenStream};

/// Forces compilation to fail.
/// 
/// e.g. 
/// ```rust
/// #![allow(unused_imports, dead_code)]
/// 
/// use compilation_guard::compilation_guard;
/// #[cfg_attr(all(unstable, feature = "enable-macro"), derive(VeryExpensiveMacro))]
/// #[cfg_attr(all(not(unstable), feature = "enable-macro"), compilation_guard("lol"))]
/// struct A;
/// 
/// fn main() {
/// 
/// }
/// ```
/// 
/// If you don't enable the unstable flag, you get this.
/// ```
/// error: custom attribute panicked
///  --> src/main.rs:5:58
///   |
/// 5 | #[cfg_attr(all(not(unstable), feature = "enable-macro"), compilation_guard("lol"))]
///   |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^
///   |
///   = help: message: Compilation guard was triggered!
///           lol
/// ```
/// 
#[proc_macro_attribute]
pub fn compilation_guard(input: TokenStream, _: TokenStream) -> TokenStream {
    panic_logic(input);
    unreachable!()
}

fn panic_logic(input: TokenStream) {
    let mut iter = input.into_iter();
    match (iter.next(), iter.next()) {
        (Some(proc_macro::TokenTree::Literal(literal)), None) => {
            let s = literal.to_string();
            let s = format!("Compilation guard was triggered!\n{}", &s[1..s.len()-1]);
            panic!("{s}");
        }
        _ => panic!("This macro is expected to receive a `literal` as an argument.")
    }
}