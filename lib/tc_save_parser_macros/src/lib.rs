use std::mem::Discriminant;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemEnum, Variant, parse_macro_input};

#[proc_macro_attribute]
pub fn kind_mapper(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemEnum);
    let orig_variants = ast.variants.clone();
    let enum_name = &ast.ident;
    let from_variants = orig_variants.iter().map(|variant| {
        let ident = &variant.ident;
        if let Some((_, expr)) = &variant.discriminant {
            quote! {
                #expr => Self::#ident,
            }
        } else {
            quote! {
                // compile_error!("All variants must have a discriminant");
            }
        }
    });
    let to_variants = orig_variants.iter().map(|variant| {
        let ident = &variant.ident;
        if let Some((_, expr)) = &variant.discriminant {
            quote! {
                #enum_name::#ident => #expr,
            }
        } else {
            quote! {
                // compile_error!("All variants must have a discriminant");
            }
        }
    });

    println!("Attrs: {:?}", ast.attrs);

    let vis = &ast.vis;
    let enum_token = &ast.enum_token;
    let ident = &ast.ident;
    let attrs = &ast.attrs;
    let generics = &ast.generics;

    let expanded = quote! {
        #(#attrs)*
        #vis #enum_token #ident #generics {
            #orig_variants
            Unmapped(u16),
        }
        // #ast

        impl From<u16> for #ident {
            fn from(value: u16) -> Self {
                match value {
                    #(#from_variants)*
                    _ => Self::Unmapped(value),
                }
            }
        }
        impl From<#ident> for u16 {
            fn from(value: #ident) -> u16 {
                u16::from(&value)
            }
        }
        impl From<&#ident> for u16 {
            fn from(value: &#ident) -> u16 {
                match value {
                    #(#to_variants)*
                    #ident::Unmapped(value) => *value,
                }
            }
        }
    };
    TokenStream::from(expanded)
}

// TODO: Figure out how to make this work
// #[proc_macro_attribute(ParserHelper)]
// pub fn length_prefixed_array(args: TokenStream, input: TokenStream) -> TokenStream {
//     let ast = parse_macro_input!(input as syn::);
//     let expanded = quote! {
//         #[bw(try_calc(u16::try_from(#ident.len())))]
//         #(ident)_len: u16
//         #[br(count = #(ident)_len)]
//         #ast
//     };
//     TokenStream::from(expanded)
// }
