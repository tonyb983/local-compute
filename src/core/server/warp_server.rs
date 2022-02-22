// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
mod filters {
    use warp::Filter;

    use super::{handlers, models};
    use crate::{
        core::types::{AddFunctionRequest, RemoveFunctionRequest},
        ComputeRequest,
    };

    /// Extract JSON [`ComputeRequest`] from request body.
    fn json_body_compute_request(
    ) -> impl Filter<Extract = (ComputeRequest,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    /// Extract JSON [`AddFunctionRequest`] from request body.
    fn json_body_add_function(
    ) -> impl Filter<Extract = (AddFunctionRequest,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    /// Extract JSON [`RemoveFunctionRequest`] from request body.
    fn json_body_remove_function(
    ) -> impl Filter<Extract = (RemoveFunctionRequest,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    /// Clone (ref-counted) [`AppState`] for endpoint.
    fn with_app_state(
        state: models::AppState,
    ) -> impl Filter<Extract = (models::AppState,), Error = std::convert::Infallible> + Clone {
        warp::any().map(move || state.clone())
    }

    /// POST /api
    pub fn post_compute_request(
        state: models::AppState,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("api")
            .and(warp::post())
            .and(json_body_compute_request())
            .and(with_app_state(state))
            .and_then(handlers::process_input_handler)
    }

    /// POST /add
    pub fn post_add_function(
        state: models::AppState,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("add")
            .and(warp::post())
            .and(json_body_remove_function())
            .and(with_app_state(state))
            .and_then(handlers::remove_function_handler)
    }

    /// POST /remove
    pub fn post_remove_function(
        state: models::AppState,
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("remove")
            .and(warp::post())
            .and(json_body_add_function())
            .and(with_app_state(state))
            .and_then(handlers::add_function_handler)
    }
}

mod handlers {
    use std::convert::Infallible;

    use warp::Reply;

    use super::models::AppState;
    use crate::{
        core::types::{AddFunctionRequest, AppError, RemoveFunctionRequest},
        ComputeRequest,
    };

    pub async fn add_function_handler(
        input: AddFunctionRequest,
        cfm: AppState,
    ) -> Result<impl warp::Reply, Infallible> {
        let cfm = cfm.lock().await;
        let result = unsafe { cfm.load_plugin(input.lib_path().to_string()).await };
        match result {
            Ok(_) => Ok(hyper::StatusCode::OK.into_response()),
            Err(e) => {
                let error: AppError = e.into();
                Ok(error.into_response())
            }
        }
    }

    pub async fn remove_function_handler(
        input: RemoveFunctionRequest,
        cfm: AppState,
    ) -> Result<impl warp::Reply, Infallible> {
        let cfm = cfm.lock().await;
        let result = cfm.unload_plugin(input.target()).await;
        match result {
            Ok(_) => Ok(hyper::StatusCode::OK.into_response()),
            Err(e) => {
                let error: AppError = e.into();
                Ok(error.into_response())
            }
        }
    }

    pub async fn process_input_handler(
        input: ComputeRequest,
        cfm: AppState,
    ) -> Result<impl warp::Reply, Infallible> {
        let cfm = cfm.lock().await;
        let result = cfm.push_request(&input).await;
        match result {
            Ok(response) => Ok(response.into_response()),
            Err(e) => Ok(e.into_response()),
        }
    }
}

mod models {
    use std::sync::Arc;

    use tokio::sync::Mutex;

    use crate::core::ComputeFunctionManager;

    pub type AppState = Arc<Mutex<ComputeFunctionManager>>;

    pub fn create_app_state() -> AppState {
        Arc::new(Mutex::new(ComputeFunctionManager::with_logger()))
    }
}
