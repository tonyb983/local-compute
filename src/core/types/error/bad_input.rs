// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::types::AppInput;

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub struct BadInputError {
    message: String,
    input: AppInput,
}

impl BadInputError {
    /// Create a new [`BadInputError`] with the given message and input.
    #[must_use]
    pub fn new(message: &str, input: AppInput) -> Self {
        Self {
            message: message.to_string(),
            input,
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn input(&self) -> &AppInput {
        &self.input
    }
}

impl fmt::Display for BadInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BadInputError: {}", self.message)
    }
}
