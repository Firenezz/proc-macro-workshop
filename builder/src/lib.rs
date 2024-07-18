use syn::parse_macro_input;

mod builder;

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);

    let builder = builder::BuilderEmitter::from(input);

    proc_macro::TokenStream::from(builder.builder_emit_tokens())
}
