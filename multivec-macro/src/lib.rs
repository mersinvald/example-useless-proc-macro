#![feature(proc_macro)]
extern crate proc_macro;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(VGenerate)]
pub fn generate(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    impl_generate(&ast)
}

fn impl_generate(ast: &syn::MacroInput) -> TokenStream {
    let name = &ast.ident;

    let mut vec_type = format!("");
    gen_step(200, &mut vec_type);

    let impl_def = format!("impl {} {{
        fn generate() -> {} {{
            Vec::new()
        }}    
    }}", name, vec_type);

    impl_def.parse().unwrap()
}

fn gen_step(n: i32, code: &mut String) {
    if n > 0 {
        code.push_str("Vec<");
        gen_step(n-1, code);
        code.push_str(">");
    } else {
        code.push_str("i32");
    }
}
