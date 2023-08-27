// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Derivation of the `WitType` trait.

use crate::util::hlist_type_for;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{Fields, Ident, Variant};

#[path = "unit_tests/wit_type.rs"]
mod tests;

/// Returns the body of the `WitType` implementation for the Rust `struct` with the specified
/// `fields`.
pub fn derive_for_struct(fields: &Fields) -> TokenStream {
    let fields_hlist = hlist_type_for(fields);

    quote! {
        const SIZE: u32 = <#fields_hlist as linera_witty::WitType>::SIZE;

        type Layout = <#fields_hlist as linera_witty::WitType>::Layout;
    }
}

/// Returns the body of the `WitType` implementation for the Rust `enum` with the specified
/// `variants`.
pub fn derive_for_enum<'variants>(
    name: &Ident,
    variants: impl DoubleEndedIterator<Item = &'variants Variant> + Clone,
) -> TokenStream {
    let variant_count = variants.clone().count();
    let variant_hlists = variants.map(|variant| hlist_type_for(&variant.fields));

    let discriminant_type = if variant_count <= u8::MAX.into() {
        quote! { u8 }
    } else if variant_count <= u16::MAX.into() {
        quote! { u16 }
    } else if variant_count <= u32::MAX as usize {
        quote! { u32 }
    } else {
        abort!(name, "Too many variants in `enum`");
    };

    let discriminant_size = quote! { std::mem::size_of::<#discriminant_type>() as u32 };

    let variant_sizes = variant_hlists.clone().map(|variant_hlist| {
        quote! {
            let variant_size =
                discriminant_size + padding + <#variant_hlist as linera_witty::WitType>::SIZE;

            if variant_size > size {
                size = variant_size;
            }
        }
    });

    let variant_layouts = variant_hlists
        .map(|variant_hlist| quote! { <#variant_hlist as linera_witty::WitType>::Layout })
        .rev()
        .reduce(|current, variant_layout| {
            quote! {
                <#variant_layout as linera_witty::Merge<#current>>::Output
            }
        });

    quote! {
        const SIZE: u32 = {
            let discriminant_size = #discriminant_size;
            let mut size = discriminant_size;
            let mut variants_alignment = <#variant_layouts as linera_witty::Layout>::ALIGNMENT;
            let padding = (-(size as i32) & (variants_alignment as i32 - 1)) as u32;

            #(#variant_sizes)*

            size
        };

        type Layout = linera_witty::HCons<#discriminant_type, #variant_layouts>;
    }
}
