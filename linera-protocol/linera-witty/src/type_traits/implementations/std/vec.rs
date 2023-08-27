// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Implementations of the custom traits for the [`Vec`] type.

use crate::{
    GuestPointer, InstanceWithMemory, Layout, Memory, Runtime, RuntimeError, RuntimeMemory,
    WitLoad, WitStore, WitType,
};
use frunk::{hlist, hlist_pat, HList};

impl<T> WitType for Vec<T>
where
    T: WitType,
{
    const SIZE: u32 = 8;

    type Layout = HList![i32, i32];
}

impl<T> WitLoad for Vec<T>
where
    T: WitLoad,
{
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

        (0..length)
            .map(|index| T::load(memory, address.index::<T>(index)))
            .collect()
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

        (0..length)
            .map(|index| T::load(memory, address.index::<T>(index)))
            .collect()
    }
}

impl<T> WitStore for Vec<T>
where
    T: WitStore,
{
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
        let size = length * T::SIZE;

        let destination = memory.allocate(size)?;

        destination.store(memory, location)?;
        length.store(memory, location.after::<GuestPointer>())?;

        self.iter()
            .zip(0..)
            .try_for_each(|(element, index)| element.store(memory, destination.index::<T>(index)))
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
        let size = length * T::SIZE;

        let destination = memory.allocate(size)?;

        self.iter().zip(0..).try_for_each(|(element, index)| {
            element.store(memory, destination.index::<T>(index))
        })?;

        Ok(destination.lower(memory)? + hlist![length as i32])
    }
}
