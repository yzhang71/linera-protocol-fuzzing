// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Internal module with code generated by [`wit-bindgen`](https://github.com/jvff/wit-bindgen).

#![allow(missing_docs)]

// Export the contract interface.
wit_bindgen_guest_rust::export!(
    export_macro = "export_contract"
    types_path = "contract::wit_types"
    reexported_crate_path = "wit_bindgen_guest_rust"
    "contract.wit"
);
