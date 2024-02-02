/*
 *  The MIT License (MIT)
 *
 *  Copyright (c) 2022-2024 Aggelos Tselios
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

use std::ffi::{c_void, CString};

use nvdialog_sys::ffi::*;

use crate::Error;

/// An enumeration of the different types of dialogs that can be created.
///
/// This enum is used to specify the type of dialog to be created when calling
/// the appropriate dialog creation functions. The different variants represent
/// different types of dialogs that can be used to communicate with the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    /// A simple dialog box with no specific type.
    Simple,
    /// A warning dialog box, indicating that the user should be cautious or take
    /// extra care when performing the action.
    Warning,
    /// An error dialog box, indicating that an error has occurred and the user should
    /// take some action to resolve it.
    Error,
}

/// A struct representing a dialog box.
///
/// This struct provides a simple interface for creating and showing different types of dialog
/// boxes, such as message boxes or question boxes. Once a dialog box is created using the
/// `DialogBox::new` method, it can be shown to the user using the `DialogBox::show` method.
///
/// # Examples
///
/// Showing a simple dialog box:
///
/// ```
/// use my_{DialogBox, DialogType};
///
/// let mut dialog_box = DialogBox::new("My App", "Hello World", DialogType::Simple);
/// dialog_box.show();
/// ```
///
/// # Safety
/// Showing the same dialog box more than once is **undefined behavior** and depending on
/// the platform, may or may not raise an error.
/// # FFI
/// Corresponds to `NvdDialogBox`.
pub struct DialogBox {
    raw: *mut NvdDialogBox,
}

impl DialogBox {
    /// Creates a new instance of `DialogBox` with the given `title`, `msg` and `dialog_type`.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the dialog box.
    ///
    /// * `msg` - The message to display in the dialog box.
    ///
    /// * `dialog_type` - The type of dialog box to display, either `Simple`, `Warning` or `Error`.
    ///
    /// # Returns
    ///
    /// Returns `Ok(DialogBox)` if the dialog box was successfully created, otherwise
    /// returns `Err(Error)` with the error converted from NvDialog's error code.
    ///
    /// # Panics
    /// This function will panic if `CString::new` fails to convert the given `title` or `msg`
    /// to a null-terminated byte string.
    pub fn new<S: AsRef<str>>(title: S, msg: S, dialog_type: DialogType) -> Result<Self, Error> {
        let _type = match dialog_type {
            DialogType::Simple => 0xff,
            DialogType::Warning => 0xff + 1,
            DialogType::Error => 0xff + 2,
        };

        let title = CString::new(title.as_ref()).expect("CString::new error");
        let msg = CString::new(msg.as_ref()).expect("CString::new error");

        let raw = unsafe {
            let raw = nvd_dialog_box_new(title.as_ptr(), msg.as_ptr(), _type);
            if raw.is_null() {
                return Err(Error::from(nvd_get_error() as i32));
            }
            raw
        };

        Ok(Self { raw })
    }

    pub fn set_accept_label<S: AsRef<str>>(&mut self, label: S) {
        let label = CString::new(label.as_ref()).expect("CString::new error");
        unsafe {
            nvd_dialog_box_set_accept_text(self.raw, label.as_ptr());
        }
    }

    /// Displays the dialog box on the screen.
    ///
    /// This function shows the dialog box on the screen, allowing the user to interact with it.
    /// It should be called after setting any necessary options and buttons on the dialog.
    /// This function is unsafe, because it uses FFI to call C code that might not be safe.
    pub fn show(&mut self) {
        unsafe {
            nvd_show_dialog(self.raw);
        }
    }

    /// Returns the raw pointer to the dialog box created
    /// from NvDialog directly.
    ///
    /// Marked as `unsafe` because accessing the internal struct is not officially
    /// supported by NvDialog and may cause race conditions.
    unsafe fn get_raw(&mut self) -> *mut NvdDialogBox {
        self.raw
    }
}

impl Drop for DialogBox {
    fn drop(&mut self) {
        unsafe {
            nvd_free_object(self.raw as *mut c_void);
        }
    }
}
