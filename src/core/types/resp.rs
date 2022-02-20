// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ComputeJsonResponse {
    data: JsonValue,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ComputeResponse {
    NoContent,
    Json(ComputeJsonResponse),
}

impl Default for ComputeResponse {
    fn default() -> Self {
        Self::NoContent
    }
}

impl ComputeResponse {
    #[must_use]
    pub const fn empty() -> Self {
        Self::NoContent
    }

    #[must_use]
    pub fn from_data(data: JsonValue) -> Self {
        Self::Json(data.into())
    }
}

impl From<Option<JsonValue>> for ComputeResponse {
    fn from(data: Option<JsonValue>) -> Self {
        match data {
            Some(data) => Self::from_data(data),
            None => Self::empty(),
        }
    }
}

impl From<JsonValue> for ComputeResponse {
    fn from(data: JsonValue) -> Self {
        Self::from_data(data)
    }
}

impl From<JsonValue> for ComputeJsonResponse {
    fn from(data: JsonValue) -> Self {
        Self { data }
    }
}

impl From<()> for ComputeResponse {
    fn from(_: ()) -> Self {
        Self::empty()
    }
}
