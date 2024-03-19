// Copyright (C) 2024 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Errors accessing system properties.

use std::str::Utf8Error;
use thiserror::Error;

/// Errors this crate can generate
#[derive(Debug, Error)]
pub enum PropertyWatcherError {
    /// We can't watch for a property whose name contains a NUL character.
    #[error("Cannot convert name to C string")]
    BadNameError(#[from] std::ffi::NulError),
    /// We can only watch for properties that exist when the watcher is created.
    #[error("System property is absent")]
    SystemPropertyAbsent,
    /// System properties are not initialized
    #[error("System properties are not initialized.")]
    Uninitialized,
    /// __system_property_wait timed out.
    #[error("Wait failed")]
    WaitFailed,
    /// read callback was not called
    #[error("__system_property_read_callback did not call callback")]
    ReadCallbackNotCalled,
    /// read callback gave us a NULL pointer
    #[error("__system_property_read_callback gave us a NULL pointer instead of a string")]
    MissingCString,
    /// read callback gave us a bad C string
    #[error("__system_property_read_callback gave us a non-UTF8 C string")]
    BadCString(#[from] Utf8Error),
    /// read callback returned an error
    #[error("Callback failed")]
    CallbackError(#[from] anyhow::Error),
    /// Failure in setting the system property
    #[error("__system_property_set failed.")]
    SetPropertyFailed,
}

/// Result type specific for this crate.
pub type Result<T> = std::result::Result<T, PropertyWatcherError>;

/// Errors returned by generated system property accessors.
#[derive(Debug, Error)]
pub enum SysPropError {
    /// Failed to fetch the system property.
    #[error("Failed to fetch system property: {0}")]
    FetchError(PropertyWatcherError),
    /// Failed to set the system property.
    #[error("Failed to set system property: {0}")]
    SetError(PropertyWatcherError),
    /// Failed to parse the system property value.
    #[error("Failed to parse the system property value: {0}")]
    ParseError(String),
}
