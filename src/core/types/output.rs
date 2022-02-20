// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::core::types::ComputeResponse;

#[derive(Debug, Deserialize, Serialize)]
pub enum AppOutput {
    ComputeResponse(ComputeResponse),
    AddFunctionSuccess,
    RemoveFunctionSuccess,
    // Other(String),
    Other { message: Option<String> },
}

impl AppOutput {
    pub const fn add_function_success() -> Self {
        Self::AddFunctionSuccess
    }

    pub const fn remove_function_success() -> Self {
        Self::RemoveFunctionSuccess
    }

    pub const fn compute_response(compute_response: ComputeResponse) -> Self {
        Self::ComputeResponse(compute_response)
    }

    pub fn other(msg: Option<impl ToString>) -> Self {
        Self::Other {
            message: msg.map(|s| s.to_string()),
        }
    }
}

impl IntoResponse for AppOutput {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::ComputeResponse(cr) => (StatusCode::OK, Json(cr)).into_response(),
            Self::AddFunctionSuccess | Self::RemoveFunctionSuccess => {
                StatusCode::OK.into_response()
            }
            Self::Other { message: maybe_msg } => match maybe_msg {
                Some(message) => (StatusCode::ACCEPTED, message).into_response(),
                None => StatusCode::NO_CONTENT.into_response(),
            },
        }
    }
}
