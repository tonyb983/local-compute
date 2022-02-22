// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::core::types::ComputeRequest;

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub struct BadRequestError {
    sender: String,
    message: String,
    request: Option<ComputeRequest>,
}

impl BadRequestError {
    #[must_use]
    pub fn new(sender: &str, msg: &str, request: Option<ComputeRequest>) -> Self {
        Self {
            sender: sender.to_string(),
            message: msg.to_string(),
            request,
        }
    }

    #[must_use]
    pub fn without_request(sender: &str, msg: &str) -> Self {
        Self::new(sender, msg, None)
    }

    #[must_use]
    pub fn sender(&self) -> &str {
        &self.sender
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[must_use]
    pub const fn request(&self) -> Option<&ComputeRequest> {
        self.request.as_ref()
    }

    #[must_use]
    pub const fn has_request(&self) -> bool {
        self.request.is_some()
    }
}

impl fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BadRequestError from {}: {}", self.sender, self.message)
    }
}
