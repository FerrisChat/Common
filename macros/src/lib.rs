//! This implements the `CastSnowflakes` derive macro. Code quality here is horrible, so think twice
//! before snatching some code from here.

#![feature(anonymous_lifetime_in_impl_trait)]
#![allow(clippy::missing_panics_doc)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data};

#[proc_macro_derive(CastSnowflakes)]
pub fn cast_snowflakes_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_cast_snowflakes(&ast)
}

fn resolve_struct_targets<'a>(
    i: impl Iterator<Item = &'a syn::Field>,
) -> Vec<(&'a syn::Ident, bool)> {
    i.filter_map(|f| match f.ty {
        syn::Type::Path(ref t) => t.path.get_ident().map_or_else(
            || {
                t.path.segments.first().and_then(|seg| match seg.arguments {
                    syn::PathArguments::AngleBracketed(ref args) => {
                        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                            match ty {
                                syn::Type::Path(ref t) => t.path.get_ident().and_then(|ident| {
                                    (*ident == "Id").then_some((f.ident.as_ref().unwrap(), true))
                                }),
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
            },
            |ident| (*ident == "Id").then_some((f.ident.as_ref().unwrap(), false)),
        ),
        _ => None,
    })
    .collect()
}

fn resolve_tuple_targets(i: impl Iterator<Item = &syn::Field>) -> Vec<(usize, bool)> {
    i.enumerate()
        .filter_map(|(i, f)| match f.ty {
            syn::Type::Path(ref t) => t.path.get_ident().map_or_else(
                || {
                    t.path.segments.first().and_then(|seg| match seg.arguments {
                        syn::PathArguments::AngleBracketed(ref args) => {
                            if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
                                match ty {
                                    syn::Type::Path(ref t) => t
                                        .path
                                        .get_ident()
                                        .and_then(|ident| (*ident == "Id").then_some((i, true))),
                                    _ => None,
                                }
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                },
                |ident| (*ident == "Id").then_some((i, false)),
            ),
            _ => None,
        })
        .collect()
}

#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
fn impl_cast_snowflakes(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if ast.generics.type_params().any(|p| p.ident == "Id") {
        let gen = match &ast.data {
            Data::Struct(s) => {
                if s.fields.iter().next().unwrap().ident.is_some() {
                    let targets = resolve_struct_targets(s.fields.iter());

                    let (u128_fields, string_fields) = s
                     .fields
                     .iter()
                     .map(|f| {
                         let field = f.ident.as_ref().unwrap();
                         if targets.contains(&(field, false)) {
                             (
                                 quote! { #field: self.#field.parse().unwrap() },
                                 quote! { #field: self.#field.to_string() },
                             )
                         } else if targets.contains(&(field, true)) {
                             (
                                 quote! { #field: CastSnowflakes::into_u128_ids(self.#field) },
                                 quote! { #field: CastSnowflakes::into_string_ids(self.#field) },
                             )
                         } else {
                             (
                                 quote! { #field: self.#field },
                                 quote! { #field: self.#field },
                             )
                         }
                     })
                     .unzip::<_, _, Vec<_>, Vec<_>>();

                    quote! {
                        impl CastSnowflakes for #name<u128> {
                            type U128Result = #name<u128>;
                            type StringResult = #name<String>;

                            fn into_u128_ids(self) -> #name<u128> where Self: Sized {
                                self
                            }

                            fn into_string_ids(self) -> #name<String> where Self: Sized {
                                #name {
                                    #( #string_fields ),*
                                }
                            }
                        }

                        impl CastSnowflakes for #name<String> {
                            type U128Result = #name<u128>;
                            type StringResult = #name<String>;

                            fn into_u128_ids(self) -> #name<u128> where Self: Sized {
                                #name {
                                    #( #u128_fields ),*
                                }
                            }

                            fn into_string_ids(self) -> #name<String> where Self: Sized {
                                self
                            }
                        }
                    }
                } else {
                    let targets = resolve_tuple_targets(s.fields.iter());

                    let (entities_string, entities_u128) = (0..s.fields.len())
                        .map(|i| {
                            let idx = syn::Index::from(i);

                            if targets.contains(&(i, false)) {
                                (
                                    quote! { self.#idx.to_string() },
                                    quote! { self.#idx.parse().unwrap() },
                                )
                            } else if targets.contains(&(i, true)) {
                                (
                                    quote! { CastSnowflakes::into_string_ids(self.#idx) },
                                    quote! { CastSnowflakes::into_u128_ids(self.#idx) },
                                )
                            } else {
                                (quote! { self.#idx }, quote! { self.#idx })
                            }
                        })
                        .unzip::<_, _, Vec<_>, Vec<_>>();

                    quote! {
                        impl CastSnowflakes for #name<u128> {
                            type U128Result = #name<u128>;
                            type StringResult = #name<String>;

                            fn into_u128_ids(self) -> #name<u128> where Self: Sized {
                                self
                            }

                            fn into_string_ids(self) -> #name<String> where Self: Sized {
                                #name(#(#entities_string),*)
                            }
                        }

                        impl CastSnowflakes for #name<String> {
                            type U128Result = #name<u128>;
                            type StringResult = #name<String>;

                            fn into_u128_ids(self) -> #name<u128> where Self: Sized {
                                #name(#(#entities_u128),*)
                            }

                            fn into_string_ids(self) -> #name<String> where Self: Sized {
                                self
                            }
                        }
                    }
                }
            }
            Data::Enum(e) => {
                let (variants_string, variants_u128) = e
                    .variants
                    .iter()
                    .map(|v| {
                        let v_name = &v.ident;
                        let tgt = if let Some(tgt) =  v.fields.iter().next() {
                            tgt
                        } else {
                            return (
                                quote! { #name::#v_name => #name::#v_name, },
                                quote! { #name::#v_name => #name::#v_name, },
                            );
                        };

                        if tgt.ident.is_some() {
                            let targets = resolve_struct_targets(v.fields.iter());

                            let (u128_fields, string_fields) = v
                                .fields
                                .iter()
                                .map(|f| {
                                    let field = f.ident.as_ref().unwrap();

                                    if targets.contains(&(field, false)) {
                                        (
                                            quote! { #field: #field.parse().unwrap() },
                                            quote! { #field: #field.to_string() },
                                        )
                                    } else if targets.contains(&(field, true)) {
                                        (
                                            quote! { #field: CastSnowflakes::into_u128_ids(#field) },
                                            quote! { #field: CastSnowflakes::into_string_ids(#field) },
                                        )
                                    } else {
                                        (
                                            quote! { #field },
                                            quote! { #field },
                                        )
                                    }
                                })
                                .unzip::<_, _, Vec<_>, Vec<_>>();

                            let field_names = v.fields.iter()
                                .map(|f| f.ident.as_ref().unwrap())
                                .collect::<Vec<_>>();

                            (
                                quote! {
                                    #name::#v_name { #(#field_names),* } => #name::#v_name {
                                        #( #string_fields ),*
                                    },
                                },
                                quote! {
                                    #name::#v_name { #(#field_names),* } => #name::#v_name {
                                        #( #u128_fields ),*
                                    },
                                },
                            )
                        } else {
                            let targets = resolve_tuple_targets(v.fields.iter());

                            let (entities_string, (entities_u128, idents)) = (0..v.fields.len())
                                .map(|i| {
                                    let idx = syn::Ident::new(&format!("f{}", i), proc_macro::Span::call_site().into());

                                    if targets.contains(&(i, false)) {
                                        (
                                            quote! { #idx.to_string() },
                                            (
                                                quote! { #idx.parse().unwrap() },
                                                quote! { #idx },
                                            ),
                                        )
                                    } else if targets.contains(&(i, true)) {
                                        (
                                            quote! { CastSnowflakes::into_string_ids(#idx) },
                                            (
                                                quote! { CastSnowflakes::into_u128_ids(#idx) },
                                                quote! { #idx },
                                            ),
                                        )
                                    } else {
                                        (
                                            quote! { #idx },
                                            (
                                                quote! { #idx },
                                                quote! { #idx },
                                            ),
                                        )
                                    }
                                })
                                .unzip::<_, _, Vec<_>, (Vec<_>, Vec<_>)>();

                            (
                                quote! {
                                    #name::#v_name(#(#idents),*) => #name::#v_name(#(#entities_string),*),
                                },
                                quote! {
                                    #name::#v_name(#(#idents),*) => #name::#v_name(#(#entities_u128),*),
                                },
                            )
                        }
                    })
                    .unzip::<_, _, Vec<_>, Vec<_>>();

                quote! {
                    impl CastSnowflakes for #name<u128> {
                        type U128Result = #name<u128>;
                            type StringResult = #name<String>;

                        fn into_u128_ids(self) -> #name<u128> where Self: Sized {
                            self
                        }

                        fn into_string_ids(self) -> #name<String> where Self: Sized {
                            match self { #(#variants_string)* }
                        }
                    }

                    impl CastSnowflakes for #name<String> where Self: Sized {
                        type U128Result = #name<u128>;
                        type StringResult = #name<String>;

                        fn into_u128_ids(self) -> #name<u128> {
                            match self { #(#variants_u128)* }
                        }

                        fn into_string_ids(self) -> #name<String> where Self: Sized {
                            self
                        }
                    }
                }
            }
            Data::Union(_) => panic!("Unions are not supported"),
        };
        gen.into()
    } else {
        panic!("CastSnowflakes requires a type parameter Id");
    }
}
