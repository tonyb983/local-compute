// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::core::types::{ComputeResponse, GenericStatusCode};

#[derive(Debug, Deserialize, Serialize)]
pub enum AppOutput {
    ComputeResponse(ComputeResponse),
    AddFunctionSuccess,
    RemoveFunctionSuccess,
    // Other(String),
    Other {
        status: GenericStatusCode,
        message: Option<String>,
    },
}

impl AppOutput {
    /// Create a new [`AppOutput::AddFunctionSuccess`].
    pub const fn add_function_success() -> Self {
        Self::AddFunctionSuccess
    }

    /// Create a new [`AppOutput::RemoveFunctionSuccess`].
    pub const fn remove_function_success() -> Self {
        Self::RemoveFunctionSuccess
    }

    /// Create a new [`AppOutput::ComputeResponse`] with the given [`ComputeResponse`].
    pub const fn compute_response(compute_response: ComputeResponse) -> Self {
        Self::ComputeResponse(compute_response)
    }

    /// Create an [`AppOutput::Other`] instance with the given code and message.
    pub fn other(code: GenericStatusCode, msg: Option<impl ToString>) -> Self {
        Self::Other {
            status: code,
            message: msg.map(|s| s.to_string()),
        }
    }

    /// Gets the http status code for this output.
    pub fn status(&self) -> hyper::StatusCode {
        match self {
            Self::AddFunctionSuccess => StatusCode::CREATED,
            Self::RemoveFunctionSuccess => StatusCode::OK,
            Self::ComputeResponse(cr) => cr.status().to_status_code(),
            Self::Other { status, .. } => (*status).to_status_code(),
        }
    }

    /// Gets the data in this output, if any.
    pub fn data(&self) -> Option<serde_json::Value> {
        use serde_json::json;

        match self {
            Self::ComputeResponse(cr) => cr.data(),
            Self::Other { message, .. } => message.as_ref().map(|s| json!(s)),
            Self::AddFunctionSuccess | Self::RemoveFunctionSuccess => None,
        }
    }

    /// FIXME: Change this to be feature gated (or delete it if a different backend is chosen).
    /// Consume this [`AppOutput`] and converts it to a [`warp`] [`warp::reply::Response`], fulfilling
    /// the [`warp`] trait [`warp::Reply`], for convenient use in [`warp::Filter`]s.
    pub fn into_warp(self) -> warp::reply::Response {
        use warp::{
            reply::{json, with_status},
            Reply,
        };
        let code = self.status();
        match self.data() {
            Some(d) => with_status(json(&d).into_response(), code).into_response(),
            None => code.into_response(),
        }
    }

    /// FIXME: Change this to be feature gated (or delete it if a different backend is chosen).
    /// Consume this [`AppOutput`] and converts it to a [`axum`] [`axum::response::Response`].
    pub fn into_axum(self) -> axum::response::Response {
        use axum::{response::IntoResponse, Json};

        let code = self.status();
        match self.data() {
            Some(s) => (code, Json(s)).into_response(),
            None => code.into_response(),
        }
    }
}

impl axum::response::IntoResponse for AppOutput {
    fn into_response(self) -> axum::response::Response {
        self.into_axum()
    }
}

impl warp::Reply for AppOutput {
    fn into_response(self) -> warp::reply::Response {
        self.into_warp()
    }
}
