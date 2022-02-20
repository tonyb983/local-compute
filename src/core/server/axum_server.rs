// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::Extension,
    routing::{post, IntoMakeService},
    AddExtensionLayer, Json, Router, Server,
};
use hyper::server::conn::AddrIncoming;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

use crate::core::{
    types::{AppInput, AppOutput, AppResult},
    ComputeFunctionManager,
};

/// Used to simplify [`axum::extract::Extension`] extraction of the [`ComputeFunctionManager`]
type MutexManager = Arc<Mutex<ComputeFunctionManager>>;
/// Used to simplify [`axum::extract::Extension`] extraction of the [`ComputeFunctionManager`].
/// Alternative to [`MutexManager`] that prioritizes writers which seems like it should
/// be more efficient in this particular use case.
type RwLockManager = Arc<RwLock<ComputeFunctionManager>>;

async fn process_input_mutex(pm: &MutexManager, input: &AppInput) -> AppResult<AppOutput> {
    match input {
        AppInput::AddComputeFunction(lib) => unsafe {
            pm.lock()
                .await
                .load_plugin(lib)
                .map(|_| AppOutput::AddFunctionSuccess)
                .map_err(std::convert::Into::into)
        },
        AppInput::RemoveComputeFunction(target) => pm
            .lock()
            .await
            .unload_plugin(target)
            .map(|_| AppOutput::RemoveFunctionSuccess)
            .map_err(std::convert::Into::into),
        AppInput::Execute(req) => pm
            .lock()
            .await
            .push_request(req)
            .await
            .map(AppOutput::compute_response),
    }
}

async fn process_input_rw(pm: RwLockManager, input: &AppInput) -> AppResult<AppOutput> {
    match input {
        AppInput::AddComputeFunction(lib) => unsafe {
            let mut pm_writer = pm.write_owned().await;
            pm_writer
                .load_plugin(lib)
                .map(|_| AppOutput::AddFunctionSuccess)
                .map_err(std::convert::Into::into)
        },
        AppInput::RemoveComputeFunction(target) => {
            let mut pm_writer = pm.write_owned().await;
            pm_writer
                .unload_plugin(target)
                .map(|_| AppOutput::RemoveFunctionSuccess)
                .map_err(std::convert::Into::into)
        }
        AppInput::Execute(req) => {
            let pm_reader = pm.read_owned().await;
            pm_reader
                .push_request(req)
                .await
                .map(AppOutput::compute_response)
        }
    }
}

async fn process_input_mutex_handler(
    Json(payload): Json<AppInput>,
    Extension(state): Extension<MutexManager>,
) -> AppResult<AppOutput> {
    process_input_mutex(&state, &payload).await
}

async fn process_input_rw_handler(
    Json(payload): Json<AppInput>,
    Extension(state): Extension<RwLockManager>,
) -> AppResult<AppOutput> {
    process_input_rw(state.clone(), &payload).await
}

async fn fake_main() {
    use tokio::sync::oneshot;
    let (sender, receiver): (oneshot::Sender<()>, oneshot::Receiver<()>) = oneshot::channel::<()>();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let res = run_rw_axum_with_shutdown(&addr, receiver);

    tokio::task::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(10 * 1000)).await;
        match sender.send(()) {
            Ok(_) => println!("Sender sent successfully."),
            Err(_) => eprintln!("Sender failed sending."),
        }
    });

    let msg = res.await;
    println!("{:?}", msg);
}

pub async fn run_rw_axum_with_shutdown(
    addr: &std::net::SocketAddr,
    rx: tokio::sync::oneshot::Receiver<()>,
) -> tokio::task::JoinHandle<String> {
    let app: Router = Router::new()
        .route("/", post(process_input_rw_handler))
        .layer(AddExtensionLayer::new(RwLockManager::default()));

    let server = axum::Server::bind(addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async move {
            rx.await.ok();
        });

    tokio::task::spawn(async move {
        if let Err(e) = server.await {
            format!("server error: {}", e)
        } else {
            "server shutdown without error".to_string()
        }
    })
}

pub async fn run_axum_with_mutex(addr: &std::net::SocketAddr) -> Result<(), hyper::Error> {
    let app: Router = Router::new()
        .route("/", post(process_input_mutex_handler))
        .layer(AddExtensionLayer::new(MutexManager::default()));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
}

pub async fn run_axum_with_rw(addr: &std::net::SocketAddr) -> Result<(), hyper::Error> {
    let app: Router = Router::new()
        .route("/", post(process_input_rw_handler))
        .layer(AddExtensionLayer::new(RwLockManager::default()));

    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum ServerSyncType {
    Mutex,
    RwLock,
}

impl std::fmt::Display for ServerSyncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerSyncType::Mutex => write!(f, "Mutex"),
            ServerSyncType::RwLock => write!(f, "RwLock"),
        }
    }
}

#[derive(Debug)]
pub struct AxumServer {
    addr: std::net::SocketAddr,
    router: Router,
    is_running: bool,
    server: Option<hyper::server::Server<AddrIncoming, IntoMakeService<Router>>>,
    shutdown_signal: tokio::sync::oneshot::Receiver<()>,
    sync_type: ServerSyncType,
}

