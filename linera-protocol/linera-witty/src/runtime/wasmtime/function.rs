// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Implementations of [`InstanceWithFunction`] for Wasmtime instances.

use super::{
    parameters::WasmtimeParameters, results::WasmtimeResults, EntrypointInstance, ReentrantInstance,
};
use crate::{memory_layout::FlatLayout, InstanceWithFunction, Runtime, RuntimeError};
use wasmtime::{AsContext, AsContextMut, Extern, TypedFunc};

/// Implements [`InstanceWithFunction`] for the Wasmtime [`Instance`] implementations.
macro_rules! impl_instance_with_function {
    ($instance:ty) => {
        impl<Parameters, Results> InstanceWithFunction<Parameters, Results> for $instance
        where
            Parameters: FlatLayout + WasmtimeParameters,
            Results: FlatLayout + WasmtimeResults,
        {
            type Function = TypedFunc<
                <Parameters as WasmtimeParameters>::Parameters,
                <Results as WasmtimeResults>::Results,
            >;

            fn function_from_export(
                &mut self,
                export: <Self::Runtime as Runtime>::Export,
            ) -> Result<Option<Self::Function>, RuntimeError> {
                Ok(match export {
                    Extern::Func(function) => Some(function.typed(self.as_context())?),
                    _ => None,
                })
            }

            fn call(
                &mut self,
                function: &Self::Function,
                parameters: Parameters,
            ) -> Result<Results, RuntimeError> {
                let results = function.call(self.as_context_mut(), parameters.into_wasmtime())?;

                Ok(Results::from_wasmtime(results))
            }
        }
    };
}

impl_instance_with_function!(EntrypointInstance);
impl_instance_with_function!(ReentrantInstance<'_>);
