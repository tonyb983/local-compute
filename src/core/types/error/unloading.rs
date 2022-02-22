// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::types::TargetComputeFunc;

/// An error that occurs during the unloading of a dynamic compute function.
#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum UnloadingError {
    /// The function indicated by the contained [`TargetComputeFunc`] was not found.
    TargetNotFound(TargetComputeFunc),
    /// An error occurred while attempting to unload the function.
    UnableToUnload(String),
}

impl fmt::Display for UnloadingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnloadingError::TargetNotFound(target) => {
                write!(f, "Target '{}' not found in loaded functions", target)
            }
            UnloadingError::UnableToUnload(msg) => {
                write!(f, "Unable to unload target. Reason: {}", msg)
            }
        }
    }
}
