// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::core::types::{AppInput, ComputeRequest, GenericStatusCode, TargetComputeFunc};

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub struct BadRequestError {
    pub sender: String,
    pub message: String,
    pub request: ComputeRequest,
}

impl fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BadRequestError from {}: {}", self.sender, self.message)
    }
}

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub struct BadInputError {
    pub message: String,
    pub input: AppInput,
}

impl fmt::Display for BadInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BadInputError: {}", self.message)
    }
}

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum LoadingError {
    BadPath(String),
    PathNotFound(String),
    LibraryLoadFailure(String),
    ConstructorLoadFailure(String),
    ConstructorCallFailure,
    FunctionNameCollision(String),
}

impl LoadingError {
    #[must_use]
    pub fn lib_load_failure<S: ToString>(err: &S) -> Self {
        Self::LibraryLoadFailure(err.to_string())
    }

    #[must_use]
    pub fn ctor_load_failure<S: ToString>(err: &S) -> Self {
        Self::ConstructorLoadFailure(err.to_string())
    }

    #[must_use]
    pub fn name_collision<S: ToString>(err: &S) -> Self {
        Self::FunctionNameCollision(err.to_string())
    }

    #[must_use]
    pub fn bad_path<S: ToString>(err: &S) -> Self {
        Self::BadPath(err.to_string())
    }

    #[must_use]
    pub fn path_not_found<S: ToString>(err: &S) -> Self {
        Self::PathNotFound(err.to_string())
    }

    #[must_use]
    pub const fn ctor_call_failure() -> Self {
        Self::ConstructorCallFailure
    }

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
}

impl std::fmt::Display for LoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Error, Deserialize, Serialize, Clone)]
pub enum UnloadingError {
    TargetNotFound(TargetComputeFunc),
    UnableToUnload(String),
}

impl std::fmt::Display for UnloadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub type AppResult<T> = Result<T, AppError>;

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
        let status: axum::http::StatusCode = self.as_generic_status_code().to_status_code();
        let body = Json(json!({
            "error": self,
        }));

        (status, body).into_response()
    }
}
