// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{collections::HashMap, ffi::OsStr};

use libloading::{Library, Symbol};
use tokio::sync::Mutex;

use crate::core::types::{
    AppError, AppResult, ComputeFunction, ComputeRequest, ComputeResponse, LoadingError,
    TargetComputeFunc, UnloadingError,
};

#[derive(Debug, Default)]
pub struct ComputeFunctionManager {
    functions: Mutex<HashMap<String, Box<dyn ComputeFunction>>>,
    loaded_libraries: Mutex<Vec<Library>>,
}

impl ComputeFunctionManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            functions: Mutex::default(),
            loaded_libraries: Mutex::default(),
        }
    }

    /// Loads a [`ComputeFunction`] plugin from a `cdylib` dll at the given path.
    ///
    /// ## Arguments
    /// - `arg_name` - Argument description
    /// ## Returns
    /// This function returns something probably
    ///
    /// ## Errors
    /// Function potentially returns the following errors in the described situations:
    /// - [`LoadingError::BadPath`] if the given path is malformed **OR NOT ABSOLUTE**
    /// - [`LoadingError::PathNotFound`] if the given path does not exist
    /// - [`LoadingError::LibraryLoadFailure`] if a [`libloading::Library`] cannot be loaded from the given path
    /// - [`LoadingError::ConstructorLoadFailure`] if the [`libloading::Symbol`] `_plugin_create` cannot be found in the loaded library
    /// - [`LoadingError::ConstructorCallFailure`] if the `_plugin_create` function returns a null pointer
    ///
    /// ## Safety
    /// The unsafe nature of this function stems from 4 calls and, due to the nature of dynamically loading
    /// [`ComputeFunction`] plugins at runtime, seems unavoidable.
    /// - [`libloading::Library::new`] - This call loads the [`libloading::Library`] and returns a result. If it results in an `Err`, this method will return immediately with a failure.
    /// - [`libloading::Library::get`] - This call attempts to find the symbol `_plugin_create` in the loaded library and returns a result. If it results in an `Err`, this method will return immediately with a failure.
    /// - The constructor found by [`libloading::Library::get`] - This call is a pointer to a function that returns a pointer to a [`ComputeFunction`] constructor function. This function is called to create a raw pointer to a new [`ComputeFunction`] object. The raw pointer is checked for null, and then immediately placed into a [`Box`].
    /// - [`Box::from_raw`] - This is called to make the resulting dynamic [`ComputeFunction`] safe.
    ///
    /// ## Example(s)
    /// ```ignore
    /// /// TODO Write examples
    /// ```
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(
        &mut self,
        library_path: P,
    ) -> Result<(), LoadingError> {
        type CfCtor = unsafe fn() -> *mut dyn ComputeFunction;

        // Validate Path
        let path = std::path::Path::new(&library_path);
        if !path.is_absolute() {
            return Err(LoadingError::bad_path(&format!(
                "Path `{}` is not absolute.",
                library_path.as_ref().to_string_lossy()
            )));
        }
        match std::fs::try_exists(path) {
            Ok(true) => (),
            Ok(false) => {
                return Err(LoadingError::path_not_found(&format!(
                    "Path `{}` does not exist.",
                    library_path.as_ref().to_string_lossy()
                )))
            }
            Err(e) => {
                return Err(LoadingError::bad_path(&format!(
                    "Could not verify the existence of `{}`, either due to errors or lack of permissions. Os error: {}",
                    library_path.as_ref().to_string_lossy(),
                    e
                )))
            }
        };

        // Unsafely load the plugin from the library
        let plugin = unsafe {
            // Attempt to load library from given path
            let lib = Library::new(path).map_err(|err| LoadingError::lib_load_failure(&err))?;

            // This "dance" is required to create a long-lived pointer to the library,
            // if the library goes out of scope our plugin becomes invalid. I am not worried
            // about the `expect` call here since something would need to be very wrong
            // for it to fail.
            self.loaded_libraries.get_mut().push(lib);
            let lib = self.loaded_libraries
            .get_mut()
            .last()
            .expect("This error should not happen, we are trying to get the last element of an array we just pushed to, so something is very wrong.");

            // Get the expected constructor function from the library
            let constructor: Symbol<CfCtor> = lib
                .get::<CfCtor>(b"_plugin_create")
                .map_err(|err| LoadingError::ctor_load_failure(&err))?;

            // Unsafely call the constructor function to create a new plugin
            let boxed_raw = constructor();
            // Ensure resulting object is not null
            if boxed_raw.is_null() {
                return Err(LoadingError::ctor_call_failure());
            }
            // Box the raw pointer for safe use
            Box::from_raw(boxed_raw)
        };

        let plugin_name = plugin.name();
        if self.functions.get_mut().contains_key(plugin_name) {
            // Name collisions are not allowed, first come first serve
            return Err(LoadingError::name_collision(&plugin_name));
        }
        // Allow plugin to initialize itself if necessary
        plugin.on_plugin_load();
        self.functions
            .get_mut()
            .insert(plugin_name.to_string(), plugin);

        Ok(())
    }

    /// Unloads a [`ComputeFunction`] plugin from the manager.
    ///
    /// ## Arguments
    /// - `target` - The target [`ComputeFunction`] to unload
    ///
    /// ## Returns
    /// A result containing nothing upon success or an [`UnloadingError`] upon failure.
    ///
    /// ## Errors
    /// - Function errors if the target [`ComputeFunction`] is not found in the manager
    ///
    /// ## Example(s)
    /// ```ignore
    /// /// TODO Write examples
    /// ```
    pub fn unload_plugin(&mut self, target: &TargetComputeFunc) -> Result<(), UnloadingError> {
        self.functions.get_mut().remove(target.name()).map_or_else(
            || Err(UnloadingError::TargetNotFound(target.clone())),
            |plugin| {
                plugin.on_plugin_unload();
                Ok(())
            },
        )
    }

    /// Unloads all functions **and libraries** that this [`ComputeFunctionManager`] is holding references for.
    /// TODO: Should this method resize the containers to 0? There should only ever be once of these instances
    ///       that lasts for the entire program so it seems unnecessary, but `drain` specifically states that
    ///       the previously allocated memory is held.
    /// ## Example(s)
    /// ```ignore
    /// /// TODO Write examples
    /// ```
    pub fn unload_all(&mut self) {
        for (_id, plugin) in self.functions.get_mut().drain() {
            // trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.get_mut().drain(..) {
            drop(lib);
        }
    }

    /// TODO: It's just dawning on me that simply comparing the [`ComputeRequest::target`] to the map key
    ///       is some real basic-bitch shit. I need to parse the target to allow for namespaces and sub-paths,
    ///       and even path parameters & queries.
    /// Sends a [`ComputeRequest`] to the [`ComputeFunction`] indicated by the request.
    ///
    /// ## Arguments
    /// - `request` - The [`ComputeRequest`] to send to the currently loaded [`ComputeFunction`] plugins.
    ///
    /// ## Returns
    /// [`AppResult`] containing the [`ComputeResponse`] from the [`ComputeFunction`] or an [`AppError`]
    /// indicating the type of failure that occurred.
    ///
    /// ## Errors
    /// - [`AppError::TargetNotFound`] if the target [`ComputeFunction`] is not found in the manager
    /// - [`AppError::BadRequest`] if the [`ComputeRequest`] is malformed or invalid
    ///
    /// ## Example(s)
    /// ```ignore
    /// /// TODO Write examples
    /// ```
    pub async fn push_request(&self, request: &ComputeRequest) -> AppResult<ComputeResponse> {
        let id = request.target().name();

        let plugins = self.functions.lock().await;
        if let Some(plugin) = plugins.get(id) {
            plugin
                .receive_request(request)
                .await
                .map_err(std::convert::Into::into)
        } else {
            Err(AppError::TargetNotFound(request.target().clone()))
        }
    }
}

impl Drop for ComputeFunctionManager {
    fn drop(&mut self) {
        let has_plugins = !self.functions.get_mut().is_empty();
        let has_libs = !self.loaded_libraries.get_mut().is_empty();
        if has_plugins || has_libs {
            self.unload_all();
        }
    }
}
