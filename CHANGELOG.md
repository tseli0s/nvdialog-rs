## v0.3.1
- Fixed linking errors on Linux with Gtk
- Added image API
- Update file dialog API
- Fix `c_string!()` macro warning
- Fix build on Windows

## v0.3.0
- Moved completely to the `nvdialog-rs` crate. 
- Fixed many compiler warnings.
- Updated copyright to 2024.

## v0.2.0
- Added notification support
- Added `Error` type
- C string creation will now use `CString` which is safer than the previous implementation.
- Removed `libc` dependency.
- All dialogs now return a Result<> to match with recent NvDialog changes.
- Largely improved documentation.

## v0.1.0
- Updated version to v0.1.0
- Updated copyright
- Made all strings more generic
- Wherever required, mutable references will now be used.

## Initial Publish
*This is not a changelog but a list of issues that have to be resolved*
- File dialogs do not work properly: They either segfault or corrupt memory.
- Even in debug mode, NvDialog is still linking in the system library which never provides
debug symbols.
- Missing notification bindings.
- Heavy use of `unsafe`. We should probably reduce it.