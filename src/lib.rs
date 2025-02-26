pub use proc_macro2;
pub use quote;
pub use syn;

pub mod utils {
    use super::*;
    use proc_macro_crate::*;
    pub fn get_call_site_crate_name(original_name: &str) -> proc_macro2::TokenStream {
        proc_macro2::TokenStream::from(
            match crate_name(original_name)
                .expect(format!("{} must be present in Cargo.toml", original_name).as_str())
            {
                FoundCrate::Itself => quote::quote!(#original_name),
                FoundCrate::Name(name) => {
                    let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
                    quote::quote! {#ident}
                }
            },
        )
    }
}
