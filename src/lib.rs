use proc_macro::{self, TokenStream};

#[proc_macro_derive(CompilationGuard)]
pub fn derive(input: TokenStream) -> TokenStream {
    panic_logic(input);
    unreachable!();
}

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