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

use std::ffi::{c_void, c_char, c_int};
use crate::{add_null_byte, NvdQuestionBox, nvd_dialog_question_new, nvd_get_reply};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// # Possible button combination for question dialogs.
/// This enum contains the button combination for a question
/// dialog. The enum members match the original constants by NvDialog
/// for better FFI support (see [Members](#members))
/// # Example
/// ```
/// extern crate nvdialog_rs;
/// use nvdialog_rs::{
///     QuestionDialog,
///     QuestionDialogButtons
/// }
/// 
/// fn main() {
///     let dialog = QuestionDialog::new(
///             "title",
///             "message",
///             &QuestionDialogButtons::YesNoCancel
///     );
///     println!("Reply from dialog: {:?}"), dialog.get_reply());
/// }
/// ```
/// # Members
/// - `Yes`: Corresponds to `NVD_YES`.
/// - `YesNo`: Corresponds to `NVD_YES_NO`.
/// - `YesNoCancel`: Corresponds to `NVD_YES_NO_CANCEL`.
pub enum QuestionDialogButtons {
        Yes = 0x04,
        YesNo,
        YesNoCancel
}

/// # Question dialog type.
/// A struct that holds information about a question dialog.
/// To create one, use the [`QuestionDialog::new()`](crate::question_dialog::QuestionDialog::new) function.
/// A question dialog may be used when you want to confirm something the user has done (Potentially accidentally)
/// and provides a series of buttons to reply to it.
pub struct QuestionDialog {
        raw: *mut NvdQuestionBox,
        title: String,
        msg: String,
        buttons: QuestionDialogButtons
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// # Question reply enum
/// An enum that holds all possible replies from a dialog.
/// Can be converted from an `i32` if needed.
/// # Example
/// ```
/// extern crate nvdialog_rs;
/// use nvdialog_rs::{
///     QuestionDialog,
///     QuestionDialogButtons
/// }
/// 
/// fn main() {
///     let dialog = QuestionDialog::new(
///             "title",
///             "message",
///             &QuestionDialogButtons::YesNoCancel
///     );
///     println!("Reply from dialog: {:?}"), dialog.get_reply());
/// }
/// ```
/// # Members
/// - `Accepted` -> Corresponds to `NVD_REPLY_OK`, returned if user pressed the Okay button.
/// - `Cancelled` -> Corresponds to `NVD_REPLY_CANCEL`, returned if user pressed the Cancel button.
/// - `Rejected` -> Corresponds to `NVD_REPLY_NO`, returned if user pressed the No button.
/// # Errors
/// In order to work with raw C integers, a conversion is done (See the `From` trait for details). If the
/// integer given is not valid though, then the return value will always be `NVD_REPLY_CANCEL` to comply
/// with Rust's safety rules.
pub enum Reply {
        Accepted = 0x04,
        Cancelled,
        Rejected
}

impl QuestionDialog {
        /// Creates a new `QuestionDialog` and returns it with the parameters given.
        pub fn new(title: String, msg: String, buttons: QuestionDialogButtons) -> Self {
                Self {
                        raw: unsafe {
                                nvd_dialog_question_new(
                                        add_null_byte(&title).as_ptr() as *mut c_char,
                                        add_null_byte(&msg).as_ptr() as *mut c_char,
                                        buttons as c_int
                                )
                        },
                        title,
                        msg,
                        buttons
                }
        }

        /// Returns the user reply from the dialog given.
        pub fn get_reply(&self) -> Reply {
                Reply::from(unsafe {
                        nvd_get_reply(self.raw)
                })
        }
}

impl From<i32> for Reply {
    fn from(value: i32) -> Self {
        if value == Reply::Accepted as i32 { Reply::Accepted }
        else if value == Reply::Cancelled as i32 { Reply::Cancelled }
        else if value == Reply::Rejected as i32 { Reply::Rejected }
        else { Reply::Cancelled }
    }
}

impl Drop for QuestionDialog {
        fn drop(&mut self) {
            unsafe { crate::nvd_free_object(self.raw as *mut c_void); }
        }
}