pub(super) mod rotated_file_writer;

pub trait Writer {
    fn write(&self, s: &str);
    fn close(&self);
}

pub(super) use rotated_file_writer::RotatedFileWriter;
