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

use std::{
    ffi::{c_char, CStr}, fmt::Display, ptr::null
};

use crate::cstr;
use nvdialog_sys::ffi::*;

/// A Rust version of `NvdDynamicString`
///
/// A [`DynamicString`] is effectively a conversion of a heap allocated string designed to work with NvDialog
/// that works with the Rust side. In essence, it's just another string type for Rust, but works with this crate instead.
/// This type is useful since version v0.10 of NvDialog when the API was completely rewritten to use `NvdDynamicString`, a separate
/// string type that addresses the limitations of working with raw character buffers and safety concerns.
///
#[derive(Debug)]
pub struct DynamicString {
    native: *mut NvdDynamicString,
    this: String,
}

impl DynamicString {
    /// Create a new `DynamicString` from optional UTF-8 data.
    ///
    /// - If `data` is `Some`, the contents are passed to the FFI (as a C string)
    ///   and mirrored into `Self` as well.
    /// - If `data` is `None`, a null is passed to `nvd_string_new`, which is
    ///   expected (by the library) to produce an empty string.
    /// 
    /// Note that if you want to duplicate a string, you can use [`DynamicString::duplicate`] instead of
    /// allocating a brand new one.
    ///
    /// ## Safety
    /// The returned object can be modified safely, however note that any modifications are permanent and
    /// if you received the `DynamicString` from another function, then trying to fetch it again will return your
    /// modified string as the internal pointer was modified.
    pub fn new<S: AsRef<str>>(data: Option<S>) -> Self {
        if let Some(s) = data {
            let string = cstr!(s.as_ref());
            Self {
                native: unsafe {
                    let s = nvd_string_new(string.as_ptr());
                    if s.is_null() {
                        panic!("libnvdialog did not produce a valid NvdDynamicString!");
                    }
                    s
                },
                this: String::from(s.as_ref()),
            }
        } else {
            Self {
                native: unsafe {
                    let s = nvd_string_new(null());
                    if s.is_null() {
                        panic!("libnvdialog did not produce a valid NvdDynamicString!");
                    }
                    s
                },
                this: String::new(),
            }
        }
    }

    /// Returns a borrowed pointer to the internal string's buffer.
    ///
    /// ## Safety
    /// The returned pointer is only valid as long as `self` is. Additionally, if the internal data was corrupted,
    /// this function may corrupt the program's memory, therefore make sure `self` was not internally modified beforehand.
    /// ## Returns
    /// A NULL-terminated, heap-allocated C-style string on success, or NULL if `nvd_string_to_cstr` failed.
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const c_char {
        assert!(!self.native.is_null());
        nvd_string_to_cstr(self.native)
    }

    /// Create a deep copy of this `DynamicString`.
    ///
    /// Note that this function returns a truly unique `DynamicString` - It is safe to modify the returned value as
    /// `self` does not point to it (and vice versa).
    ///
    /// ## Safety
    /// This function is safe to use, unless the internal structure was corrupted beforehand, in which case `nvd_duplicate_string`
    /// may return invalid or corrupt data. Make sure `self` has not been modified incorrectly before calling this function.
    /// 
    /// ## Returns
    /// A deep copy of `self`, assuming no errors from the FFI-side.

    #[inline]
    pub fn duplicate(&self) -> Self {
        Self {
            native: unsafe { nvd_duplicate_string(self.native) },
            this: self.this.clone(),
        }
    }

    /// Returns a reference to the internal [`String`] that mirrors the FFI string to avoid extra allocations.
    /// # Safety
    /// This function is always safe to use. If you modify `self` correctly, then the contents are always mirrored.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.this
    }
}

impl From<*mut NvdDynamicString> for DynamicString {
    fn from(value: *mut NvdDynamicString) -> Self {
        assert!(!value.is_null());
        Self {
            native: value,
            this: unsafe {
                String::from(CStr::from_ptr(nvd_string_to_cstr(value)).to_string_lossy())
            },
        }
    }
}

impl From<&str> for DynamicString {
    fn from(s: &str) -> Self {
        let string = cstr!(s);
        Self {
            native: unsafe { nvd_string_new(string.as_ptr()) },
            this: s.to_owned(),
        }
    }
}

impl From<String> for DynamicString {
    fn from(data: String) -> Self {
        let string = cstr!(&*data);
        Self {
            native: unsafe { nvd_string_new(string.as_ptr()) },
            this: data,
        }
    }
}

impl Display for DynamicString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.this)
    }
}

impl Clone for DynamicString {
    fn clone(&self) -> Self {
        assert!(!self.native.is_null());
        self.duplicate()
    }
}

impl Drop for DynamicString {
    fn drop(&mut self) {
        unsafe { nvd_delete_string(self.native) }
    }
}
