/*
 *  The MIT License (MIT)
 *
 *  Copyright (c) 2022-2025 Aggelos Tselios
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a copy
 *  of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 *  furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

//! This crate provides high-level, low-overhead bindings to NvDialog for Rust, and serves as the successor to the
//!  [`nvdialog`](https://crates.io/crates/nvdialog) crate. Unlike its predecessor, this crate does not rely on libloading
//! for system bindings to NvDialog, offering a more direct integration by manually building and linking with `libnvdialog` through `nvdialog-sys`.
//!
//! # Safety
//! The crate strives to replicate Rust’s compile-time safety checks within the context of NvDialog to ensure the integrity of your code.
//! For instance, we use mutable references where necessary in FFI calls that modify data, avoiding plain references
//! which aren't checked by Rust's borrow checker.
//! 
//! # Threading Considerations
//! NvDialog’s threading rules remain the same, regardless of the language. Dialogs should always be created and used on the same thread.
//! While it's technically possible to create dialogs from secondary threads, it is not officially supported and can lead
//! to issues on certain platforms. Specifically:
//!
//! - **Windows**: Creating dialogs from secondary threads may work in a lot of cases, but it is considered unsafe and not recommended.
//! - **macOS**: UI operations, including dialogs, must be performed on the main thread. Creating dialogs from other threads is not supported.
//! - **Gtk3/4 (GNU/Linux)**: While Gtk does not directly support cross-thread UI operations, GLib provides mechanisms to safely send data between threads. However `nvdialog-rs` does not make use of them, meaning sending and receiving data across threads is still unsupported.
 
 
//! # Example dialog:
//! ```rust
//! use nvdialog_rs::DialogBox;
//! use nvdialog_rs::DialogType;
//!
//! /* Initialize the library. This corresponds to 'nvd_init' */
//! nvdialog_rs::init();
//!
//! /* Creating the dialog box. */
//! let dialog_box = DialogBox::new(
//!        "Hello from Rust!", /* Title of the dialog */
//!        /* Message of the dialog */
//!        "This dialog has been created using Rust and NvDialog bindings to the language.",
//!        /* See documentation for more */
//!        DialogType::Simple
//! );
//!
//! /* Showing the dialog box. */
//! dialog_box.show();
//! ```
//! Memory management is performed automatically internally by implementing Drop on all types, therefore every dialog is implicitly freeing
//! any memory it used at the end of scope.

#![allow(dead_code, improper_ctypes)]

mod about_dialog;
mod dialog_box;
mod error;
mod file_dialog;
mod image;
mod notification;
mod question_dialog;
mod util;

pub use about_dialog::*;
pub use dialog_box::*;
pub use error::*;
pub use file_dialog::*;
pub use image::*;
pub use notification::*;
use nvdialog_sys::ffi::nvd_init;
pub use question_dialog::*;

/// Initialize NvDialog in the current thread.
///
/// This function initializes NvDialog and its associated backends, and should be called at the
/// top of your program. Note that this function is required to be called in order to show dialogs.
/// Not calling this function before using most of NvDialog's available API is **undefined behavior**.
///
/// # Returns
/// If the initialization is successful (i.e., `nvd_init` returns 0), then this function returns
/// `Ok(())`. Otherwise, an [`Error`] is returned built from the error that NvDialog returned.
///
/// # Examples
/// Basic usage:
///
/// ```
/// fn main() {
///     nvdialog_rs::init().expect("Failed to initialize NvDialog");
///     // the rest of your application...
/// }
/// ```
///
/// Initializing from a second thread:
///
/// ```
/// use std::thread;
/// fn main() {
///     println!("Main thread!");
///     thread::spawn(move ||{
///         nvdialog_rs::init().expect("Init error");
///         // Use `nvdialog_rs` only within this thread now!
///     })
/// }
/// ```
/// The `init` function is intended to be called once at the beginning of your program. Calling it
/// again after it has already been called succesfully is going to return [`Error::AlreadyInitialized`].
///
/// # Multithreading
/// For projects that wish to use multiple threads with NvDialog, you must make **ALL** calls in the second
/// thread. That is, do not call this function on your main thread and other functions in the secondary thread,
/// as that produces undefined behavior on some platforms. The CI on the [**NvDialog Repo**](https://github.com/tseli0s/nvdialog)
/// runs a multithreading test on most desktop platforms with that exact undefined behavior to monitor the runtime
/// behavior.
///
/// # FFI
/// Corresponds to `nvd_init`.
pub fn init() -> Result<(), Error> {
    let result = unsafe { nvd_init() };

    if result == 0 {
        Ok(())
    } else {
        Err(Error::from(result))
    }
}
/// Sets the application name for NvDialog.
///
/// This function sets the application name for NvDialog, often used in notifications
/// and system configuration (eg. DBus). By default, the name is set to `NvDialog Application`
/// since empty strings may cause issues.
/// **NOTICE:** Do not confuse this function with your program's executable name! That used to be
/// handled by [`crate::init`] but has been deprecated entirely!
pub fn set_app_name<S: AsRef<str>>(name: S) {
    let name = cstr!(name.as_ref());
    unsafe {
        nvdialog_sys::ffi::nvd_set_application_name(name.as_ptr());
    }
}
