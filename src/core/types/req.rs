// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::core::types::TargetComputeFunc;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AddFunctionRequest(String);

impl AddFunctionRequest {
    #[must_use]
    pub fn new(library_path: String) -> Self {
        Self(library_path)
    }

    #[must_use]
    pub fn lib_path(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RemoveFunctionRequest(TargetComputeFunc);
impl RemoveFunctionRequest {
    #[must_use]
    pub fn new(target: TargetComputeFunc) -> Self {
        Self(target)
    }

    #[must_use]
    pub fn target(&self) -> &TargetComputeFunc {
        &self.0
    }
}
