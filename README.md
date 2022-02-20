# local-compute

This is me playing around with a) server frameworks in rust, b) async programming in rust, c) a little bit of dynamically loaded rust. The goal is to create a simple server application that will serve as a fake-cloud-function type thing. The end goal is to have it be able to load dynamically written rust crates (that are compiled to `cdylib`) that implement the [`ComputeFunction`](./src/core/types/func.rs) Trait, and use the [`declare_plugin`](./src/core/mod.rs) macro to create a `no_mangle` constructor function.

Usage would look something like this:

```rust
// my-logger-function.rs
// must be built with the same compiler as the project, with --crate-type cdylib

use local_compute::{
    // Main plugin interface
    ComputeFunction,
    // Re-exported from `serde_json` crate for convenience
    JsonValue,
    // Re-exported from `async-trait` crate for convenience
    async_trait,
    // Convenience macro to declare plugin constructor
    declare_plugin,
    // Re-exported from `serde_json` crate for convenience
    json
};

#[derive(Debug, Default)]
struct ComputeLogger {
    pub logs: tokio::sync::Mutex<Vec<serde_json::Value>>,
}

#[async_trait]
impl local_compute::ComputeFunction for ComputeLogger {
    fn name(&self) -> &'static str {
        "ComputeLogger"
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

// declare_plugin takes the type of the ComputeFunction and the default constructor
declare_plugin!(ComputeLogger, ComputeLogger::default);
```