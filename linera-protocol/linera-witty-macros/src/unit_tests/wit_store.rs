// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Unit tests for the `WitStore` derive macro.

#![cfg(test)]

use super::{derive_for_enum, derive_for_struct};
use quote::quote;
use syn::{parse_quote, Fields, ItemEnum, ItemStruct};

/// Check the generated code for the body of the implementation of `WitStore` for a unit struct.
#[test]
fn zero_sized_type() {
    let input = Fields::Unit;
    let output = derive_for_struct(&input);

    let expected = quote! {
        fn store<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
            mut location: linera_witty::GuestPointer,
        ) -> Result<(), linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            let Self = self;
            linera_witty::hlist![].store(memory, location)
        }

        fn lower<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
        ) -> Result<<Self::Layout as linera_witty::Layout>::Flat, linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            let Self = self;
            linera_witty::hlist![].lower(memory)
        }
    };

    assert_eq!(output.to_string(), expected.to_string());
}

/// Check the generated code for the body of the implementation of `WitStore` for a named struct.
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
        fn store<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
            mut location: linera_witty::GuestPointer,
        ) -> Result<(), linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            let Self { first, second } = self;
            linera_witty::hlist![first, second].store(memory, location)
        }

        fn lower<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
        ) -> Result<<Self::Layout as linera_witty::Layout>::Flat, linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            let Self { first, second } = self;
            linera_witty::hlist![first, second].lower(memory)
        }
    };

    assert_eq!(output.to_string(), expected.to_string());
}

/// Check the generated code for the body of the implementation of `WitStore` for a tuple struct.
#[test]
fn tuple_struct() {
    let input: ItemStruct = parse_quote! {
        struct Type(String, Vec<CustomType>, i64);
    };
    let output = derive_for_struct(&input.fields);

    let expected = quote! {
        fn store<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
            mut location: linera_witty::GuestPointer,
        ) -> Result<(), linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            let Self(field0, field1, field2) = self;
            linera_witty::hlist![field0, field1, field2].store(memory, location)
        }

        fn lower<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
        ) -> Result<<Self::Layout as linera_witty::Layout>::Flat, linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            let Self(field0, field1, field2) = self;
            linera_witty::hlist![field0, field1, field2].lower(memory)
        }
    };

    assert_eq!(output.to_string(), expected.to_string());
}

/// Check the generated code for the body of the implementation of `WitStore` for a enum.
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
        fn store<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
            mut location: linera_witty::GuestPointer,
        ) -> Result<(), linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            match self {
                Enum::Empty => {
                    0_u8.store(memory, location)?;
                    location = location
                        .after::<u8>()
                        .after_padding_for::<linera_witty::HList![]>()
                        .after_padding_for::<linera_witty::HList![i8, CustomType]>()
                        .after_padding_for::<linera_witty::HList![(), String]>();

                    linera_witty::hlist![].store(memory, location)
                }
                Enum::Tuple(field0, field1) => {
                    1_u8.store(memory, location)?;
                    location = location
                        .after::<u8>()
                        .after_padding_for::<linera_witty::HList![]>()
                        .after_padding_for::<linera_witty::HList![i8, CustomType]>()
                        .after_padding_for::<linera_witty::HList![(), String]>();

                    linera_witty::hlist![field0, field1].store(memory, location)
                }
                Enum::Struct { first, second } => {
                    2_u8.store(memory, location)?;
                    location = location
                        .after::<u8>()
                        .after_padding_for::<linera_witty::HList![]>()
                        .after_padding_for::<linera_witty::HList![i8, CustomType]>()
                        .after_padding_for::<linera_witty::HList![(), String]>();

                    linera_witty::hlist![first, second].store(memory, location)
                }
            }
        }

        fn lower<Instance>(
            &self,
            memory: &mut linera_witty::Memory<'_, Instance>,
        ) -> Result<<Self::Layout as linera_witty::Layout>::Flat, linera_witty::RuntimeError>
        where
            Instance: linera_witty::InstanceWithMemory,
            <Instance::Runtime as linera_witty::Runtime>::Memory:
                linera_witty::RuntimeMemory<Instance>,
        {
            match self {
                Enum::Empty => {
                    let variant_flat_layout = linera_witty::hlist![].lower(memory)?;
                    let flat_layout: <Self::Layout as linera_witty::Layout>::Flat =
                        linera_witty::JoinFlatLayouts::into_joined(
                            0_u8.lower(memory)? + variant_flat_layout,
                        );

                    Ok(flat_layout)
                }
                Enum::Tuple(field0, field1) => {
                    let variant_flat_layout = linera_witty::hlist![field0, field1].lower(memory)?;
                    let flat_layout: <Self::Layout as linera_witty::Layout>::Flat =
                        linera_witty::JoinFlatLayouts::into_joined(
                            1_u8.lower(memory)? + variant_flat_layout,
                        );

                    Ok(flat_layout)
                }
                Enum::Struct { first, second } => {
                    let variant_flat_layout = linera_witty::hlist![first, second].lower(memory)?;
                    let flat_layout: <Self::Layout as linera_witty::Layout>::Flat =
                        linera_witty::JoinFlatLayouts::into_joined(
                            2_u8.lower(memory)? + variant_flat_layout,
                        );

                    Ok(flat_layout)
                }
            }
        }
    };

    assert_eq!(output.to_string(), expected.to_string());
}
