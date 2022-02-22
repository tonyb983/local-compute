// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::core::types::GenericStatusCode;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ComputeJsonResponse {
    status: GenericStatusCode,
    data: JsonValue,
}

impl ComputeJsonResponse {
    pub const fn new(status: GenericStatusCode, data: JsonValue) -> Self {
        Self { status, data }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ComputeResponse {
    NoContent(GenericStatusCode),
    Json(ComputeJsonResponse),
}

impl Default for ComputeResponse {
    fn default() -> Self {
        Self::ok()
    }
}

impl ComputeResponse {
    /// Create a new [`ComputeResponse`] with status `Ok` and no data.
    #[must_use]
    pub const fn ok() -> Self {
        Self::NoContent(GenericStatusCode::Ok)
    }

    /// Create a new [`ComputeResponse`] with no content and the given status.
    #[must_use]
    pub const fn status_only(status: GenericStatusCode) -> Self {
        Self::NoContent(status)
    }

    /// Create a new [`ComputeResponse`] with status `Ok` and the given JSON data.
    #[must_use]
    pub const fn json_ok(data: JsonValue) -> Self {
        Self::Json(ComputeJsonResponse {
            status: GenericStatusCode::Ok,
            data,
        })
    }

    /// Create a new [`ComputeResponse`] with the given status code and json data.
    #[must_use]
    pub const fn json(status: GenericStatusCode, data: JsonValue) -> Self {
        Self::Json(ComputeJsonResponse { status, data })
    }
}

impl ComputeResponse {
    /// Get the [`GenericStatusCode`] for this response.
    #[must_use]
    pub const fn status(&self) -> GenericStatusCode {
        match self {
            Self::NoContent(status) => *status,
            Self::Json(json) => json.status,
        }
    }

    /// Get the [`hyper::StatusCode`] for this response.
    #[must_use]
    pub fn http_status(&self) -> StatusCode {
        self.status().to_status_code()
    }

    /// Gets the inner json data of this response if it contains any, None otherwise.
    #[must_use]
    pub fn data(&self) -> Option<JsonValue> {
        match self {
            ComputeResponse::NoContent(_) => None,
            ComputeResponse::Json(ComputeJsonResponse { data, .. }) => Some(data.clone()),
        }
    }

    /// FIXME: Change this to be feature gated (or delete it if a different backend is chosen).
    /// Consume this [`ComputeResponse`] and converts it to a [`warp`] [`warp::reply::Response`], fulfilling
    /// the [`warp`] trait [`warp::Reply`], for convenient use in [`warp::Filter`]s.
    #[must_use]
    pub fn into_warp(self) -> warp::reply::Response {
        use warp::{
            reply::{json, with_status},
            Reply,
        };
        let code = self.http_status();
        match self.data() {
            Some(val) => with_status(json(&val).into_response(), code).into_response(),
            None => code.into_response(),
        }
    }

    /// FIXME: Change this to be feature gated (or delete it if a different backend is chosen).
    /// Consume this [`AppOutput`] and converts it to a [`axum`] [`axum::response::Response`].
    #[must_use]
    pub fn into_axum(self) -> axum::response::Response {
        use axum::{response::IntoResponse, Json};
        match self {
            Self::NoContent(status) => status.to_status_code().into_response(),
            Self::Json(ComputeJsonResponse { status, data }) => {
                (status.to_status_code(), Json(data)).into_response()
            }
        }
    }
}

// ====== Server Impls ======

impl axum::response::IntoResponse for ComputeResponse {
    fn into_response(self) -> axum::response::Response {
        self.into_axum()
    }
}

impl warp::Reply for ComputeResponse {
    fn into_response(self) -> warp::reply::Response {
        self.into_warp()
    }
}

// ====== Misc Convenience Impls ======

impl From<Option<JsonValue>> for ComputeResponse {
    fn from(data: Option<JsonValue>) -> Self {
        match data {
            Some(data) => Self::json_ok(data),
            None => Self::default(),
        }
    }
}
impl From<JsonValue> for ComputeResponse {
    fn from(data: JsonValue) -> Self {
        Self::json_ok(data)
    }
}
impl From<JsonValue> for ComputeJsonResponse {
    fn from(data: JsonValue) -> Self {
        Self::new(GenericStatusCode::default(), data)
    }
}
impl From<()> for ComputeResponse {
    fn from(_: ()) -> Self {
        Self::ok()
    }
}
