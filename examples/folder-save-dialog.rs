use nvdialog_rs::{FileDialog, FileDialogType};

fn main() {
    nvdialog_rs::init().expect("failed to initialize nvdialog");
    let mut folder_dialog = FileDialog::new("Choose a folder", FileDialogType::OpenFolder, None::<Vec<&str>>);
    let mut where_to_save = FileDialog::new("Save as...", FileDialogType::SaveFile, None::<Vec<&str>>);
    
    if let Some(p) = folder_dialog.retrieve_filename() {
        println!("You chose folder {}", p.display());
    }

    if let Some(s) = where_to_save.retrieve_filename() {
        println!("Saved as {}", s.display());
    }
}