/*
 *  The MIT License (MIT)
 *
 *  Copyright (c) 2022 Aggelos Tselios
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

//! This crate offers high level, low overhead bindings to NvDialog for Rust. It's the successor of
//! the [`nvdialog`](https://crates.io/crates/nvdialog) crate which provided system bindings to NvDialog
//! using `libloading`.
//! 
//! # Safety
//! The crate tries to imitate Rust's compile time checks with NvDialog, to ensure that safety is
//! present within your code. This includes adding mutable references when an FFI call changes something
//! instead of plain references (Which aren't checked anyways).
//! 
//! When it comes to threads, NvDialog's rules don't change here either: The dialogs must be created and used
//! within the same thread. Creating dialogs from secondary threads is not supported officially.
//! 
//! 
//! # Example dialog:
//! ```rust
//! /* Importing types */
//! extern crate nvdialog_rs;
//! use nvdialog_rs::DialogBox;
//! use nvdialog_rs::DialogType;
//! 
//! /* Initialize the library. This corresponds to 'nvd_init' */
//! nvdialog_rs::init(String::new());
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

#![allow(dead_code, improper_ctypes)]

mod dialog_box;
mod question_dialog;
mod file_dialog;
<<<<<<< HEAD
=======
mod notification;

use libc::fork;

pub use notification::*;
>>>>>>> d2916b4 (Fix `git` errors.)
pub use dialog_box::*;
pub use question_dialog::*;
pub use file_dialog::*;

use std::ffi::{c_char, c_void, c_int};
#[repr(C)]
pub(crate) struct NvdDialogBox;
#[repr(C)]
pub(crate) struct NvdQuestionBox;

#[repr(C)]
pub(crate) struct NvdFileDialog;

<<<<<<< HEAD
=======
#[repr(C)]
pub(crate) struct NvdNotification;

#[repr(C)]
#[allow(non_camel_case_types)]
pub(crate) enum NvdNotifyType {
    NVD_NOTIFICATION_SIMPLE = 0,
    NVD_NOTIFICATION_WARNING = 1,
    NVD_NOTIFICATION_ERROR = 2
}

>>>>>>> d2916b4 (Fix `git` errors.)
#[link(name = "nvdialog")]
extern "C" {
        pub(crate) fn nvd_init(argv0: *const c_char) -> i32;
        
        /* TODO: Convert _type to a proper C enum */
        pub(crate) fn nvd_dialog_box_new(title: *const c_char, msg: *const c_char, _type: c_int) -> *mut NvdDialogBox;
        pub(crate) fn nvd_show_dialog(dialog: *mut NvdDialogBox);
        pub(crate) fn nvd_free_object(object: *mut c_void);

        pub(crate) fn nvd_dialog_question_new(title: *const c_char, msg: *const c_char, button: c_int) -> *mut NvdQuestionBox;
        pub(crate) fn nvd_get_reply(__box: *mut NvdQuestionBox) -> c_int;

        pub(crate) fn nvd_open_file_dialog_new(title: *const c_char, file_extensions: *const c_char) -> *mut NvdFileDialog;
        pub(crate) fn nvd_get_file_location(file_dialog: *mut NvdFileDialog, savebuf: *const *const c_char);
        pub(crate) fn nvd_save_file_dialog_new(title: *const c_char, default_filename: *const c_char) -> *mut NvdFileDialog;
<<<<<<< HEAD
=======

        pub(crate) fn nvd_notification_new(title: *const c_char, msg: *const c_char, kind: NvdNotifyType) -> *mut NvdNotification;
        pub(crate) fn nvd_send_notification(notification: *mut NvdNotification);
        pub(crate) fn nvd_delete_notification(notification: *mut NvdNotification);
        pub(crate) fn nvd_add_notification_action(notification: *mut NvdNotification, action: *const c_char, value_to_set: *mut c_int, value_to_return: *mut *mut c_int);
>>>>>>> d2916b4 (Fix `git` errors.)
}

/// # Function to initialize NvDialog.
/// This function initializes NvDialog and its associated backends.
/// You should call this function somewhere in the top of your program,
/// as this function is expensive and is required to be called before any other
/// function of this crate.
/// # Returns
/// On success (0 returned from `nvd_init`), `Ok(())` is returned. Otherwise, an `Err` with the error
/// code returned from NvDialog is returned.
/// # Examples:
/// ```rust
/// fn main() {
///     nvdialog_rs::init(String::new())
///         .expect("failed to initialize NvDialog");
/// }
/// ```
pub fn init(argv0: String) -> Result<(), i32> {
<<<<<<< HEAD
    let argv0 = add_null_byte(argv0);
    unsafe { match nvd_init(argv0.as_ptr() as *const c_char) {
        0 => Ok(()),
        e => Err(e)
    }}
=======
    let mut pid = -1;
    let mut result: Result<(), i32> = Ok(());
    if cfg!(feature = "secproc") {
        pid = unsafe { fork() };
        if pid < 0 {
            panic!("fork() failed!")
        } else if pid == 0 {
            let argv0 = add_null_byte(argv0);
            unsafe { match nvd_init(argv0.as_ptr() as *const c_char) {
                0 => result = Ok(()),
                e => result = Err(e)
            }};
        }
    } else {
        let argv0 = add_null_byte(argv0);
        unsafe {
            match nvd_init(argv0.as_ptr() as *const c_char) {
                0 => result = Ok(()),
                e => result = Err(e)
            }
        };
    }
    result
>>>>>>> d2916b4 (Fix `git` errors.)
}

pub(crate) fn add_null_byte<S>(s: S) -> String
where
    S: AsRef<str> + std::fmt::Display
{
    format!("{}\0", s)
}
