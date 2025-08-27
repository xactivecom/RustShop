// Import procedural macro tooling
extern crate proc_macro;

// Output of procedural macro is the token stream
use proc_macro::TokenStream;

// Macro that generates a token stream of Rust code from a syntax tree
use quote::quote;

// Parser for Rust code into a syntax tree
use syn::{ parse_macro_input, ItemFn };

#[proc_macro_attribute]
pub fn debug_print(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse Rust code
    let mut item_fn = parse_macro_input!(item as ItemFn);

    // Get metadata about item_fn
    let ident = &item_fn.sig.ident;

    item_fn.block.stmts.insert(
        0,
        syn::parse_quote!( println!("Entering fn: {}", stringify!(#ident)); ),
    );

    TokenStream::from(quote!{
        #item_fn
    })
}
