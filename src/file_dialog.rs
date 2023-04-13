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

use crate::{
    c_string, nvd_get_file_location, nvd_open_file_dialog_new, nvd_save_file_dialog_new,
    NvdFileDialog,
};
use std::{
    ffi::{c_char, CStr},
    path::PathBuf,
    ptr::{null, null_mut},
};

/// # Mode of the file dialog
/// A file dialog may either be used for getting a file (`OpenFile`) or
/// saving a file (`SaveFile`). When creating a new file dialog, you must set
/// its mode by one of the enums below.
/// # Example
/// ```
/// extern crate nvdialog_rs;
/// use nvdialog_rs::{
///     FileDialog,
///     FileDialogType
/// }
///
/// fn main() {
///     let dialog = FileDialog::new(
///             "Choose a file",
///             &FileDialogType::OpenFile
///     );
///     println!("Filename: {:?}", dialog.retrieve_filename());
/// }
/// ```
pub enum FileDialogType {
    OpenFile,
    SaveFile,
}

/// A struct representing a file dialog window.
///
/// This struct is used to display a file dialog window to the user,
/// allowing them to select a file or folder. It provides an interface to the
/// operating system's file dialog APIs, which are used to display the dialog
/// and retrieve the selected file or folder location.
///
/// # Examples
/// Creating a new `FileDialog` and showing it to the user, returning the path selected:
/// ```
/// use nvdialog_rs as nvdialog;
/// use nvdialog::FileDialog;
/// use nvdialog::FileDialogType;
///
/// let mut file_dialog = FileDialog::new("Title", FileDialogType::OpenFile);
/// let path = PathBuf::new();
/// if let Some(filename) = file_dialog.retrieve_filename() {
///     path = filename;
///     // do something with the returned Path
/// } else {
///     println!("No file selected.");
/// }
/// ```
/// ## FFI
/// Matches with `NvdFileDialog`.
pub struct FileDialog {
    raw: *mut NvdFileDialog,
    location_chosen: Option<String>,
}

impl FileDialog {
    /// Creates a new `FileDialog` instance with the specified title and
    /// type of dialog. This function returns a new `FileDialog` instance
    /// with a raw pointer to the underlying `NvdFileDialog` struct
    /// initialized based on the specified type of dialog. The title
    /// argument specifies the title to be displayed in the file dialog
    /// window. The `type_of_dialog` argument determines whether the dialog
    /// is used for opening a file (`FileDialogType::OpenFile`) or saving
    /// a file (`FileDialogType::SaveFile`).
    ///
    /// If `FileDialogType::OpenFile` is specified, the `raw` pointer is
    /// obtained by calling the `nvd_open_file_dialog_new` function from
    /// the underlying C API. If `FileDialogType::SaveFile` is specified,
    /// the `raw` pointer is obtained by calling the
    /// `nvd_save_file_dialog_new` function from the underlying C API. In
    /// the case of `FileDialogType::SaveFile`, the dialog defaults to
    /// suggesting a filename of "filename".
    ///
    /// # Examples
    ///
    /// Creating a new `FileDialog` instance for opening a file:
    ///
    /// ```
    /// let file_dialog = FileDialog::new("Open File", FileDialogType::OpenFile);
    /// ```
    ///
    /// Creating a new `FileDialog` instance for saving a file:
    ///
    /// ```
    /// let file_dialog = FileDialog::new("Save File", FileDialogType::SaveFile);
    /// ```
    pub fn new<S: AsRef<str>>(title: S, type_of_dialog: FileDialogType, file_extensions: impl IntoIterator<Item = S>) -> Self {
        /* Just converting this into a format NvDialog will understand */
        let mut extensions = String::new();
        for extension in file_extensions {
            extensions += extension.as_ref();
            extensions += ";";
            extensions += "\0";
        }
        match type_of_dialog {
            FileDialogType::OpenFile => Self {
                raw: unsafe { nvd_open_file_dialog_new(c_string!(title.as_ref()), c_string!(extensions)) },
                location_chosen: None,
            },
            FileDialogType::SaveFile => Self {
                raw: unsafe {
                    nvd_save_file_dialog_new(c_string!(title.as_ref()), c_string!("filename"))
                },
                location_chosen: None,
            },
        }
    }

    /// Retrieves the file name selected in the file dialog. This
    /// function returns a `PathBuf` instance containing the selected
    /// file name, or `None` if no file was selected.
    ///
    /// This function calls the `nvd_get_file_location`
    /// function from the underlying C API. If the returned pointer from NvDialog is `NULL`,
    /// this function returns `None`. If the pointer is not null, this function constructs a
    /// `CStr` instance from the raw buffer, and constructs a `PathBuf`
    /// instance from the `CStr`.
    ///
    /// # Safety
    /// This function constructs the returned [`PathBuf`](std::path::PathBuf) using C bytes, which may
    /// cause a panic if the C data contains non-valid UTF-8 characters.
    ///
    /// # Returns
    ///
    /// - `Some(PathBuf)` if a file was selected and the selected file name
    ///   could be converted to a `PathBuf` instance.
    /// - `None` if no file was selected.
    ///
    /// # Panics
    /// This function may panic with the message "Invalid UTF-8 data" if the
    /// raw buffer returned from the underlying C API contains invalid
    /// UTF-8 data.
    ///
    /// # Examples
    /// ```
    /// let mut file_dialog = FileDialog::new("Open File", FileDialogType::OpenFile);
    ///
    /// if let Some(path) = file_dialog.retrieve_filename() {
    ///     // A file was selected. Do something with the file...
    /// } else {
    ///     eprintln!("No file was selected")
    /// }
    /// ```
    pub fn retrieve_filename(&mut self) -> Option<PathBuf> {
        let raw_buffer: *mut c_char = null_mut();
        unsafe {
            nvd_get_file_location(self.raw, &raw_buffer as *const _ as *mut _);
        }
        if raw_buffer.is_null() {
            return None;
        }
        let filename = unsafe { CStr::from_ptr(raw_buffer) };
        Some(PathBuf::from(
            filename.to_str().expect("Invalid UTF-8 data"),
        ))
    }
}
