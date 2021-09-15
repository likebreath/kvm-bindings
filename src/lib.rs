// Copyright 2019 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Rust FFI bindings to KVM, generated using [bindgen](https://crates.io/crates/bindgen).

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
#[cfg(feature = "fam-wrappers")]
extern crate vmm_sys_util;

#[cfg(feature = "with-serde")]
extern crate serde;

#[cfg(feature = "with-serde")]
extern crate serde_derive;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::*;

#[cfg(any(target_arch = "aarch", target_arch = "aarch64"))]
mod arm64;
#[cfg(any(target_arch = "aarch", target_arch = "aarch64"))]
pub use self::arm64::*;

#[cfg(all(test, feature = "with-serde"))]
mod tests {
    extern crate serde_json;

    use super::Version;

    #[test]
    fn test_version_ser_deser() {
        let ver = Version {
            arch: "x86_64",
            kernel_ver: "v5.13.0",
            crate_ver: "v0.5.0",
        };
        let ver_str = "{\"arch\":\"x86_64\",\"kernel_ver\":\"v5.13.0\",\"crate_ver\":\"v0.5.0\"}";
        let ver_ser = serde_json::to_string(&ver).unwrap();
        assert_eq!(ver_ser, ver_str.to_string());

        let ver_deser = serde_json::from_str::<Version>(ver_str).unwrap();
        assert_eq!(ver, ver_deser);
    }
}
