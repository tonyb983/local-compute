// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::any::Any;

use async_trait::async_trait;

use crate::core::types::{BadRequestError, ComputeRequest, ComputeResponse};

#[async_trait]
/// A plugin which allows you to add extra functionality to the REST client.
pub trait ComputeFunction: Any + Send + Sync + std::fmt::Debug {
    /// Get a name describing the `Plugin`. This will be used as the identifier
    /// for any callers who are trying to reach your function.
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    /// Other than `name`, this is the only function that **must** be implemented.
    /// It takes a **non-mutable** self to encourage interior mutability and thread-safety.
    /// See the [`ComputeRequest`] documentation for more information on the input.
    /// If your function has no output, [`ComputeResponse::empty`] can be used to
    /// create an empty response.
    async fn receive_request(
        &self,
        request: &ComputeRequest,
    ) -> Result<ComputeResponse, BadRequestError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[derive(Debug, Default)]
    struct FakePlugin;

    #[derive(Debug, Default)]
    struct ShittyCloudLogger {
        pub logs: tokio::sync::Mutex<Vec<serde_json::Value>>,
    }

    #[async_trait]
    impl ComputeFunction for FakePlugin {
        fn name(&self) -> &'static str {
            "FakePlugin"
        }

        async fn receive_request(
            &self,
            _request: &ComputeRequest,
        ) -> Result<ComputeResponse, BadRequestError> {
            Ok(ComputeResponse::from_data(
                json!({ "message": "Hello, World!" }),
            ))
        }
    }

    #[async_trait]
    impl ComputeFunction for ShittyCloudLogger {
        fn name(&self) -> &'static str {
            "ShittyCloudLogger"
        }

        async fn receive_request(
            &self,
            request: &ComputeRequest,
        ) -> Result<ComputeResponse, BadRequestError> {
            {
                let mut lock = self.logs.lock().await;
                lock.push(request.data().clone());
            }
            Ok(ComputeResponse::empty())
        }
    }
}
