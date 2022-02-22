// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{convert::Infallible, str::FromStr};

use tracing::{debug, error, info, trace, warn};

use crate::{async_trait, BadRequestError, ComputeFunction, ComputeRequest, ComputeResponse};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Unknown,
}

impl FromStr for LogLevel {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "info" | "i" => Ok(Self::Info),
            "warn" | "w" | "warning" => Ok(Self::Warn),
            "error" | "e" => Ok(Self::Error),
            "debug" | "d" => Ok(Self::Debug),
            "trace" | "t" => Ok(Self::Trace),
            _ => Ok(Self::Unknown),
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Unknown
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => write!(f, "Trace"),
            Self::Debug => write!(f, "Debug"),
            Self::Info => write!(f, " Info"),
            Self::Warn => write!(f, " Warn"),
            Self::Error => write!(f, "Error"),
            Self::Unknown => write!(f, " ??? "),
        }
    }
}

#[derive(Debug, Default)]
pub struct Logger;

#[async_trait]
impl ComputeFunction for Logger {
    fn name(&self) -> &'static str {
        "logger"
    }

    #[allow(clippy::unused_async)]
    async fn receive_request(
        &self,
        request: &ComputeRequest,
    ) -> Result<ComputeResponse, BadRequestError> {
        let data = request.data();
        let is_str = data.is_string();
        let is_obj = data.is_object();

        if !is_str && !is_obj {
            return Err(BadRequestError::new(
                self.name(),
                "Data must be an object or string",
                Some(request.clone()),
            ));
        }

        if is_str {
            match data.as_str() {
                Some(s) => send_log(LogLevel::Info, s),
                None => {
                    return Err(BadRequestError::new(
                        self.name(),
                        "Unable to convert data (which returned true for is_string) to a string",
                        Some(request.clone()),
                    ))
                }
            }
        } else {
            let obj =
                match data.as_object() {
                    Some(o) => o,
                    None => return Err(BadRequestError::new(
                        self.name(),
                        "Unable to convert data (which returned true for is_object) to a object",
                        Some(request.clone()),
                    )),
                };

            let level = multi_string_keys(obj, &["level", "lvl", "l"], LogLevel::default(), |s| {
                s.parse::<LogLevel>().unwrap_or_default()
            });

            let msg = multi_string_keys(
                obj,
                &["message", "msg", "m", "text", "log"],
                "".to_string(),
                std::string::ToString::to_string,
            );

            let sender = multi_string_keys(
                obj,
                &["sender", "s", "app", "self", "this"],
                "".to_string(),
                std::string::ToString::to_string,
            );

            let ts = get_timestamp();

            let log = obj.get("data").map_or_else(
                || format!("{}:[{}]{}| {}", ts, level, sender, msg),
                |d| format!("{}:[{}]{}| {} | {}", ts, level, sender, msg, d),
            );

            send_log(level, &log);
        }

        Ok(ComputeResponse::ok())
    }
}

#[allow(
    clippy::cognitive_complexity,
    reason = "Simple function, but macro expansion apparently makes it fucking huge."
)]
fn send_log(lvl: LogLevel, log: &str) {
    match lvl {
        LogLevel::Trace => trace!("{}", log),
        LogLevel::Debug => debug!("{}", log),
        LogLevel::Info => info!("{}", log),
        LogLevel::Warn => warn!("{}", log),
        LogLevel::Error => error!("{}", log),
        LogLevel::Unknown => debug!("{}", log),
    }
}

/// Simple function to get a timestamp string.
fn get_timestamp() -> String {
    use chrono::{SecondsFormat, Utc};

    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

/// Helper function try multiple variations of a key to find one that might exist.
/// I only had need for retrieving strings so I didn't make it generic over the
/// type of value. If I used it more often it could probably be made so.
///
/// TODO: Maybe I could provide it in the planned exported utility library for
/// function implementations.
fn multi_string_keys<Output, F: Fn(&str) -> Output>(
    map: &serde_json::Map<String, serde_json::Value>,
    keys: &[&str],
    def: Output,
    converter: F,
) -> Output {
    for &key in keys {
        if let Some(v) = map.get(key) {
            if let Some(s) = v.as_str() {
                return converter(s);
            }
        }
    }
    def
}
