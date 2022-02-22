// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::core::types::{
    BadInputError, BadRequestError, GenericStatusCode, LoadingError, TargetComputeFunc,
    UnloadingError,
};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum AppError {
    #[error("{0}")]
    BadInput(BadInputError),
    #[error("{0}")]
    BadRequest(BadRequestError),
    #[error("Target compute function '{0}' not found")]
    TargetNotFound(TargetComputeFunc),
    #[error("Error loading compute function: {0}")]
    Loading(LoadingError),
    #[error("Error unloading compute function: {0}")]
    Unloading(UnloadingError),
    #[error("Unknown error occurred: {0}")]
    Other(String),
    #[error("You should not be seeing this.")]
    None,
}

impl AppError {
    #[must_use]
    pub fn other(msg: &str) -> Self {
        msg.to_string().into()
    }

    #[must_use]
    pub const fn as_generic_status_code(&self) -> GenericStatusCode {
        match self {
            Self::BadInput(_) | Self::BadRequest(_) => GenericStatusCode::BadRequest,
            Self::Unloading(un) => match un {
                UnloadingError::TargetNotFound(_) => GenericStatusCode::NotFound,
                UnloadingError::UnableToUnload(_) => GenericStatusCode::InternalError,
            },
            Self::Loading(load) => match load {
                LoadingError::FunctionNameCollision(_) => GenericStatusCode::Conflict,
                LoadingError::BadPath(_) => GenericStatusCode::PreconditionFailed,
                LoadingError::PathNotFound(_) => GenericStatusCode::NotFound,
                _ => GenericStatusCode::InternalError,
            },
            Self::TargetNotFound(_) => GenericStatusCode::NotFound,
            Self::Other(_) | Self::None => GenericStatusCode::InternalError,
        }
    }

    /// FIXME: Change this to be feature gated (or delete it if a different backend is chosen).
    /// Consume this error and converts it to an [`axum`] [`axum::response::Response`], for use
    /// in [`axum::Router`] and [`axum::Server`].
    #[must_use]
    pub fn into_axum(self) -> axum::response::Response {
        use axum::response::IntoResponse;

        let status = self.as_generic_status_code().to_status_code();
        let body = Json(json!({
            "error": self,
        }));

        (status, body).into_response()
    }

    /// FIXME: Change this to be feature gated (or delete it if a different backend is chosen).
    /// Consume this error and converts it to a [`warp`] [`warp::reply::Response`], fulfilling
    /// the [`warp`] trait [`warp::Reply`], for convenient use in [`warp::Filter`]s.
    pub fn into_warp(self) -> warp::reply::Response {
        use warp::{reply::json, Reply};

        let status = self.as_generic_status_code().to_status_code();
        let mut resp = json(&self).into_response();
        {
            let resp_status = resp.status_mut();
            *resp_status = status;
        }

        resp
    }
}

impl From<BadRequestError> for AppError {
    fn from(e: BadRequestError) -> Self {
        Self::BadRequest(e)
    }
}

impl From<BadInputError> for AppError {
    fn from(e: BadInputError) -> Self {
        Self::BadInput(e)
    }
}

impl From<LoadingError> for AppError {
    fn from(e: LoadingError) -> Self {
        Self::Loading(e)
    }
}

impl From<UnloadingError> for AppError {
    fn from(e: UnloadingError) -> Self {
        Self::Unloading(e)
    }
}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

#[allow(
    clippy::from_over_into,
    reason = "This should only be a one way conversion."
)]
impl Into<GenericStatusCode> for AppError {
    fn into(self) -> GenericStatusCode {
        self.as_generic_status_code()
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.into_axum()
    }
}

impl warp::Reply for AppError {
    fn into_response(self) -> warp::reply::Response {
        self.into_warp()
    }
}
