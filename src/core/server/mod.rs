// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod axum_server;
mod hyper_server;
mod warp_server;

pub trait ServerInstance {
    type Error;
    fn start(&self, addr: &std::net::SocketAddr) -> Result<(), Self::Error>;
    fn stop(&self) -> Result<(), Self::Error>;
}
