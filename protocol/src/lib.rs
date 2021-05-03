//  Copyright (c) Facebook, Inc. and its affiliates.
//  SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]
#![crate_name = "protocol"]

#[macro_use]
extern crate log;

pub mod fileio;
pub mod private_id;

#[cfg(not(target_arch = "wasm32"))]
pub mod cross_psi;

#[cfg(not(target_arch = "wasm32"))]
pub mod pjc;

#[cfg(not(target_arch = "wasm32"))]
pub mod private_id_multi_key;

#[cfg(not(target_arch = "wasm32"))]
pub mod suid_create;

pub mod shared {
    extern crate crypto;

    use crypto::prelude::*;
    use std::path::Path;

    /// Type of the input expected right now
    pub type TDomain = u64;
    /// Feature matrix type
    pub type TFeatures = Vec<Vec<TDomain>>;

    /// trait to get the encryption key
    #[cfg(not(target_arch = "wasm32"))]
    pub trait ShareableEncKey {
        fn get_he_public_key(&self) -> EncryptionKey;
    }

    pub trait LoadData {
        fn load_data<T>(&self, input_path: T)
        where
            T: AsRef<Path>;
    }

    pub trait Reveal {
        fn reveal<T: AsRef<Path>>(&self, path: T);
    }
}
