use std::fs::{self, DirEntry, FileType};

const DOWNLOADS_PATH: &'static str = "/home/quinn-caverly/Downloads/";
const EPUBS_PATH: &'static str = "/home/quinn-caverly/Epubs/";
const EPUB_EXT: &'static str = "epub";

const DAYS_ELAPSED_TO_DELETE: u64 = 10;
const SECONDS_ELAPSED_TO_DELETE: u64 = DAYS_ELAPSED_TO_DELETE * 24 * 60 * 60;

fn main() {
    for dir_entry_result in fs::read_dir(DOWNLOADS_PATH).unwrap() {
        let dir_entry = dir_entry_result.unwrap();

        let file_type = dir_entry.file_type().unwrap();

        if file_type.is_file()
            && dir_entry.path().extension().unwrap().to_str().unwrap() == EPUB_EXT
        {
            move_epub(dir_entry.path().to_str().unwrap());
        } else {
            clear_if_necessary(&dir_entry, &file_type);
        }
    }
}

fn move_epub(entire_path: &str) {
    let ender = entire_path.split("/").last().unwrap();
    fs::rename(entire_path, EPUBS_PATH.to_string() + ender).unwrap();
}

fn clear_if_necessary(dir_entry: &DirEntry, file_type: &FileType) {
    let last_modified = dir_entry.metadata().unwrap().modified().unwrap();
    let elapsed_time = last_modified.elapsed().unwrap().as_secs();

    if elapsed_time > SECONDS_ELAPSED_TO_DELETE {
        if file_type.is_dir() {
            fs::remove_dir_all(dir_entry.path()).unwrap();
        } else if file_type.is_file() {
            fs::remove_file(dir_entry.path()).unwrap();
        } else {
            panic!("Did not expect neither a file or a directory.");
        }
    }
}
