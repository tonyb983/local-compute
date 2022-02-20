// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum GenericStatusCode {
    Ok,
    NotFound,
    BadRequest,
    InternalError,
    Conflict,
    PreconditionFailed,
    Other(u16),
    Unknown,
}

impl GenericStatusCode {
    #[must_use]
    pub const fn from_u16(i: u16) -> Self {
        match i {
            200 => Self::Ok,
            400 => Self::BadRequest,
            404 => Self::NotFound,
            409 => Self::Conflict,
            412 => Self::PreconditionFailed,
            500 => Self::InternalError,
            0 => Self::Unknown,
            _ => Self::Other(i),
        }
    }

    #[must_use]
    pub const fn to_u16(self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
            Self::Conflict => 409,
            Self::PreconditionFailed => 412,
            Self::BadRequest => 400,
            Self::InternalError => 500,
            Self::Other(i) => i,
            Self::Unknown => 0,
        }
    }

    pub fn to_status_code(self) -> StatusCode {
        match self {
            Self::Ok => StatusCode::OK,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Conflict => StatusCode::CONFLICT,
            Self::PreconditionFailed => StatusCode::PRECONDITION_FAILED,
            Self::Other(i) => StatusCode::from_u16(i).unwrap_or(StatusCode::IM_A_TEAPOT),
            Self::Unknown => StatusCode::IM_A_TEAPOT,
        }
    }
}

impl From<StatusCode> for GenericStatusCode {
    fn from(code: StatusCode) -> Self {
        match code {
            StatusCode::OK => Self::Ok,
            StatusCode::NOT_FOUND => Self::NotFound,
            StatusCode::BAD_REQUEST => Self::BadRequest,
            StatusCode::INTERNAL_SERVER_ERROR => Self::InternalError,
            _ => Self::Other(code.as_u16()),
        }
    }
}

impl From<u16> for GenericStatusCode {
    fn from(code: u16) -> Self {
        Self::from_u16(code)
    }
}
