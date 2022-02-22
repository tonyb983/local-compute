// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// An error that occurs during the loading of a dynamic compute function.
#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum LoadingError {
    /// The path provided was invalid.
    BadPath(String),
    /// No library could be found **or there were insufficient permissions to access it**.
    PathNotFound(String),
    /// The library file was found but could not be loaded.
    LibraryLoadFailure(String),
    /// The `_plugin_create` function could not be loaded from the library.
    ConstructorLoadFailure(String),
    /// The `_plugin_create` function returned a null pointer.
    ConstructorCallFailure,
    /// The plugin manager already contains an instance of the given plugin.
    FunctionNameCollision(String),
}

impl LoadingError {
    /// Create a [`LoadingError::LibraryLoadFailure`] with the given message.
    #[must_use]
    pub fn lib_load_failure<S: ToString>(err: &S) -> Self {
        Self::LibraryLoadFailure(err.to_string())
    }

    /// Create a [`LoadingError::ConstructorLoadFailure`] with the given message.
    #[must_use]
    pub fn ctor_load_failure<S: ToString>(err: &S) -> Self {
        Self::ConstructorLoadFailure(err.to_string())
    }

    /// Create a [`LoadingError::FunctionNameCollision`] with the given message.
    #[must_use]
    pub fn name_collision<S: ToString>(err: &S) -> Self {
        Self::FunctionNameCollision(err.to_string())
    }

    /// Create a [`LoadingError::BadPath`] with the given message.
    #[must_use]
    pub fn bad_path<S: ToString>(err: &S) -> Self {
        Self::BadPath(err.to_string())
    }

    /// Create a [`LoadingError::PathNotFound`] with the given message.
    #[must_use]
    pub fn path_not_found<S: ToString>(err: &S) -> Self {
        Self::PathNotFound(err.to_string())
    }

    /// Create a [`LoadingError::ConstructorCallFailure`] with the given message.
    #[must_use]
    pub const fn ctor_call_failure() -> Self {
        Self::ConstructorCallFailure
    }

    /// Gets the message contained in this [`LoadingError`], unless it is a
    /// [`LoadingError::ConstructorCallFailure`], in which case it returns None.
    #[must_use]
    pub fn inner_msg(&self) -> Option<&str> {
        match self {
            Self::LibraryLoadFailure(s)
            | Self::PathNotFound(s)
            | Self::ConstructorLoadFailure(s)
            | Self::FunctionNameCollision(s)
            | Self::BadPath(s) => Some(s),
            Self::ConstructorCallFailure => None,
        }
    }

    /// Determines if this [`LoadingError`] has an inner message.
    #[must_use]
    pub fn has_msg(&self) -> bool {
        match self {
            Self::LibraryLoadFailure(s)
            | Self::PathNotFound(s)
            | Self::ConstructorLoadFailure(s)
            | Self::FunctionNameCollision(s)
            | Self::BadPath(s) => !s.is_empty(),
            Self::ConstructorCallFailure => false,
        }
    }
}

impl fmt::Display for LoadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LibraryLoadFailure(msg) => write!(
                f,
                "ComputeFunction Library was unable to be loaded: {}",
                msg
            ),
            Self::ConstructorLoadFailure(msg) => {
                write!(f, "ComputeFunction ctor not found: {}", msg)
            }
            Self::FunctionNameCollision(msg) => {
                write!(f, "ComputeFunction name collision: {}", msg)
            }
            Self::ConstructorCallFailure => {
                write!(f, "ComputeFunction construction failed (returned null ptr)")
            }
            Self::PathNotFound(msg) => write!(f, "No library found at path: {}", msg),
            Self::BadPath(msg) => write!(
                f,
                "Given path is badly formed (all paths must be absolute): {}",
                msg
            ),
        }
    }
}
