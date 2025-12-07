use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Getters, attributes(skip))]
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

    'main: for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        for attr in &field.attrs{
            if attr.path().is_ident("skip") {
                continue 'main;
            }
        }

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

#[proc_macro_derive(DebugIf, attributes(skip))]
pub fn debug_if_debug(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => panic!("DebugIf only for structs with named fields"),
        },
        _ => panic!("DebugIf only for structs"),
    };

    let mut fmt_fields = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();

        fmt_fields.push(quote! {
            .field(stringify!(#field_name), &self.#field_name)
        });
    }

    let (impl_generics, ty_generics_impl, original_where_clause) = generics.split_for_impl();

    let mut new_where_clause = original_where_clause.cloned();

    for param in &generics.params {
        if let syn::GenericParam::Type(type_param) = param {
            let ident = &type_param.ident;

            let new_predicate: WherePredicate = syn::parse_quote! {
                #ident: std::fmt::Debug
            };

            if let Some(ref mut clause) = new_where_clause {
                clause.predicates.push(new_predicate);
            } else {
                new_where_clause = Some(syn::parse_quote! {
                    where #ident: std::fmt::Debug
                });
            }
        }
    }


    let expanded = quote! {
        impl #impl_generics Debug for #name #ty_generics_impl #new_where_clause {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Engine")
                #(#fmt_fields)*
                .finish()
            }
        }
    };

    TokenStream::from(expanded)
}
