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

use std::ffi::{c_void, c_char};

use crate::{NvdDialogBox, nvd_dialog_box_new, add_null_byte};

/// # Specifies the type of the dialog box to be created
/// This enum holds possible values for the type of the dialog
/// that will be created:
/// - `Simple` means the dialog box will not get any special decoration.
/// - `Warning` means the dialog box will get more serious colors and a warning icon sometimes.
/// - `Error` means the dialog box is intended to be used for an error, which means bright red
/// colors and an error icon will be shown.
/// # FFI Matches
/// The members of this enum correspond to:\
/// `Simple`  -> `NVD_DIALOG_SIMPLE` (0x04) \
/// `Warning` -> `NVD_DIALOG_WARNING` \
/// `Error`   -> `NVD_DIALOG_ERROR`
pub enum DialogType {
        /// Simple dialog
        Simple,
        /// Warning dialog
        Warning,
        /// Error dialog
        Error
}

/// # A Dialog Box
/// This struct represents a single dialog box created by this crate.
/// In order to create a new dialog box, see [`DialogBox::new`](crate::dialog_box::DialogBox::new).
pub struct DialogBox {
        raw: *mut NvdDialogBox,
        title: String,
        msg: String
}

impl DialogBox {
        /// Creates a new dialog box with the parameters `title` and `msg` given and
        /// returns it.
        pub fn new<S: AsRef<str>>(title: S, msg: S, dialog_type: DialogType) -> Self {
                let _type = match dialog_type {
                        DialogType::Simple => 0xff,
                        DialogType::Warning => 0xff + 1,
                        DialogType::Error => 0xff + 2,
                };

                let title = add_null_byte(title.as_ref());
                let msg = add_null_byte(msg.as_ref());

                Self {
                        raw: unsafe { nvd_dialog_box_new(
                                title.as_ptr() as *mut c_char,
                                msg.as_ptr() as *mut c_char,
                                _type
                        ) },
                        title: String::from(title),
                        msg: String::from(msg)
                }
        }

        /// Renders the dialog box to the screen.
        /// This function simply calls `nvd_show_dialog`.
        pub fn show(&self) {
                unsafe { crate::nvd_show_dialog(self.raw); }
        }

        /// Returns the raw pointer to the dialog box created
        /// from NvDialog directly.
        fn get_raw(&self) -> *mut NvdDialogBox {
                self.raw
        }
}

impl Drop for DialogBox {
        fn drop(&mut self) {
            unsafe { crate::nvd_free_object(self.raw as *mut c_void); }
        }
}