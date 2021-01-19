use rotating_file::RotatingFile;

use super::Writer;

use log::*;

pub(crate) struct RotatedFileWriter {
    file: RotatingFile,
}

impl RotatedFileWriter {
    pub fn new(root_dir: &str, prefix: &str) -> Self {
        RotatedFileWriter {
            file: RotatingFile::new(
                root_dir,
                None,
                Some(3600), // 3600 seconds = one hour
                Some(rotating_file::Compression::GZip),
                Some("%Y-%m-%d-%H".to_string()),
                Some(prefix.to_string()),
                Some(".json".to_string()),
            ),
        }
    }
}

impl Writer for RotatedFileWriter {
    fn write(&self, s: &str) {
        if let Err(e) = self.file.writeln(s) {
            error!("{}", e);
        }
    }

    fn close(&self) {
        self.file.close();
    }
}
