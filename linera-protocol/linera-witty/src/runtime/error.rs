// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Common error type for usage of different Wasm runtimes.

use std::{num::TryFromIntError, string::FromUtf8Error};
use thiserror::Error;

/// Errors that can occur when using a Wasm runtime.
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// Attempt to allocate a buffer larger than `i32::MAX`.
    #[error("Requested allocation size is too large")]
    AllocationTooLarge,

    /// Call to `cabi_realloc` returned a negative value instead of a valid address.
    #[error("Memory allocation failed")]
    AllocationFailed,

    /// Attempt to deallocate an address that's after `i32::MAX`.
    #[error("Attempt to deallocate an invalid address")]
    DeallocateInvalidAddress,

    /// Attempt to load a function not exported from a module.
    #[error("Function `{_0}` could not be found in the module's exports")]
    FunctionNotFound(String),

    /// Attempt to load a function with a name that's used for a different import in the module.
    #[error("Export `{_0}` is not a function")]
    NotAFunction(String),

    /// Attempt to load the memory export from a module that doesn't export it.
    #[error("Failed to load `memory` export")]
    MissingMemory,

    /// Attempt to load the memory export from a module that exports it as something else.
    #[error("Unexpected type for `memory` export")]
    NotMemory,

    /// Attempt to load a string from a sequence of bytes that doesn't contain a UTF-8 string.
    #[error("Failed to load string from non-UTF-8 bytes")]
    InvalidString(#[from] FromUtf8Error),

    /// Attempt to create a `GuestPointer` from an invalid address representation.
    #[error("Invalid address read")]
    InvalidNumber(#[from] TryFromIntError),

    /// Attempt to load an `enum` type but the discriminant doesn't match any of the variants.
    #[error("Unexpected variant discriminant")]
    InvalidVariant,

    /// Wasmer runtime error.
    #[cfg(feature = "wasmer")]
    #[error(transparent)]
    Wasmer(#[from] wasmer::RuntimeError),

    /// Attempt to access an invalid memory address using Wasmer.
    #[cfg(feature = "wasmer")]
    #[error(transparent)]
    WasmerMemory(#[from] wasmer::MemoryAccessError),

    /// Wasmtime error.
    #[cfg(feature = "wasmtime")]
    #[error(transparent)]
    Wasmtime(#[from] anyhow::Error),

    /// Wasmtime trap during execution.
    #[cfg(feature = "wasmtime")]
    #[error(transparent)]
    WasmtimeTrap(#[from] wasmtime::Trap),
}
