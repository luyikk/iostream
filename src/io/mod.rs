pub use self::file::*;
pub use self::memory::memory_stream::MemoryStream;
pub use self::stream_rw::stream_reader::StreamReader;
pub use self::stream_rw::stream_writer::StreamWriter;
use std::error::Error;
use StreamError;
use StreamErrorKind;

pub mod file;
pub mod memory;
pub mod stream_rw;

//     Specifies the position in a stream to use for seeking.
pub enum SeekOrigin {
    ///
    ///     Specifies the beginning of a stream.
    ///
    Begin = 0,
    ///
    ///
    ///     Specifies the current position within a stream.
    Current = 1,
    ///
    ///
    ///     Specifies the end of a stream.
    End = 2,
}

pub trait Stream {
    ///
    /// copy current stream to target stream
    /// # Examples
    ///```rust
    ///  use iostream::io::*;
    ///  use std::fs::File;
    ///
    ///  let mut ms = MemoryStream::new();
    ///  let data:[u8;50]=[54;50];
    ///  ms.write(&data,0,50).unwrap();
    ///
    ///  let mut fs=File::open_fs("c.data",FileMode::CreateNew,FileAccess::ReadWrite).unwrap();
    ///  ms.copy_to(&mut fs);
    /// ```
    fn copy_to(&mut self, target: &mut Stream) -> Result<(), Box<dyn Error>> {
        if !self.can_read() {
            StreamError::from_str("the current stream cannot read")?
        }

        if !target.can_write() {
            StreamError::from_str("the target stream cannot read")?
        }

        let _potion = self.position();
        self.set_position(0)?;
        const LENGTH: usize = 1024;
        let mut buffs: [u8; LENGTH] = [0; LENGTH];
        loop {
            let result = self.read(&mut buffs, 0, LENGTH);

            match result {
                Ok(size) => {
                    if size == 0 {
                        break;
                    } else {
                        let mut lp = target.write(&buffs, 0, size)?;
                        while lp < size {
                            let have = size - lp;
                            lp = target.write(&buffs, lp, have)?;
                        }
                    }
                }
                Err(e) => {
                    if let StreamErrorKind::End = e.err_type {
                        break;
                    } else {
                        return Err(Box::new(e));
                    }
                }
            }
        }
        target.flush()?;
        self.set_position(_potion)?;
        Ok(())
    }

    /// gets or sets the position within the current stream
    fn set_position(&mut self, _potion: u64) -> Result<(), Box<dyn Error>>;
    /// The current position within the stream.
    fn position(&mut self) -> u64;
    /// A long value representing the length of the stream in bytes.
    fn length(&mut self) -> u64;
    /// Sets the length of this stream to the given value.
    fn set_length(&mut self, len:u64) -> Result<(), Box<dyn Error>>;
    /// gets a value indicating whether the current stream supports writing.
    /// true if the stream supports writing; otherwise, false.
    fn can_write(&mut self) -> bool;
    /// gets a value indicating whether the current  stream supports reading.
    /// true if the stream supports reading; otherwise, false.
    fn can_read(&mut self) -> bool;

    /// gets a value indicating whether the current  stream supports seeking.
    fn can_seek(&mut self) -> bool;
    /// When overridden in a derived class, reads a sequence of bytes from the current
    /// stream and advances the position within the stream by the number of bytes read.
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, StreamError>;

    /// reads a sequence of bytes from the current
    /// stream and advances the position within the stream by the number of bytes read.
    fn read(&mut self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, StreamError>;

    /// Reads a byte from the stream and advances the position within the stream by one
    /// byte, or Err returns -1 if at the end of the stream.
    fn read_byte(&mut self) -> Result<u8, StreamError>;

    /// sets the position within the current stream.
    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<u64, Box<dyn Error>>;

    /// writes a sequence of all bytes to the current stream
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Box<dyn Error>>;

    /// writes a sequence of bytes to the current
    /// stream and advances the current position within this stream by the number of
    /// bytes written.
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, Box<dyn Error>>;

    /// clears all buffers for this stream and causes
    /// any buffered data to be written to the underlying device.
    fn flush(&mut self) -> Result<(), Box<dyn Error>>;
}
