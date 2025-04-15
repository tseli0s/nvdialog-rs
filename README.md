<div align="center" style="padding-top: 25px;">
        <img src="assets/logo.svg" width="256px">
        <h1>nvdialog-rs: Safe, elegant Rust bindings for <a href="https://github.com/tseli0s/nvdialog">NvDialog</a></h1>
        <p>
        `nvdialog-rs` provides high-level Rust bindings to the libnvdialog library, enabling developers to easily integrate native dialog boxes into their applications. This crate eliminates the complexity of working directly with C pointers, ensuring a safe and ergonomic Rust interface that allows Rust projects to safely use the libnvdialog library.
        </p>
        <p>
        For your convenience, libnvdialog is built and linked along with the crate and packaged as a static library along with your application.
        The heavy work is done by the <a href="./nvdialog-sys/"><code>nvdialog-sys</code></a> crate which provides a bridge between the C and Rust projects. This means that you won't need to ship a copy of NvDialog along with your application.
        </p>
        <img src="https://img.shields.io/crates/v/nvdialog-rs?style=flat-square">
        <img src="https://img.shields.io/docsrs/nvdialog-rs?label=Documentation&style=flat-square">
        <img src="https://img.shields.io/github/license/tseli0s/nvdialog-rs?style=flat-square">
</div>

# Features
- **Rust Safety**: Enjoy the power of NvDialog with the full safety and ergonomics of Rust.
- **Native Dialogs**: Leverage your OS's native API to create platform-specific dialogs (e.g., file pickers, notifications, etc.).
- **Comprehensive Dialog Support**: From file pickers to custom dialog boxes, nvdialog-rs supports the full range of dialog types provided by NvDialog.
- **Minimal Overhead**: Designed for efficiency with minimal performance impact.
- **Simple API**: The crate provides an easy, human-readable API to interact with libnvdialog.
- **Light Dependency**: Only one tiny dependency—no unnecessary bloat.

# Installation and usage
To add `nvdialog-rs` to your project, simply include it in your Cargo.toml file:
```toml
[dependencies]
nvdialog-rs = "0.3.1"
```

# Example
```rust
extern crate nvdialog_rs;
use nvdialog_rs::DialogBox;
use nvdialog_rs::DialogType;

fn main() {
        /* Initialize the library. This corresponds to `nvd_init` */
        nvdialog_rs::init().expect("Failed to initialize NvDialog");

        /* Creating the dialog box. */
        DialogBox::new(
                "Hello from Rust!", /* Title of the dialog */
                /* Message of the dialog */
                "This dialog has been created using Rust and NvDialog bindings to the language.",
                /* See documentation for more */
                DialogType::Simple
        ).expect("Couldn't create dialog");
         .show();
}
```
In just a few lines of code, you can create and display a dialog with a native look and feel on your system. The library is designed to minimize boilerplate and ensure a smooth user experience.

# License
This library is licensed under the MIT License:
```
The MIT License (MIT)
Copyright © 2022-2025 Aggelos Tselios

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

# Contributing
We welcome contributions to `nvdialog-rs!` Whether you're reporting bugs, suggesting features, or submitting code improvements, your input is valuable:

- Fork the repository.
- Create a new branch for your changes.
- Submit a pull request with a clear description of your changes and their purpose.

# Support
If you encounter any issues or have questions, please feel free to open an issue in the GitHub repository. We also encourage you to check the documentation for detailed information on how to use the crate.

