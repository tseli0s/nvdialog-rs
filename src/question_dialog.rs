/*
 *  The MIT License (MIT)
 *
 *  Copyright (c) 2022-2023 Aggelos Tselios
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

use crate::{c_string, nvd_dialog_question_new, nvd_get_reply, NvdQuestionBox};
use std::ffi::{c_int, c_void};

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
    YesNoCancel,
}

/// A dialog box for asking a question and getting a response from
/// the user.
///
/// This struct wraps the underlying C API for creating a question dialog
/// box. It provides a high-level interface for displaying a dialog box,
/// getting a response from the user, and destroying the dialog box.
///
/// # Examples
///
/// ```
/// let question_dialog = QuestionDialog::new(
///     "Are you sure you want to delete this file?",
///     "This action cannot be undone.",
///     QuestionDialogButtons::YesNo,
/// );
/// let reply = question_dialog.get_reply();
///
/// match result {
///     Reply::Accepted => {
///         // The user clicked "Yes". Delete the file...
///     }
///     Reply::Rejected => {
///         // The user clicked "No". Do not delete the file...
///     }
///     Reply::Cancelled => {
///         // The user clicked "Cancel". Do not delete the file...
///     }
/// }
/// ```
/// ## Safety
/// This function converts the C enum for the reply (See
/// [`NvdReply`](https://github.com/tseli0s/nvdialog/blob/master/include))
/// into the crate's [`Reply`](crate::question_dialog::Reply) type using the
/// `From<i32>` trait. This is generally safe, however, keep in mind that invalid
/// or garbage values returned by some other, unrelated unsafe code may actually bypass
/// the Rust safety rules.
/// For further information see the documentation of [`Reply`](crate::question_dialog::Reply).
/// ## FFI
/// Corresponds to `NvdQuestionBox`.
pub struct QuestionDialog {
    raw: *mut NvdQuestionBox,
    title: String,
    msg: String,
    buttons: QuestionDialogButtons,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
/// An enum that holds all possible replies from a dialog.
/// Can be converted from an `i32` if needed.
/// # Example
/// ```rust
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
    Rejected,
}

impl QuestionDialog {
    /// Creates a new `QuestionDialog` with the specified title, message, and
    /// buttons.
    ///
    /// # Arguments
    /// * `title` - A string slice or reference that contains the title of the dialog box.
    /// * `msg` - A string slice or reference that contains the message to display in the dialog box.
    /// * `buttons` - A `QuestionDialogButtons` enum that specifies the buttons to display in the dialog box.
    ///
    /// # Examples
    ///
    /// ```
    /// let question_dialog = QuestionDialog::new(
    ///     "Are you sure you want to delete this file?",
    ///     "This action cannot be undone.",
    ///     QuestionDialogButtons::YesNo,
    /// );
    /// ```

    pub fn new<S: AsRef<str>>(title: S, msg: S, buttons: QuestionDialogButtons) -> Self {
        Self {
            raw: unsafe {
                nvd_dialog_question_new(
                    c_string!(title.as_ref()),
                    c_string!(msg.as_ref()),
                    buttons as c_int,
                )
            },
            title: String::from(title.as_ref()),
            msg: String::from(msg.as_ref()),
            buttons,
        }
    }

    /// Returns the user's reply to the question displayed in the dialog box.
    ///
    /// # Examples
    /// ```
    /// let question_dialog = QuestionDialog::new(
    ///     "Are you sure you want to delete this file?",
    ///     "This action cannot be undone.",
    ///     QuestionDialogButtons::YesNo,
    /// );
    /// let reply = question_dialog.get_reply();
    /// if reply == Reply::Yes {
    ///     // Delete the file.
    /// } else {
    ///     // Do nothing.
    /// }
    /// ```
    pub fn get_reply(&mut self) -> Reply {
        Reply::from(unsafe { nvd_get_reply(self.raw) })
    }
}

impl From<i32> for Reply {
    fn from(value: i32) -> Self {
        if value == Reply::Accepted as i32 {
            Reply::Accepted
        } else if value == Reply::Cancelled as i32 {
            Reply::Cancelled
        } else if value == Reply::Rejected as i32 {
            Reply::Rejected
        } else {
            Reply::Cancelled
        }
    }
}

impl Drop for QuestionDialog {
    fn drop(&mut self) {
        unsafe {
            crate::nvd_free_object(self.raw as *mut c_void);
        }
    }
}
