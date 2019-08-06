use super::super::Stream;
use super::*;
use std::error::Error;
use {StreamError, StreamErrorKind};

///reader stream struct
pub struct StreamReader<'a> {
    pub base_stream: &'a mut dyn Stream,
    endian: EndianType,
}

impl<'a> StreamReader<'a> {
    /// from stream use LittleEndian reader integer
    /// # Examples
    ///```rust
    /// use iostream::io::*;
    /// let mut ms=MemoryStream::new();
    /// //...write data to ms
    /// let mut ws: StreamReader = StreamReader::from(&mut ms).unwrap();
    /// ```
    pub fn from(stream: &'a mut dyn Stream) -> Result<StreamReader<'a>, String> {
        if !stream.can_read() {
            return Err("the stream not can read".to_string());
        }

        Ok(StreamReader {
            base_stream: stream,
            endian: EndianType::LittleEndian,
        })
    }
    /// from stream use BigEndian reader integer
    /// # Examples
    ///```rust
    /// use iostream::io::*;
    /// let mut ms=MemoryStream::new();
    /// //...write data to ms
    /// let mut ws: StreamReader = StreamReader::from_big_endian(&mut ms).unwrap();
    /// ```
    pub fn from_big_endian(stream: &'a mut dyn Stream) -> Result<StreamReader<'a>, String> {
        if !stream.can_read() {
            return Err("the stream not can read".to_string());
        }

        Ok(StreamReader {
            base_stream: stream,
            endian: EndianType::BigEndian,
        })
    }

    /// peek the current stream next byte
    pub fn peek(&mut self) -> Result<u8, StreamError> {
        let d = self.base_stream.read_byte()?;
        let p = self.base_stream.position() - 1;
        let r = self.base_stream.set_position(p);

        if let Err(e) = r {
            StreamError::from_str(&format!("{}", e))?
        }

        Ok(d)
    }

    ///read u8 from stream
    pub fn read_byte(&mut self) -> Result<u8, StreamError> {
        Ok(self.base_stream.read_byte()?)
    }

    ///read vec<u8> from stream,return read length
    pub fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, StreamError> {
        Ok(self.base_stream.read_all(buf)?)
    }

    ///read [u8] from stream,can use offset and count buff position,return read len
    pub fn read(
        &mut self,
        buf: &mut [u8],
        offset: usize,
        count: usize,
    ) -> Result<usize, StreamError> {
        Ok(self.base_stream.read(buf, offset, count)?)
    }

    ///read i16 from stream,return it value
    pub fn read_i16(&mut self) -> Result<i16, StreamError> {
        let mut data: [u8; 2] = [0; 2];
        let r = self.read(&mut data, 0, 2)?;

        if r == 2 {
            let result: i16 = (data[0] as i16 | ((data[1] as i16) << 8)) as i16;

            if let EndianType::BigEndian = self.endian {
                let result = swap_i16(&result);
                Ok(result)
            } else {
                Ok(result)
            }
        } else {
            panic!(format!(
                "read length error for i16,need length 2,read length {}",
                r
            ))
        }
    }
    ///
    ///read u16 from stream,return it value
    ///
    pub fn read_u16(&mut self) -> Result<u16, StreamError> {
        let mut data: [u8; 2] = [0; 2];
        let r = self.read(&mut data, 0, 2)?;

        if r == 2 {
            let result: u16 = (data[0] as u16 | ((data[1] as u16) << 8)) as u16;
            if let EndianType::BigEndian = self.endian {
                let result = swap_u16(&result);
                Ok(result)
            } else {
                Ok(result)
            }
        } else {
            panic!(format!(
                "read length error for u16,need length 2,read length {}",
                r
            ))
        }
    }

    ///
    ///read u32 from stream,return it value
    ///
    pub fn read_u32(&mut self) -> Result<u32, StreamError> {
        let mut data: [u8; 4] = [0; 4];
        let r = self.read(&mut data, 0, 4)?;

        if r == 4 {
            unsafe {
                let result: u32 = *(data.as_ptr() as *const u32);
                if let EndianType::BigEndian = self.endian {
                    let result = swap_u32(&result);
                    Ok(result)
                } else {
                    Ok(result)
                }
            }
        } else {
            panic!(format!(
                "read length error for u32,need length 4,read length {}",
                r
            ))
        }
    }

    ///
    /// read i32 from stream,return it value
    ///
    pub fn read_i32(&mut self) -> Result<i32, StreamError> {
        let mut data: [u8; 4] = [0; 4];
        let r = self.read(&mut data, 0, 4)?;

        if r == 4 {
            unsafe {
                let result: i32 = *(data.as_ptr() as *const i32);
                if let EndianType::BigEndian = self.endian {
                    let result = swap_i32(&result);
                    Ok(result)
                } else {
                    Ok(result)
                }
            }
        } else {
            panic!(format!(
                "read length error for i32,need length 4,read length {}",
                r
            ))
        }
    }

    ///
    /// read i64 from stream,return it value
    ///
    pub fn read_i64(&mut self) -> Result<i64, StreamError> {
        let mut data: [u8; 8] = [0; 8];
        let r = self.read(&mut data, 0, 8)?;

        if r == 8 {
            unsafe {
                let result: i64 = *(data.as_ptr() as *const i64);
                if let EndianType::BigEndian = self.endian {
                    let result = swap_i64(&result);
                    Ok(result)
                } else {
                    Ok(result)
                }
            }
        } else {
            panic!(format!(
                "read length error for i64,need length 8,read length {}",
                r
            ))
        }
    }

    ///
    ///read u64 from stream,return it value
    ///
    pub fn read_u64(&mut self) -> Result<u64, StreamError> {
        let mut data: [u8; 8] = [0; 8];
        let r = self.read(&mut data, 0, 8)?;

        if r == 8 {
            unsafe {
                let result: u64 = *(data.as_ptr() as *const u64);
                if let EndianType::BigEndian = self.endian {
                    let result = swap_u64(&result);
                    Ok(result)
                } else {
                    Ok(result)
                }
            }
        } else {
            panic!(format!(
                "read length error for u64,need length 8,read length {}",
                r
            ))
        }
    }

    ///
    ///read f32 from stream,return it value
    ///
    pub fn read_single(&mut self) -> Result<f32, StreamError> {
        let mut data: [u8; 4] = [0; 4];
        let r = self.read(&mut data, 0, 4)?;

        if r == 4 {
            unsafe {
                let result: f32 = *(data.as_ptr() as *const f32);
                Ok(result)
            }
        } else {
            panic!(format!(
                "read length error for f32,need length 4,read length {}",
                r
            ))
        }
    }

    ///
    ///read f64 from stream,return it value
    ///
    pub fn read_double(&mut self) -> Result<f64, StreamError> {
        let mut data: [u8; 8] = [0; 8];
        let r = self.read(&mut data, 0, 8)?;

        if r == 8 {
            unsafe {
                let result: f64 = *(data.as_ptr() as *const f64);
                Ok(result)
            }
        } else {
            panic!(format!(
                "read length error for f64,need length 8,read length {}",
                r
            ))
        }
    }

    ///
    ///  read vec<u8> from the current stream,can specify need read length to buffer
    /// # Examples
    ///```rust
    ///    use iostream::io::*;
    ///    let mut ms=MemoryStream::new();
    ///    let data:[u8;1024]=[5;1024];
    ///    ms.write_all(&data).unwrap();
    ///
    ///    ms.set_position(0).unwrap();
    ///
    ///    let mut rs=StreamReader::from(&mut ms).unwrap();
    ///    let rt= rs.read_u8_array(1024).unwrap();
    ///    assert_eq!(data[..],rt[..] );
    /// ```
    pub fn read_u8_array(&mut self, len: u32) -> Result<Vec<u8>, StreamError> {
        let mut array: Vec<u8> = Vec::new();
        for _i in 0..len {
            let v = self.read_byte();

            match v {
                Ok(i) => {
                    array.push(i);
                }
                Err(e) => {
                    if let StreamErrorKind::End = e.err_type {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Ok(array)
    }

    fn read7bit_encode_to_int(&mut self) -> Result<u32, String> {
        let mut i = 0;
        let mut a1: u32 = 0;
        let mut a2: u32 = 0;

        loop {
            if a1 == 0x23 {
                return Err("bad 7 bit i32".to_string());
            }
            let r = self.read_byte();
            if let Ok(v) = r {
                i = v;
            }

            a1 |= ((i & 0x7f) << a2 as u8) as u32;
            a2 += 7;

            if i & 0x80 == 0 {
                break;
            }
        }

        Ok(a1)
    }

    ///
    /// read string to current stream
    ///
    pub fn read_string(&mut self) -> Result<String, Box<dyn Error>> {
        let capacity: u32 = self.read7bit_encode_to_int()?;
        let data = self.read_u8_array(capacity)?;
        let str = String::from_utf8_lossy(&data);
        Ok(str.to_string())
    }

    ///
    ///  read str line from current stream
    ///
    pub fn read_line(&mut self) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<u8> = Vec::new();

        loop {
            let rt = self.read_byte();

            match rt {
                Ok(i) => {
                    data.push(i);

                    if i == '\r' as u8 {
                        let r = self.peek();
                        if let Ok(i) = r {
                            if i == '\n' as u8 {
                                //if windows \r\n
                                data.push(i);
                                let position = self.base_stream.position();
                                let r = self.base_stream.set_position(position + 1);
                                if let Err(e) = r {
                                    StreamError::from_str(&format!("{}", e))?
                                }
                            }
                        }

                        break;
                    } else if i == '\n' as u8 {
                        break;
                    }

                    continue;
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

        return Ok(String::from_utf8_lossy(&data).to_string());
    }

    ///
    ///  read all str line from current stream
    ///
    pub fn read_all_lines(&mut self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut list: Vec<String> = Vec::new();

        loop {
            let rt = self.peek();

            if let Err(e) = rt {
                if let StreamErrorKind::End = e.err_type {
                    break;
                } else {
                    return Err(Box::new(e));
                }
            } else if let Ok(_i) = rt {
                let str = self.read_line()?;
                list.push(str);
            }
        }

        return Ok(list);
    }

    ///
    ///  read all data to string
    ///
    pub fn read_all_text(&mut self) -> Result<String, Box<dyn Error>> {
        let mut data: Vec<u8> = Vec::new();
        let size = self.read_all(&mut data)?;

        if size > 0 {
            Ok(String::from_utf8_lossy(&data).to_string())
        } else {
            Err(Box::new(StreamError::new_bad()))
        }
    }
}
