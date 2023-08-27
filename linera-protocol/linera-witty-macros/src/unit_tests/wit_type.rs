// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Unit tests for the `WitLoad` derive macro.

#![cfg(test)]

use super::{derive_for_enum, derive_for_struct};
use quote::quote;
use syn::{parse_quote, Fields, ItemEnum, ItemStruct};

/// Check the generated code for the body of the implementation of `WitLoad` for a unit struct.
#[test]
fn zero_sized_type() {
    let input = Fields::Unit;
    let output = derive_for_struct(&input);

    let expected = quote! {
        const SIZE: u32 = <linera_witty::HList![] as linera_witty::WitType>::SIZE;

        type Layout = <linera_witty::HList![] as linera_witty::WitType>::Layout;
    };

    assert_eq!(output.to_string(), expected.to_string());
}

/// Check the generated code for the body of the implementation of `WitLoad` for a named struct.
#[test]
fn named_struct() {
    let input: ItemStruct = parse_quote! {
        struct Type {
            first: u8,
            second: CustomType,
        }
    };
    let output = derive_for_struct(&input.fields);

    let expected = quote! {
        const SIZE: u32 = <linera_witty::HList![u8, CustomType] as linera_witty::WitType>::SIZE;

        type Layout = <linera_witty::HList![u8, CustomType] as linera_witty::WitType>::Layout;
    };

    assert_eq!(output.to_string(), expected.to_string());
}

/// Check the generated code for the body of the implementation of `WitLoad` for a tuple struct.
#[test]
fn tuple_struct() {
    let input: ItemStruct = parse_quote! {
        struct Type(String, Vec<CustomType>, i64);
    };
    let output = derive_for_struct(&input.fields);

    let expected = quote! {
        const SIZE: u32 =
            <linera_witty::HList![String, Vec<CustomType>, i64] as linera_witty::WitType>::SIZE;

        type Layout =
            <linera_witty::HList![String, Vec<CustomType>, i64] as linera_witty::WitType>::Layout;
    };

    assert_eq!(output.to_string(), expected.to_string());
}

/// Check the generated code for the body of the implementation of `WitType` for an enum.
#[test]
fn enum_type() {
    let input: ItemEnum = parse_quote! {
        enum Enum {
            Empty,
            Tuple(i8, CustomType),
            Struct {
                first: (),
                second: String,
            },
        }
    };
    let output = derive_for_enum(&input.ident, input.variants.iter());

    let expected = quote! {
        const SIZE: u32 = {
            let discriminant_size = std::mem::size_of::<u8>() as u32;
            let mut size = discriminant_size;
            let mut variants_alignment = <
                < <linera_witty::HList![] as linera_witty::WitType>::Layout as linera_witty::Merge<
                    < <linera_witty::HList![i8, CustomType] as linera_witty::WitType>::Layout
                    as linera_witty::Merge<
                        <linera_witty::HList![(), String] as linera_witty::WitType>::Layout
                    >>::Output
                >>::Output
            as linera_witty::Layout>::ALIGNMENT;
            let padding = (-(size as i32) & (variants_alignment as i32 - 1)) as u32;

            let variant_size = discriminant_size
                + padding
                + <linera_witty::HList![] as linera_witty::WitType>::SIZE;

            if variant_size > size {
                size = variant_size;
            }

            let variant_size = discriminant_size
                + padding
                + <linera_witty::HList![i8, CustomType] as linera_witty::WitType>::SIZE;

            if variant_size > size {
                size = variant_size;
            }

            let variant_size = discriminant_size
                + padding
                + <linera_witty::HList![(), String] as linera_witty::WitType>::SIZE;

            if variant_size > size {
                size = variant_size;
            }

            size
        };

        type Layout = linera_witty::HCons<u8,
            < <linera_witty::HList![] as linera_witty::WitType>::Layout as linera_witty::Merge<
                < <linera_witty::HList![i8, CustomType] as linera_witty::WitType>::Layout
                as linera_witty::Merge<
                    <linera_witty::HList![(), String] as linera_witty::WitType>::Layout
                >>::Output
            >>::Output>;
    };

    assert_eq!(output.to_string(), expected.to_string());
}
