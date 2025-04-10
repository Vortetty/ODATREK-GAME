#![feature(exitcode_exit_method)]

use std::{fs::File, path::Path, process::ExitCode};

pub struct OdatrekLogger {
    file_dir: String,
    file_name: String,
    file_log_level: Option<log::Level>, // Lowest that will be printed, default is all (trace, debug, info, warn, error)
    console_log_level: Option<log::Level>, // Lowest that will be printed, default is error (trace, debug, info, warn, error)
    file: File
}

impl OdatrekLogger {
    pub fn new(file_dir: String, file_log_level: Option<log::Level>, console_log_level: Option<log::Level>) -> Self {
        let file_name = chrono::Local::now().format("%d%m%Y_%H%M%S.log").to_string();
        let file = File::options().read(true).write(true).open(Path::new(&file_dir).join(file_name.clone()));
        if file.is_ok() {
            OdatrekLogger {
                file_dir,
                file_name,
                console_log_level,
                file_log_level,
                file: file.unwrap()
            }
        } else {
            println!("IO Error creating file {}", Path::new(&file_dir).join(file_name.clone()).to_str().unwrap());
            println!("> {:?}", file.unwrap_err());
            ExitCode::FAILURE.exit_process();
        }
    }
    pub fn stow_log(&self, max_logs: Option<u16>) { // Stows the log file as an lzma compressed file with the given name, deleting the oldest if there are too many
        let dir_path = Path::new(&self.file_dir);
    }
}

impl log::Log for OdatrekLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool { // Only calculates if it will be logged to the file, console is separate.
        if metadata.level() <= self.file_log_level.unwrap_or(log::Level::Trace) {
            return true;
        }
        false
    }

    fn log(&self, record: &log::Record) {
        todo!()
    }

    fn flush(&self) {
        todo!()
    }
}
