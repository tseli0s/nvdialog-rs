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

use thiserror::Error;

/// `Error` is the main type for handling errors in the crate. It's direct C equivalent is
/// `NvdError` (Located at `include/nvdialog_types.h`, line 53). Errors are converted from the
/// C side into this crate using the `From<i32>` implementation.
///
/// When a function returns this type as an error, you should probably handle it, as NvDialog's design
/// makes one error persist until fixed.
/// ## Example
/// ```rust
/// use nvdialog_rs::DialogBox;
/// use nvdialog_rs::DialogType;
/// use nvdialog_rs::Error;
///
/// fn main() {
///     // notice how we do not call nvdialog_rs::init() ?
///
///     if let Err(e) = DialogBox::new("Title", "Message", DialogType::Simple) {
///         // Will print 'Error creating dialog: Library has not been initialized yet'
///         eprintln!("Error creating dialog: {}", e.to_string());
///     }   
/// }
/// ```
///
/// ## Notes
/// - Stringified explanations of the errors are not done with NvDialog's `nvd_stringify_error()` function,
/// but using the [`thiserror`](https://crates.io/crates/thiserror) crate instead.
/// - While you could do this, you shouldn't manually call `Error::from(your_number)`, because the trait is
/// implemented for internal usage mainly.
#[derive(Debug, Error, Clone, Copy)]
pub enum Error {
    #[error("No error")]
    NoError = 0x0,
    #[error("No display found")]
    NoDisplay = 0xff,
    #[error("Backend failure occured")]
    BackendFailed,
    #[error("Invalid parameter passed")]
    ParametersError,
    #[error("Library has not been initialized yet")]
    NotYetInitialized,
    #[error("Invalid backend chosen")]
    InvalidBackend,
    #[error("Inaccessible file")]
    InaccessibleFile,
    #[error("Empty string passed")]
    EmptyString,
    #[error("Host has ran out of memory")]
    OutOfMemory,
    #[error("Internal NvDialog error")]
    InternalError,
    #[error("Already initialized NvDialog")]
    AlreadyInitialized,
}

impl From<i32> for Error {
    fn from(value: i32) -> Error {
        let mut result: Error = Error::NoError;
        [
            Self::NoError,
            Self::NoDisplay,
            Self::BackendFailed,
            Self::ParametersError,
            Self::NotYetInitialized,
            Self::InvalidBackend,
            Self::InaccessibleFile,
            Self::EmptyString,
            Self::OutOfMemory,
            Self::InternalError,
            Self::AlreadyInitialized,
        ]
        .into_iter()
        .for_each(|member| {
            if member as i32 == value {
                result = member;
            }
        });
        result
    }
}
