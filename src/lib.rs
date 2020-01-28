extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(Dirty)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let name = Ident::new(&format!("Dirty{}", &name), name.span());

    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
            .iter()
            .filter(|s| {
                if let Some(name) = &s.ident {
                    return name != "id"
                }
                false
            })
    } else {
        unimplemented!()
    };

    let expanded = quote! {

        struct #name {
            #(#fields,)*
        };

    };
    expanded.into()
}
