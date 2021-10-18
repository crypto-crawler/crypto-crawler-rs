pub(super) mod file_writer;

pub trait Writer {
    fn write(&mut self, s: &str);
    fn close(&mut self);
}

pub use file_writer::FileWriter;
