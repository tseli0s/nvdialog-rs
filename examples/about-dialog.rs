use nvdialog_rs::{AboutDialog, FileDialog, FileDialogType, Image, Object};
use std::process::abort;

fn main() {
    nvdialog_rs::init().expect("failed to initialize libnvdialog");
    let extensions = ["png", "jpeg", "jpg", "ico", "webp"];
    let filedlg = FileDialog::new(
        "Choose the icon for the dialog",
        FileDialogType::OpenFile,
        Some(extensions),
    );
    let filename = filedlg.retrieve_filename().unwrap_or_else(|| {
        eprintln!("No filename given!");
        abort()
    });
    let img = Image::from_filename(filename).unwrap_or_else(|e| {
        eprintln!("Failed to read image! {}", e.to_string());
        abort();
    });

    let dialog = AboutDialog::new()
        .name("NvDialog Example")
        .description(
            "An example of using nvdialog-rs and NvdAboutDialog, with a custom user-picked icon",
        )
        .icon(img)
        .build();

    dialog.show();
}
