use super::Writer;

use log::*;
use reopen::Reopen;
use std::{
    fs,
    io::{Error, Write},
    path::Path,
};

#[cfg(not(windows))]
use signal_hook::consts::signal::SIGHUP;
#[cfg(windows)] // Windows has a very limited set of signals, but make it compile at least :-(
use signal_hook::consts::signal::SIGINT as SIGHUP;

fn open<P: AsRef<Path>>(p: P) -> Result<fs::File, Error> {
    info!("reopen {}", p.as_ref().display());
    fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(p)
}

pub struct FileWriter {
    file: Reopen<fs::File>,
    path: String,
}

impl FileWriter {
    pub fn new(path: &str) -> Self {
        let path_clone = path.to_string();
        let file = Reopen::new(Box::new(move || open(&path_clone))).unwrap();
        // Make sure it gets reopened on SIGHUP
        file.handle().register_signal(SIGHUP).unwrap();

        FileWriter {
            file,
            path: path.to_string(),
        }
    }
}

impl Writer for FileWriter {
    fn write(&mut self, s: &str) {
        if let Err(e) = writeln!(self.file, "{}", s) {
            error!("{}, {}", self.path, e);
        }
    }

    fn close(&mut self) {
        if let Err(e) = self.file.flush() {
            error!("{}, {}", self.path, e);
        }
    }
}
