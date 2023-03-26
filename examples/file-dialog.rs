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

extern crate nvdialog_rs;
use nvdialog_rs::DialogType;
use nvdialog_rs::FileDialog;
use nvdialog_rs::FileDialogType;
use nvdialog_rs::DialogBox;

fn main() {
    let argv0 = {
        let argv: Vec<String> = std::env::args().collect();
        &argv.clone()[0]
    };

    nvdialog_rs::init(String::from(argv0)).expect("Can't initialize Rustland");
    let file_dialog = FileDialog::new(
        String::from("Choose file"),
        FileDialogType::OpenFile
    );

    {
        let file = file_dialog.retrieve_filename();
        let dialog_box = DialogBox::new("File chosen", &file, DialogType::Simple);
        dialog_box.show();
    }
}