impl AxumServer {
    async fn process_input_mutex(pm: &MutexManager, input: &AppInput) -> AppResult<AppOutput> {
        match input {
            AppInput::AddComputeFunction(lib) => unsafe {
                pm.lock()
                    .await
                    .load_plugin(lib)
                    .map(|_| AppOutput::AddFunctionSuccess)
                    .map_err(std::convert::Into::into)
            },
            AppInput::RemoveComputeFunction(target) => pm
                .lock()
                .await
                .unload_plugin(target)
                .map(|_| AppOutput::RemoveFunctionSuccess)
                .map_err(std::convert::Into::into),
            AppInput::Execute(req) => pm
                .lock()
                .await
                .push_request(req)
                .await
                .map(AppOutput::compute_response),
        }
    }

    async fn process_input_rw(pm: RwLockManager, input: &AppInput) -> AppResult<AppOutput> {
        match input {
            AppInput::AddComputeFunction(lib) => unsafe {
                let mut pm_writer = pm.write_owned().await;
                pm_writer
                    .load_plugin(lib)
                    .map(|_| AppOutput::AddFunctionSuccess)
                    .map_err(std::convert::Into::into)
            },
            AppInput::RemoveComputeFunction(target) => {
                let mut pm_writer = pm.write_owned().await;
                pm_writer
                    .unload_plugin(target)
                    .map(|_| AppOutput::RemoveFunctionSuccess)
                    .map_err(std::convert::Into::into)
            }
            AppInput::Execute(req) => {
                let pm_reader = pm.read_owned().await;
                pm_reader
                    .push_request(req)
                    .await
                    .map(AppOutput::compute_response)
            }
        }
    }

    async fn input_handler_mutex(
        Json(payload): Json<AppInput>,
        Extension(state): Extension<MutexManager>,
    ) -> AppResult<AppOutput> {
        Self::process_input_mutex(&state, &payload).await
    }

    async fn input_handler_rw(
        Json(payload): Json<AppInput>,
        Extension(state): Extension<RwLockManager>,
    ) -> AppResult<AppOutput> {
        Self::process_input_rw(state.clone(), &payload).await
    }

    fn init_as_rw(
        addr: SocketAddr,
        start: bool,
        shutdown_receiver: tokio::sync::oneshot::Receiver<()>,
    ) -> Self {
        let router = Router::new()
            .route("/", post(Self::input_handler_rw))
            .layer(AddExtensionLayer::new(RwLockManager::default()));

        let server: Option<Server<AddrIncoming, IntoMakeService<Router>>> = if start {
            Some(Server::bind(&addr).serve(router.clone().into_make_service()))
        } else {
            None
        };

        Self {
            addr,
            router,
            is_running: start,
            server,
            shutdown_signal: shutdown_receiver,
            sync_type: ServerSyncType::RwLock,
        }
    }

    fn init_as_mutex(
        addr: SocketAddr,
        start: bool,
        shutdown_receiver: tokio::sync::oneshot::Receiver<()>,
    ) -> Self {
        let router = Router::new()
            .route("/", post(Self::input_handler_mutex))
            .layer(AddExtensionLayer::new(MutexManager::default()));

        let server: Option<Server<AddrIncoming, IntoMakeService<Router>>> = if start {
            Some(Server::bind(&addr).serve(router.clone().into_make_service()))
        } else {
            None
        };

        Self {
            addr,
            router,
            is_running: start,
            server,
            shutdown_signal: shutdown_receiver,
            sync_type: ServerSyncType::Mutex,
        }
    }

    pub fn init(
        addr: &SocketAddr,
        start: bool,
        sync_type: ServerSyncType,
        shutdown_receiver: tokio::sync::oneshot::Receiver<()>,
    ) -> Self {
        match sync_type {
            ServerSyncType::Mutex => Self::init_as_mutex(*addr, start, shutdown_receiver),
            ServerSyncType::RwLock => Self::init_as_rw(*addr, start, shutdown_receiver),
        }
    }

    pub fn run(
        addr: &SocketAddr,
        sync_type: ServerSyncType,
        shutdown_signal: tokio::sync::oneshot::Receiver<()>,
    ) -> tokio::task::JoinHandle<()> {
        let router = match sync_type {
            ServerSyncType::Mutex => Router::new()
                .route("/", post(Self::input_handler_mutex))
                .layer(AddExtensionLayer::new(MutexManager::default())),
            ServerSyncType::RwLock => Router::new()
                .route("/", post(Self::input_handler_rw))
                .layer(AddExtensionLayer::new(RwLockManager::default())),
        };
        let server = Server::bind(addr)
            .serve(router.into_make_service())
            .with_graceful_shutdown(async move {
                shutdown_signal.await.ok();
            });

        tokio::task::spawn(async move {
            if let Err(e) = server.await {
                eprintln!("server error: {}", e);
            }
        })
    }
}
