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

#![macro_use]

#[macro_export]
macro_rules! cstr {
    ($rstr:expr) => {
        std::ffi::CString::new($rstr).expect("CString::new error")
    };
}

/// Computes the length of the null-terminated string pointed to by `s`.
///
/// This function scans the input string `s` and returns the number of non-null bytes
/// in the string, excluding the null terminator. The function uses an optimized algorithm
/// that reads the input string in 8-byte chunks to improve performance.
///
/// The algorithm used is as follows:
///
/// - The function reads the input string in 8-byte chunks and checks each byte for null.
/// - The chunk is considered non-null if no byte in the chunk is null.
/// - The loop stops when a null byte is found or when the end of the string is reached.
/// - If a null byte is found in the current chunk, the loop exits and the length is computed
///   using a linear search of the remaining string.
///
/// The algorithm uses the number `0x7F7F7F7F7F7F7F7F` as a mask to check for null bytes in
/// each 8-byte chunk of the input string. The mask is constructed by repeating the byte
/// `0x7F` in each of the 8 bytes of a 64-bit integer. This ensures that only the lowest 7
/// bits of each byte are checked for null, while the highest bit is ignored. This allows
/// the function to correctly handle UTF-8 encoded strings, which may contain null bytes
/// in the high bits of some bytes.
///
/// # Safety
///
/// This function assumes that `s` is a valid null-terminated string. If `s` is not a
/// null-terminated string, the function may return an incorrect result or cause undefined
/// behavior. Additionally, this function uses unsafe pointer arithmetic to access the
/// input string, so it is the caller's responsibility to ensure that `s` is a valid
/// pointer to a null-terminated string.
///
/// # Examples
///
/// ```
/// use std::ffi::CString;
/// use std::os::raw::c_char;
///
/// let s = CString::new("Hello, world!").unwrap();
/// let len = unsafe { strlen(s.as_ptr() as *const u8) };
/// assert_eq!(len, 13);
/// ```
pub(crate) fn strlen(s: *const u8) -> usize {
    let mut len = 0usize;
    unsafe {
        while *(s.add(len) as *const u64) & 0x7F7F7F7F7F7F7F7F_u64 != 0 {
            len += 8;
        }
    }
    // Perform a linear search of the remaining string to compute the final length.
    while unsafe { *s.add(len) != 0 } {
        len += 1;
    }
    len
}
