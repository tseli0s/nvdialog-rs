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

use std::ffi::c_char;

use crate::{NvdNotification, nvd_delete_notification, nvd_notification_new, add_null_byte, NvdNotifyType};

pub struct Notification {
    raw: *mut NvdNotification,
    shown: bool,
}

pub enum NotificationKind {
    Simple,
    Warning,
    Error
}

impl Notification {
    /// Creates a new notification and returns it.
    /// I
    /// ## Panic
    /// Panics if `nvd_notification_new` returns `null`.
    pub fn new<S: AsRef<str>>(title: S, msg: S, kind: NotificationKind) -> Self {
        let kind = match kind {
            NotificationKind::Simple => NvdNotifyType::NVD_NOTIFICATION_SIMPLE,
            NotificationKind::Warning => NvdNotifyType::NVD_NOTIFICATION_WARNING,
            NotificationKind::Error => NvdNotifyType::NVD_NOTIFICATION_ERROR
        };
        Self {
            raw: unsafe {
                let n = nvd_notification_new(
                    add_null_byte(title.as_ref()).as_ptr() as *const c_char,
                    add_null_byte(msg.as_ref()).as_ptr() as *const c_char,
                    kind
                );
                if n.is_null() {
                    panic!("nvd_notification_new returned NULL!")
                }
                n
            },
            shown: false,
        }
    }

    pub fn add_action<S: AsRef<str>>(name: S, f: S) -> bool {
        false
    }
}

impl Drop for Notification {
    fn drop(&mut self) {
        unsafe { nvd_delete_notification(self.raw) };
    }
}