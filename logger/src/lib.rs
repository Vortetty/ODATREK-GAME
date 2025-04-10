#![feature(exitcode_exit_method)]

use std::{cell::RefCell, fs::{self, DirEntry, File}, io::{Read, Seek, Write}, path::Path, process::ExitCode, sync::RwLock};
use liblzma::{read::XzEncoder, stream::{LzmaOptions, Stream}};

pub struct OdatrekLogger {
    file_dir: String,
    file_name: String,
    file_log_level: Option<log::Level>, // Lowest that will be printed, default is all (trace, debug, info, warn, error)
    console_log_level: Option<log::Level>, // Lowest that will be printed, default is error (trace, debug, info, warn, error)
    file: RwLock<Option<File>>
}

impl OdatrekLogger {
    pub const fn new() -> Self {
        OdatrekLogger {
            file_dir: String::new(),
            file_name: String::new(),
            file_log_level: None,
            console_log_level: None,
            file: RwLock::new(None)
        }
    }
    pub fn init(&mut self, file_dir: String, file_log_level: Option<log::Level>, console_log_level: Option<log::Level>) {
        fs::create_dir_all(file_dir.clone());
        let file_name = chrono::Local::now().format("%S%M%H_%d%m%Y.log").to_string();
        let file = File::options().create(true).read(true).write(true).open(Path::new(&file_dir).join(file_name.clone()));
        if file.is_ok() {
            self.file_dir = file_dir;
            self.file_name = file_name;
            self.console_log_level = console_log_level;
            self.file_log_level = file_log_level;
            self.file = RwLock::new(Some(file.unwrap()));
        } else {
            println!("IO Error creating file {}", Path::new(&file_dir).join(file_name.clone()).to_str().unwrap());
            println!("> {:?}", file.unwrap_err());
            ExitCode::FAILURE.exit_process();
        }
    }
    pub fn stow_log(&mut self, max_xz_logs: Option<usize>, compress_level: Option<u32>) { // Stows the log file as an lzma compressed file with the given name, deleting the oldest if there are too many. defaults to 10 logs and xz level 9.
        self.file.write().unwrap().as_mut().unwrap().flush();

        let dir_path = Path::new(&self.file_dir);
        let mut xz_logs: Vec<DirEntry> = dir_path.read_dir().unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                if entry.path().extension().map_or(false, |ext| ext == "xz") {
                    Some(entry)  // Keep the .xz file
                } else {
                    None  // Discard non-.xz files
                }
            }).collect();

        xz_logs.sort_by(|a, b| b.file_name().cmp(&a.file_name())); // sort for oldest first newest last

        while xz_logs.len() >= max_xz_logs.unwrap_or(10).min(1) {
            fs::remove_file(xz_logs.pop().unwrap().path());
        }

        let mut filecopy = self.file.write().unwrap().as_mut().unwrap().try_clone().unwrap();
        filecopy.seek(std::io::SeekFrom::Start(0));
        let mut reader = XzEncoder::new_stream(self.file.write().unwrap().as_mut().unwrap().try_clone().unwrap(), Stream::new_easy_encoder(compress_level.unwrap_or(9), liblzma::stream::Check::Crc32).unwrap());
        let mut xz_out = File::options().create(true).write(true).open(dir_path.join(self.file_name.clone() + ".xz")).unwrap();
        let mut writer = std::io::BufWriter::new(xz_out);

        let mut buf: [u8;1024] = [0; 1024];
        while let Ok(x) = reader.read(&mut buf) {
            writer.write(&buf[..x]);
        }

        writer.flush();
        fs::remove_file(dir_path.join(self.file_name.clone()));
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
        if record.metadata().level() <= self.file_log_level.unwrap_or(log::Level::Trace) {
            self.file.write().unwrap().as_mut().unwrap().write(format!("{:?}\n", record).as_bytes());
        }
        if record.metadata().level() <= self.console_log_level.unwrap_or(log::Level::Error) {
            println!("{:?}\n", record);
        }
    }

    fn flush(&self) {
        todo!()
    }
}
