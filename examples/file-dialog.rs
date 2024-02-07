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

extern crate nvdialog_rs;
use nvdialog_rs::DialogBox;
use nvdialog_rs::DialogType;
use nvdialog_rs::FileDialog;
use nvdialog_rs::FileDialogType;

fn main() {
    nvdialog_rs::init().expect("Can't initialize NvDialog");
    let mut file_dialog = FileDialog::new(
        String::from("Select an image"),
        FileDialogType::OpenFile,
        Some(vec![
            "png".to_owned(),
            "jpg".to_owned(),
            "jpeg".to_owned(),
            "ico".to_owned(),
            "webp".to_owned(),
        ]),
    );

    if let Some(file) = file_dialog.retrieve_filename() {
        DialogBox::new("File chosen", &file.to_str().unwrap(), DialogType::Simple)
            .expect("Can't create dialog")
            .show()
    } else {
        DialogBox::new("Error", "No file chosen", DialogType::Error)
            .expect("Can't create dialog")
            .show()
    }
}
