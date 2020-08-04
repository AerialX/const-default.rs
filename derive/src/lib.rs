#![doc(html_root_url = "http://docs.rs/const-default-derive/0.1.0")]
extern crate proc_macro;

use core::iter;
use proc_macro2::{TokenStream, Span};
use syn::{Fields, Ident, Error};
use syn::punctuated::Pair;
use quote::quote;

fn derive_default(input: syn::DeriveInput) -> TokenStream {
    let ident = input.ident;
    let mut generics = input.generics;

    let (default, struct_) = match input.data {
        syn::Data::Struct(s) => (match &s.fields {
            Fields::Unit => quote! { Self },
            Fields::Unnamed(fields) => {
                let fields = fields.unnamed.pairs()
                    .map(|_| quote!(ConstDefault::DEFAULT));
                quote! {
                    Self(#(#fields)*)
                }
            },
            Fields::Named(fields) => {
                let fields = fields.named.pairs()
                    .map(|field| match field {
                        Pair::End(f) => {
                            let ident = &f.ident;
                            Pair::End(quote!(#ident: ConstDefault::DEFAULT))
                        },
                        Pair::Punctuated(f, p) => {
                            let ident = &f.ident;
                            Pair::Punctuated(quote!(#ident: ConstDefault::DEFAULT), p)
                        },
                    });
                quote! {
                    Self { #(#fields)* }
                }
            },
        }, s),
        _ => return Error::new(Span::call_site(), "derive can only impl ConstDefault for a struct").to_compile_error().into(),
    };

    {
        let where_clause = generics.make_where_clause();
        // TODO: filter fields to those that reference generic params only?
        for field in struct_.fields.into_iter() {
            where_clause.predicates.push(syn::WherePredicate::Type(syn::PredicateType {
                lifetimes: None,
                colon_token: Default::default(),
                bounds: iter::once(syn::TypeParamBound::Trait(syn::TraitBound {
                    paren_token: None,
                    modifier: syn::TraitBoundModifier::None,
                    lifetimes: None,
                    path: syn::Path {
                        leading_colon: Some(Default::default()),
                        segments: vec![
                            Ident::new("const_default", Span::call_site()),
                            Ident::new("ConstDefault", Span::call_site()),
                        ].into_iter().map(syn::PathSegment::from).collect(),
                    },
                })).collect(),
                bounded_ty: field.ty,
            }));
        }
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    quote! {
        impl #impl_generics ::const_default::ConstDefault for #ident #ty_generics #where_clause {
            const DEFAULT: Self = #default;
        }
    }
}

#[proc_macro_derive(ConstDefault/*, attributes(const_default)*/)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_default(syn::parse(input).unwrap()).into()
}
