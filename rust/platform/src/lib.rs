/*
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT license.
 */
#![cfg_attr(
    not(test),
    warn(clippy::panic, clippy::unwrap_used, clippy::expect_used)
)]
pub mod perf;
pub use perf::{get_process_cycle_time, get_process_handle};

#[cfg(target_os = "windows")]
pub mod file_io;
#[cfg(target_os = "windows")]
pub use file_io::{get_queued_completion_status, read_file_to_slice};

#[cfg(target_os = "windows")]
pub mod file_handle;
#[cfg(target_os = "windows")]
pub use file_handle::FileHandle;

#[cfg(target_os = "windows")]
pub mod io_completion_port;
#[cfg(target_os = "windows")]
pub use io_completion_port::IOCompletionPort;
