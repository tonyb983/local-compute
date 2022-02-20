// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// TODO: See extended comment on ComputeResponse below.
/// An input identifier that indicates which compute function this request
/// is intended for.
///
/// For now it is just a string, but i'd like to have it be
/// more of a file URI. I'm sure whatever server framework I end up using will
/// have utilities (or `hyper` itself might have something) to help with this
/// functionality.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TargetComputeFunc(String);
impl TargetComputeFunc {
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self(name)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TargetComputeFunc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// TODO: While it is good that I have already extracted [`TargetComputeFunc`], I need to be
//       a better job of handling input dispatch. The target needs to be parsed to get the
//       basename, the extended path, and maybe even parameters and queries. The more options
//       an implementer has, the better.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ComputeRequest {
    target: TargetComputeFunc,
    data: JsonValue,
}

impl ComputeRequest {
    #[must_use]
    pub const fn new(target: TargetComputeFunc, data: JsonValue) -> Self {
        Self { target, data }
    }

    #[must_use]
    pub const fn target(&self) -> &TargetComputeFunc {
        &self.target
    }

    #[must_use]
    pub const fn data(&self) -> &JsonValue {
        &self.data
    }
}
