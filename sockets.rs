// Copyright (C) 2022 The Android Open Source Project
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

//! Provides utilities for sockets.

use std::ffi::CString;
use std::os::fd::OwnedFd;
use thiserror::Error;

use crate::inherited_fd;

/// Errors this crate can generate
#[derive(Error, Debug)]
pub enum SocketError {
    /// Invalid socket name. It could be either due to a null byte in the name, or the name refers
    /// to a non-existing socket.
    #[error("socket name {0} is invalid")]
    InvalidName(String),

    /// Error when taking ownership of the socket file descriptor.
    #[error("Failed to take file descriptor ownership: {0}")]
    OwnershipFailed(inherited_fd::Error),
}

/// Get `OwnedFd` for a Unix domain socket that init created under the name `name`. See
/// [Android Init Language]
/// (https://cs.android.com/android/platform/superproject/main/+/main:system/core/init/README.md)
/// for creating sockets and giving them names.
///
/// The returned file descriptor has the flag CLOEXEC set.
///
/// This function returns `SocketError::OwnershipFailed` if `crate::inherited_fd::init_once` was
/// not called very early in the process startup or this function is called multile times with the
/// same `name`.
pub fn android_get_control_socket(name: &str) -> Result<OwnedFd, SocketError> {
    let cstr = CString::new(name).map_err(|_| SocketError::InvalidName(name.to_owned()))?;
    // SAFETY: android_get_control_socket doesn't take ownership of name
    let fd = unsafe { cutils_bindgen::android_get_control_socket(cstr.as_ptr()) };
    if fd < 0 {
        return Err(SocketError::InvalidName(name.to_owned()));
    }
    inherited_fd::take_fd_ownership(fd).map_err(SocketError::OwnershipFailed)
}
