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

use std::path::PathBuf;
use nvdialog_sys::ffi::*;
use thiserror::Error;

use crate::cstr;

/// A simple image loaded from NvDialog. This is the equivalent to `NvdImage`, although the implementation
/// makes a few extra safety checks from the Rust side.
pub struct Image {
    pub raw: *mut NvdImage,
    path: Option<PathBuf>,
}

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("Path does not exist in the filesystem")]
    PathNotFound,
    #[error("Could not read the image's contents")]
    ReadingError,
    #[error("Image is in invalid format and cannot be read")]
    InvalidFormat,
    #[error("Unknown error while reading an image - Try using nvd_get_error()")]
    UnknownError,
}

impl Image {
    /// Creates an `Image` from a specified filename.
    ///
    /// This function attempts to load an image from the given file path using 
    /// NvDialog's image loading facilities. It performs a few checks to ensure 
    /// robustness and safety from the Rust side.
    ///
    /// # Arguments
    ///
    /// * `filename` - A `PathBuf` specifying the file path of the image to be loaded.
    ///
    /// # Returns
    ///
    /// * `Result<Self, ImageError>` - Returns an `Ok` variant containing the `Image` 
    ///   if successful, or an `ImageError` if an error occurs during the process.
    ///
    /// # Errors
    ///
    /// * `ImageError::PathNotFound` - Returned if the file path does not exist.
    /// * `ImageError::UnknownError` - Returned if the image cannot be created from
    ///   the data.
    ///
    /// # Safety
    ///
    /// This function makes use of `unsafe` blocks to interface with the C functions 
    /// from NvDialog. It is assumed that `nvd_image_from_filename` and 
    /// `nvd_create_image` are safe to call with the provided arguments.
    pub fn from_filename(filename: PathBuf) -> Result<Self, ImageError> {
        if !filename.exists() { return Err(ImageError::PathNotFound); }
        let mut w = 0;
        let mut h = 0;
        let data = unsafe { nvd_image_from_filename(cstr!(filename.to_str().unwrap()).as_ptr(), &mut w, &mut h) };
        let raw = unsafe { nvd_create_image(data, w, h) };
        if raw.is_null() { return Err(ImageError::UnknownError); }

        Ok(Self {
            raw,
            path: Some(filename)
        })
    }

    /// Creates an `Image` from the raw data of an image file.
    ///
    /// This function creates an `Image` from raw data that is assumed to be in a format
    /// that NvDialog can read. The data is sent to the NvDialog library without any
    /// checking on the Rust side, so it is left up to the user to ensure that the data
    /// is valid.
    ///
    /// # Safety
    ///
    /// This function is marked `unsafe` because the data passed to it is not checked
    /// in any way. If the data is not valid, it may cause undefined behavior when
    /// passed to the C library.
    ///
    /// # Arguments
    ///
    /// * `data` - The raw image data. This is assumed to be the contents of an image
    ///   file and should be in a format that NvDialog can read.
    /// * `width` - The width of the image in pixels.
    /// * `height` - The height of the image in pixels.
    ///
    /// # Returns
    ///
    /// * `Result<Self, ImageError>` - Returns an `Ok` variant containing the `Image` if
    ///   successful, or an `ImageError` if an error occurs.
    ///
    /// # Errors
    ///
    /// * `ImageError::InvalidFormat` - Returned if the data is not in a valid format
    ///   for NvDialog.
    /// * `ImageError::UnknownError` - Returned if the image cannot be created from the
    ///   data for some other reason.
    pub unsafe fn from_data(data: &[u8], width: u32, height: u32) -> Result<Self, ImageError> {
        if data.len() <= 1 { return Err(ImageError::ReadingError) }
        if data.len() % 4 != 0 {
            return Err(ImageError::InvalidFormat);
        }
        
        let raw = nvd_create_image(data.as_ptr(), width as i32, height as i32);
        if raw.is_null() {
            return Err(ImageError::UnknownError);
        }

        Ok(Self {
            raw,
            path: None
        })
    }

    pub(crate) fn get_raw(&self) -> *mut NvdImage {
        self.raw
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            nvd_destroy_image(self.raw)
        }
    }
}