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

use std::ffi::c_void;

use crate::{cstr, string::DynamicString, Object};
use nvdialog_sys::ffi::*;

/// A struct representing an input box.
///
/// Input boxes are similar to [`DialogBox`](crate::DialogBox) ones but instead of just showing text, they also allow the user to
/// provide some text input. The input field is always single-line. In addition, the [`InputBox`] can display some optional text
/// to provide more information to the user (Like "Enter your name here").
///
/// # Examples
///
/// Showing a simple input box, then retrieving the input:
/// 
/// ```
/// use std::process::abort;
/// use nvdialog_rs::InputBox;
/// 
/// nvdialog_rs::init().unwrap_or_else(|e| {
/// eprintln!("Failed to initialize NvDialog: {}", e.to_string());
/// abort();
/// });
/// let mut input_box = InputBox::new(
///     "Input Box",
///     "Please enter some text here. The text will be printed to stdout.",
/// );
/// input_box.display();
/// let input = input_box.get_input();
/// if let Some(string) = input {
///     println!("You entered the following text: \"{}\"", string);
/// } else {
///     eprintln!("You didn't enter anything!");
/// }
/// ```
///
/// # Safety
/// The returned string's contents, as described in the NvDialog documentation, is heap-allocated, and therefore it's safe to modify this
/// string through methods inside the struct. However, **any modifications are applied to the original string DIRECTLY**, meaning that if
/// you wish to preserve the original string but still modify it somewhere, you should duplicate it instead.
/// # FFI
/// Corresponds to `NvdInputBox`.
pub struct InputBox {
    raw: *mut NvdInputBox,
    user_input: Option<DynamicString>,
}

impl InputBox {
    #[inline]
    pub fn new<S: AsRef<str>>(title: S, prompt: S) -> Self {
        let title = cstr!(title.as_ref());
        let prompt = cstr!(prompt.as_ref());

        Self {
            raw: unsafe { nvd_input_box_new(title.as_ptr(), prompt.as_ptr()) },
            user_input: None,
        }
    }

    #[inline]
    pub fn display(&mut self) {
        unsafe {
            nvd_show_input_box(self.raw);
            let str = DynamicString::from(nvd_input_box_get_string(self.raw));
            self.user_input = Some(str);
        }
    }

    #[inline]
    pub fn get_input(&mut self) -> Option<DynamicString> {
        self.user_input.clone()
    }
}

impl Object for InputBox {
    type NativeType = NvdInputBox;
    type ReturnValue = ();

    fn get_raw(&self) -> *mut Self::NativeType {
        unimplemented!("NvdInputBox does not have an API to get the raw object yet.");
    }

    fn show(&self) -> Self::ReturnValue {
        unimplemented!("Due to trait limitations, use InputBox::display() instead. Object::show() requires that the internal data is not mutated.")
    }

    fn free(&mut self) {
        unsafe { nvd_free_object(self.raw as *mut c_void) }
    }
}

impl Drop for InputBox {
    fn drop(&mut self) {
        self.free();
    }
}
