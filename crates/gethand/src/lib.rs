use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Getters)]
pub fn getters_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("Getters only for structs with named fields"),
        },
        _ => panic!("Getters only for structs"),
    };

    let mut immut_getters = Vec::new();
    let mut mut_getters = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;

        immut_getters.push(quote! {
            pub fn #field_name(&self) -> &#field_type {
                &self.#field_name
            }
        });

        let mut_name = Ident::new(&format!("{field_name}_mut"), field_name.span());

        mut_getters.push(quote! {
            pub fn #mut_name(&mut self) -> &mut #field_type {
                &mut self.#field_name
            }
        });
    }

    let (impl_generics, ty_generics_impl, original_where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics_impl #original_where_clause {
            #(#immut_getters)*
            #(#mut_getters)*
        }
    };

    TokenStream::from(expanded)
}
