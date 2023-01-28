use std::{ptr::{null, null_mut}, ffi::c_char};

use crate::{NvdFileDialog, nvd_open_file_dialog_new, add_null_byte, nvd_get_file_location, nvd_save_file_dialog_new};

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
        SaveFile
}

/// # A file dialog.
/// This struct is simply a handle to a file dialog for NvDialog.
/// For more information on file dialogs, see NvDialog's documentation.
/// # WARNING
/// File dialogs are still experimental and will most likely fail to run properly.
pub struct FileDialog {
        raw: *mut NvdFileDialog,
        location_chosen: Option<String>
}

impl FileDialog {
        /// Creates a new file dialog and returns it.
        /// # Safety
        /// This dialog will most likely cause a segfault.
        /// Please use with caution.
        /// # Examples
        /// ```rust
        /// use nvdialog_rs::FileDialog;
        /// 
        /// let dialog = FileDialog::new(String::from("Open File"), FileDialogType::OpenFile);
        /// ```
        pub fn new(title: String, type_of_dialog: &FileDialogType) -> Self {
                match type_of_dialog {
                        FileDialogType::OpenFile => Self {
                                raw: unsafe {
                                        nvd_open_file_dialog_new(
                                                add_null_byte(title).as_ptr() as *const c_char,
                                                null()
                                        )
                                },
                                location_chosen: None

                        },
                        FileDialogType::SaveFile => Self {
                                raw: unsafe { nvd_save_file_dialog_new(
                                        add_null_byte(title).as_ptr() as *const c_char,
                                        "Empty\0".as_ptr() as *const c_char,
                                )},
                                location_chosen: None
                        }
                }
        }

        /// Returns the filename chosen from the file dialog.
        pub fn retrieve_filename(&self) -> String {
                let raw_buffer: *mut c_char = null_mut();
                unsafe { nvd_get_file_location(self.raw, &raw_buffer as *const _ as *mut _); }
                if raw_buffer.is_null() {
                        return String::new();
                }
                let filename = unsafe {
                        String::from_raw_parts(
                                raw_buffer as *mut u8,
                                libc::strlen(raw_buffer),
                                libc::strlen(raw_buffer)
                        )
                };
                filename
        }
}