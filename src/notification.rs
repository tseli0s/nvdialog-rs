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

use crate::c_string;
use nvdialog_sys::ffi::*;

/// A notification dialog, which can be used to send a notification to the user.
///
/// # Examples
/// ```
/// use nvdialog_rs::Notification;
///
/// let mut notification = Notification::new("Hello world!", "This is a notification.");
/// notification.send();
/// ```
pub struct Notification {
    raw: *mut NvdNotification,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotificationKind {
    Simple,
    Warning,
    Error,
}

impl Into<u32> for NotificationKind {
    fn into(self) -> u32 {
        if self == NotificationKind::Simple {
            NotificationKind::Simple as u32
        } else if self == NotificationKind::Warning {
            NotificationKind::Warning as u32
        } else if self == NotificationKind::Error {
            NotificationKind::Error as u32
        } else {
            NotificationKind::Simple as u32
        }
    }
}

impl Notification {
    /// Constructs a new `Notification` object with the given title, message and kind of notification.
    ///
    /// # Arguments
    ///
    /// * `title`: A string for the title / summary of the notification.
    /// * `msg`: The actual body of the notification.
    /// * `kind`: The kind of the notification, see [`NotificationKind`].
    ///
    /// # Errors
    /// Returns an `Error` of type `OutOfMemory` if NvDialog's allocation failed.
    ///
    /// # Examples
    /// ```
    /// use nvdialog_rs::{Notification, NotificationKind};
    /// let notification = Notification::new("Hello", "This is a notification", NotificationKind::Simple);
    /// ```
    /// ## FFI
    /// Correspons to `nvd_notification_new`.
    pub fn new<S: AsRef<str>>(
        title: S,
        msg: S,
        kind: NotificationKind,
    ) -> Result<Self, crate::Error> {
        let t = c_string!(title.as_ref());
        let m = c_string!(msg.as_ref());
        let raw = unsafe { nvd_notification_new(t.as_ptr(), m.as_ptr(), kind.into()) };

        if raw.is_null() {
            return Err(crate::Error::OutOfMemory);
        }
        Ok(Self { raw })
    }

    pub fn add_action<S: AsRef<str>>(&mut self, name: S, val: i32, ptr: &mut i32) {
        let a = c_string!(name.as_ref());
        unsafe {
            nvd_add_notification_action(self.raw, a.as_ptr(), val, ptr);
        }
    }

    /// Sends the notification to the desktop notification system. If the notification has
    /// already been shown or sent, calling this method again will have no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use nvdialog_rs::{Notification, NotificationKind};
    ///
    /// let mut notification = Notification::new("Title", "Body", NotificationKind::Simple)
    ///     .expect("Failed to create notification");
    ///
    /// // Send the notification
    /// notification.send();
    /// ```
    ///
    /// Note that calling `send` multiple times will result in multiple notifications being shown:
    ///
    /// ```
    /// use nvdialog_rs::{Notification, NotificationKind};
    /// let mut notification = Notification::new("Title", "Body", NotificationKind::Simple)
    ///     .expect("Failed to create notification");
    /// // Send the notification
    /// notification.send();
    /// ```
    ///
    /// It is safe to call this method on a notification that has not yet been shown, as well as
    /// on a notification that has already been shown or sent.
    /// ```
    /// # use nvdialog_rs::{Notification, NotificationKind};
    /// # let mut notification = Notification::new("Title", "Body", NotificationKind::Simple)
    /// #     .expect("Failed to create notification");
    /// // Send the notification
    /// notification.send();
    ///
    /// // It is safe to call `send` multiple times
    /// notification.send();
    ///
    /// // It is also safe to call `send` on a notification that hasn't been shown yet
    /// let mut other_notification = Notification::new("Other title", "Other body", NotificationKind::Simple)
    ///     .expect("Failed to create notification");
    /// other_notification.send();
    /// ```
    ///
    /// # FFI
    /// Corresponds to `nvd_send_notification`.
    pub fn send(&mut self) {
        unsafe { nvd_send_notification(self.raw) }
    }
}

impl Drop for Notification {
    fn drop(&mut self) {
        unsafe { nvd_delete_notification(self.raw) };
    }
}
