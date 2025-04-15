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

use nvdialog_sys::ffi::*;

use crate::{cstr, Image};

/// A struct for a dialog to show about your application.
/// 
/// Dialogs like this are used in the traditional Help > About dialogs found in most programs.
/// In this case, this `AboutDialog` uses, like the rest of the library, the OS' native toolkit to show
/// it. This may create inconsistency in some situations, for example in web apps.
/// 
/// # Examples
/// Basic about dialog:
/// ```rust
/// use nvdialog_rs::AboutDialog;
/// 
/// let dialog = AboutDialog::new()
///                 .name("App Name".into())
///                 .description("A short description for your app".into())
///                 .version("0.1.0")
///                 .build();
/// 
/// dialog.show()
/// ```
pub struct AboutDialog {
    app_name: String,
    details: String,
    version: String,
    icon: Option<Image>,
    raw: *mut NvdAboutDialog
}

impl AboutDialog {
    pub fn new() -> Self {
        Self {
            app_name: String::new(),
            details: String::new(),
            version: String::new(),
            icon: None,
            raw: std::ptr::null_mut(),
        }
    }

    pub fn name<S: ToString>(mut self, name: S) -> Self {
        self.app_name = name.to_string();
        self
    }

    pub fn description<S: ToString>(mut self, description: S) -> Self {
        self.details = description.to_string();
        self
    }

    pub fn version<S: ToString>(mut self, version: S) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn icon(mut self, icon: Image) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn build(mut self) -> Self {
        let dialog = unsafe {
            let n = cstr!(&*self.app_name);
            let d = cstr!(&*self.details);
            let v = cstr!(&*self.version);
            let raw = nvd_about_dialog_new(
                n.as_ptr(),
                d.as_ptr(),
                std::ptr::null_mut()
            );
            nvd_about_dialog_set_version(raw, v.as_ptr());
            if let Some(ref i) = self.icon {
                nvd_dialog_set_icon(raw, i.get_raw())
            }
            raw
        };
        
        self.raw = dialog;
        self
    }

    pub fn show(&mut self) {
        unsafe {
            nvd_show_about_dialog(self.raw)
        }
    }
}