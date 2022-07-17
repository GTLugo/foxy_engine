use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
  // Parse input code as abstract syntax tree
  let ast = parse_macro_input!(input as DeriveInput);
  impl_derive_component(&ast)
}

fn impl_derive_component(ast: &DeriveInput) -> TokenStream {
  let name = &ast.ident;

  // Construct output code
  (quote!{
    impl Component for #name {
      fn register() {

      }
    }
  }).into()
}