use super::super::Stream;
use super::*;

///write stream struct
pub struct StreamWriter<'a>{
    pub base_stream:&'a mut Stream,
    endian:EndianType
}

impl<'a>  StreamWriter<'a > {
    /// from stream use LittleEndian write integer
    /// # Examples
    ///```rust
    /// use iostream::io::*;
    /// let mut ms=MemoryStream::new();
    /// //...write data to ms
    /// let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
    /// ```
    pub fn from(stream: &'a mut Stream) -> Result<StreamWriter<'a>,String> {

        if !stream.can_write(){
            return  Err("is stream not can write!".to_string())
        }

       Ok( StreamWriter { base_stream: stream,endian:EndianType::LittleEndian })
    }

    /// from stream use BigEndian write integer
    /// # Examples
    ///```rust
    /// use iostream::io::*;
    /// let mut ms=MemoryStream::new();
    /// //...write data to ms
    /// let mut ws: StreamReader = StreamReader::from_big_endian(&mut ms).unwrap();
    /// ```
    pub fn from_big_endian(stream: &'a mut Stream) -> Result<StreamWriter<'a>,String> {

        if !stream.can_write(){
            return  Err("is stream not can write!".to_string())
        }

        Ok( StreamWriter { base_stream: stream,endian:EndianType::BigEndian })
    }


    ///read [u8] from stream,can use offset and count buff position,return read len
    pub fn write(&mut self,buf:&[u8],offset:usize,count:usize)->Result<usize,String>
    {
        self.base_stream.write(buf,offset,count)
    }

    ///read vec<u8> from stream,return read length
    pub fn write_all(&mut self,buf:&[u8])->Result<(),String>
    {
        self.base_stream.write_all(buf)
    }

    ///write u8 from stream
    pub fn write_byte(&mut self,b:&u8)->Result<(),String>
    {
        let data:[u8;1]=[b.clone();1];

        self.base_stream.write_all(&data)
    }

    ///write i16 from stream,return it value
    pub fn write_i16(&mut self,value:&i16)->Result<(),String>
    {
        let mut data:[u8;2]=[0;2];

        unsafe {
            let x=data.as_mut_ptr() as *mut i16;

            match self.endian {
                EndianType::LittleEndian => {
                    *x = value.clone();
                },
                EndianType::BigEndian => {
                    let v=swap_i16(value);
                    *x=v;
                }
            }


        }

        self.base_stream.write_all(&data)
    }
    ///
    /// write u16 from stream,return it value
    ///
    pub fn write_u16(&mut self,value:&u16)->Result<(),String>
    {
        let mut data:[u8;2]=[0;2];

        unsafe {
            let x=data.as_mut_ptr() as *mut u16;

            match self.endian {
                EndianType::LittleEndian => {
                    *x = value.clone();
                },
                EndianType::BigEndian => {
                    let v=swap_u16(value);
                    *x=v;
                }
            }

        }

        self.base_stream.write_all(&data)
    }
    ///
    /// write i32 from stream,return it value
    ///
    pub fn write_i32(&mut self,value:&i32)->Result<(),String>
    {
        let mut data:[u8;4]=[0;4];

        unsafe {
            let x=data.as_mut_ptr() as *mut i32;
            match self.endian {
                EndianType::LittleEndian => {
                    *x = value.clone();
                },
                EndianType::BigEndian => {
                    let v=swap_i32(value);
                    *x=v;
                }
            }
        }

        self.base_stream.write_all(&data)
    }
    ///
    /// write u32 from stream,return it value
    ///
    pub fn write_u32(&mut self,value:&u32)->Result<(),String>
    {
        let mut data:[u8;4]=[0;4];

        unsafe {
            let x=data.as_mut_ptr() as *mut u32;
            match self.endian {
                EndianType::LittleEndian => {
                    *x = value.clone();
                },
                EndianType::BigEndian => {
                    let v=swap_u32(value);
                    *x=v;
                }
            }
        }

        self.base_stream.write_all(&data)
    }
    ///
    /// write i64 from stream,return it value
    ///
    pub fn write_i64(&mut self,value:&i64)->Result<(),String>
    {
        let mut data:[u8;8]=[0;8];

        unsafe {
            let x=data.as_mut_ptr() as *mut i64;
            match self.endian {
                EndianType::LittleEndian => {
                    *x = value.clone();
                },
                EndianType::BigEndian => {
                    let v=swap_i64(value);
                    *x=v;
                }
            }
        }

        self.base_stream.write_all(&data)
    }
    ///
    /// write u64 from stream,return it value
    ///
    pub fn write_u64(&mut self,value:&u64)->Result<(),String>
    {
        let mut data:[u8;8]=[0;8];

        unsafe {
            let x=data.as_mut_ptr() as *mut u64;
            match self.endian {
                EndianType::LittleEndian => {
                    *x = value.clone();
                },
                EndianType::BigEndian => {
                    let v=swap_u64(value);
                    *x=v;
                }
            }
        }

        self.base_stream.write_all(&data)
    }
    ///
    /// write f32 from stream,return it value
    ///
    pub fn write_single(&mut self,value:&f32)->Result<(),String> {
        let mut data:[u8;4]=[0;4];

        unsafe {
            let x=data.as_mut_ptr() as *mut f32;{
                    *x = value.clone();
            }
        }

        self.base_stream.write_all(&data)
    }

    ///
    /// write f64 from stream,return it value
    ///
    pub fn write_double(&mut self,value:&f64)->Result<(),String>{
        let mut data:[u8;8]=[0;8];

        unsafe {
            let x=data.as_mut_ptr() as *mut f64;{
                *x = value.clone();
            }
        }

        self.base_stream.write_all(&data)
    }

    fn write_7bit_encode_int(&mut self,value:&u32)->Result<(),String>{
        let mut num=value.clone();

        while num>=0x80 {
            self.write_byte(&((num|0x80) as u8))?;
            num=num>>7;
        }

        self.write_byte(&(num as u8))?;

        Ok(())
    }

    ///
    /// write string to current stream
    ///
    pub fn write_string(&mut self,value:&str)->Result<(),String> {
        let data = value.as_bytes();
        self.write_7bit_encode_int(&(data.len() as u32))?;
        self.write_all(data)?;
        Ok(())
    }

    ///
    /// Writes a row string, which adds the ending character '\n'
    ///
    pub fn write_line(&mut self,value:&str)->Result<(),String> {

        let source=value.to_string()+"\n";
        self.write_all(&source.as_bytes())
    }

    ///
    /// writes all row string from str array
    ///
    pub fn write_all_lines(&mut self,value:&[&str])->Result<(),String>{
        for i in value.iter(){
            self.write_line(i)?;
        }
        Ok(())
    }



}