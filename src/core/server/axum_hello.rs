// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::net::SocketAddr;

use axum::{response::Json, routing::get, Router};
use lazy_static::lazy_static;

pub async fn run_hello_server(
    addr: SocketAddr,
) -> tokio::task::JoinHandle<Result<(), hyper::Error>> {
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    println!("listening on {}", addr);

    tokio::task::spawn(async move {
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
    })
}

#[allow(clippy::unused_async)]
async fn handler() -> Json<String> {
    lazy_static! {
        static ref COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    }

    Json(format!(
        "Calls: {}",
        COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    ))
}
