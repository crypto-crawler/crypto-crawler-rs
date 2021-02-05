pub(super) mod file_writer;

pub trait Writer {
    fn write(&self, s: &str);
    fn close(&self);
}

pub(super) use file_writer::FileWriter;
