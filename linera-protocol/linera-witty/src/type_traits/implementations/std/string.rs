// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Implementations of the custom traits for the [`String`] type.

use crate::{
    GuestPointer, InstanceWithMemory, Layout, Memory, Runtime, RuntimeError, RuntimeMemory,
    WitLoad, WitStore, WitType,
};
use frunk::{hlist, hlist_pat, HList};

impl WitType for String {
    const SIZE: u32 = 8;

    type Layout = HList![i32, i32];
}

impl WitLoad for String {
    fn load<Instance>(
        memory: &Memory<'_, Instance>,
        location: GuestPointer,
    ) -> Result<Self, RuntimeError>
    where
        Instance: InstanceWithMemory,
        <Instance::Runtime as Runtime>::Memory: RuntimeMemory<Instance>,
    {
        let address = GuestPointer::load(memory, location)?;
        let length = u32::load(memory, location.after::<GuestPointer>())?;

        let bytes = memory.read(address, length)?.to_vec();

        Ok(String::from_utf8(bytes)?)
    }

    fn lift_from<Instance>(
        hlist_pat![address, length]: <Self::Layout as Layout>::Flat,
        memory: &Memory<'_, Instance>,
    ) -> Result<Self, RuntimeError>
    where
        Instance: InstanceWithMemory,
        <Instance::Runtime as Runtime>::Memory: RuntimeMemory<Instance>,
    {
        let address = GuestPointer(address.try_into()?);
        let length = length as u32;

        let bytes = memory.read(address, length)?.to_vec();

        Ok(String::from_utf8(bytes)?)
    }
}

impl WitStore for String {
    fn store<Instance>(
        &self,
        memory: &mut Memory<'_, Instance>,
        location: GuestPointer,
    ) -> Result<(), RuntimeError>
    where
        Instance: InstanceWithMemory,
        <Instance::Runtime as Runtime>::Memory: RuntimeMemory<Instance>,
    {
        let length = u32::try_from(self.len())?;
        let destination = memory.allocate(length)?;

        destination.store(memory, location)?;
        length.store(memory, location.after::<GuestPointer>())?;

        memory.write(destination, self.as_bytes())
    }

    fn lower<Instance>(
        &self,
        memory: &mut Memory<'_, Instance>,
    ) -> Result<Self::Layout, RuntimeError>
    where
        Instance: InstanceWithMemory,
        <Instance::Runtime as Runtime>::Memory: RuntimeMemory<Instance>,
    {
        let length = u32::try_from(self.len())?;
        let destination = memory.allocate(length)?;

        memory.write(destination, self.as_bytes())?;

        Ok(destination.lower(memory)? + hlist![length as i32])
    }
}
