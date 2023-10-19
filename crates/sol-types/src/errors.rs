// Copyright 2015-2020 Parity Technologies
// Copyright 2023-2023 Alloy Contributors

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::abi;
use alloc::{borrow::Cow, string::String};
use core::fmt;

/// ABI result type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// ABI Encoding and Decoding errors.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// A typecheck detected a word that does not match the data type.
    TypeCheckFail {
        /// The Solidity type we failed to produce.
        expected_type: Cow<'static, str>,
        /// Hex-encoded data.
        data: String,
    },

    /// Overran deserialization buffer.
    Overrun,

    /// Trailing bytes in deserialization buffer.
    BufferNotEmpty,

    /// Validation reserialization did not match input.
    ReserMismatch,

    /// Invalid enum value.
    InvalidEnumValue {
        /// The name of the enum.
        name: &'static str,
        /// The invalid value.
        value: u8,
        /// The maximum valid value.
        max: u8,
    },

    /// Unknown selector.
    UnknownSelector {
        /// The type name.
        name: &'static str,
        /// The unknown selector.
        selector: alloy_primitives::FixedBytes<4>,
    },

    /// Hex error.
    FromHexError(hex::FromHexError),

    /// Other errors.
    Other(Cow<'static, str>),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FromHexError(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TypeCheckFail {
                expected_type,
                data,
            } => write!(
                f,
                "type check failed for \"{expected_type}\" with data: {data}",
            ),
            Self::Overrun => f.write_str("buffer overrun while deserializing"),
            Self::BufferNotEmpty => f.write_str("buffer not empty after deserialization"),
            Self::ReserMismatch => f.write_str("reserialization did not match original"),
            Self::InvalidEnumValue { name, value, max } => write!(
                f,
                "`{value}` is not a valid {name} enum value (max: `{max}`)"
            ),
            Self::UnknownSelector { name, selector } => {
                write!(f, "unknown selector `{selector}` for {name}")
            }
            Self::FromHexError(e) => e.fmt(f),
            Self::Other(e) => f.write_str(e),
        }
    }
}

impl Error {
    /// Instantiates a new error with a static str.
    #[cold]
    pub fn custom(s: impl Into<Cow<'static, str>>) -> Self {
        Self::Other(s.into())
    }

    /// Instantiates a new [`Error::TypeCheckFail`] with the provided data.
    #[cold]
    pub fn type_check_fail_sig(mut data: &[u8], signature: &'static str) -> Self {
        if data.len() > 4 {
            data = &data[..4];
        }
        let expected_type = signature.split('(').next().unwrap();
        Self::type_check_fail(data, expected_type)
    }

    /// Instantiates a new [`Error::TypeCheckFail`] with the provided token.
    #[cold]
    pub fn type_check_fail_token<T: crate::SolType>(token: &T::TokenType<'_>) -> Self {
        Self::type_check_fail(&abi::encode(token), T::NAME)
    }

    /// Instantiates a new [`Error::TypeCheckFail`] with the provided data.
    #[cold]
    pub fn type_check_fail(data: &[u8], expected_type: impl Into<Cow<'static, str>>) -> Self {
        Self::TypeCheckFail {
            expected_type: expected_type.into(),
            data: hex::encode(data),
        }
    }

    /// Instantiates a new [`Error::UnknownSelector`] with the provided data.
    #[cold]
    pub fn unknown_selector(name: &'static str, selector: [u8; 4]) -> Self {
        Self::UnknownSelector {
            name,
            selector: selector.into(),
        }
    }
}

impl From<hex::FromHexError> for Error {
    fn from(value: hex::FromHexError) -> Self {
        Self::FromHexError(value)
    }
}
