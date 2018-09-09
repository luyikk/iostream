use super::super::Stream;
extern crate core;
use io::SeekOrigin;
use self::core::ptr::copy_nonoverlapping;

pub struct MemoryStream{
    data:Vec<u8>,
    position:u64,
}

impl MemoryStream {
    pub fn new()->MemoryStream{

        MemoryStream{
            data:Vec::new(),
            position:0
        }
    }

    pub fn new_to(buf:&[u8])->MemoryStream{

       let m= if buf.len()>0{
           MemoryStream{
               data:Vec::from(buf),
               position:0
           }
       }else {
           MemoryStream{
               data:Vec::new(),
               position:0
           }
       };

       m
    }
}

impl MemoryStream{
    ///  return current stream data to Vec<u8>
    pub fn to_vec(&self)->Vec<u8>{
        self.data.clone()
    }

    ///Clears the vector, removing all values,and set position =0
    pub fn clear(&mut self){
        self.data.clear();
        self.position =0;
    }
}

impl Stream for MemoryStream {

    fn set_position(&mut self, _potion: u64) -> Result<(), String> {
        if _potion > self.data.len() as u64 {
            return Err(format!("set position value greater then stream length,please use Seek"));
        }
        self.position = _potion;
        Ok(())
    }

    fn position(&mut self) -> u64 {
        self.position
    }

    fn length(&mut self) -> u64 {
        self.data.len() as u64
    }

    fn set_length(&mut self, len: u64) -> Result<(), String> {
        if len < self.data.len() as u64 {
            self.position = len;
        }

        self.data.resize(len as usize, 0);

        Ok(())
    }

    fn can_write(&mut self) -> bool {
        true
    }

    fn can_read(&mut self) -> bool {
        true
    }

    fn can_seek(&mut self) -> bool {
        true
    }

    ///```rust
    ///    use iostream::io::*;
    ///
    ///    let mut ms=MemoryStream::new();
    ///    let data:[u8;1024]=[55;1024];
    ///    ms.write_all(&data).unwrap();
    ///
    ///    ms.set_position(0).unwrap();
    ///
    ///    let mut p:Vec<u8>=Vec::new();
    ///    ms.read_all(&mut p).unwrap();
    ///
    ///    for i  in p {
    ///    print!("{}",i);
    ///    }
    ///
    ///    println!();
    ///
    ///    ms.set_position(0).unwrap();
    ///    let mut p:Vec<u8>=vec![0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff];
    ///    ms.read_all(&mut p).unwrap();
    ///
    ///    for i  in p {
    ///    print!("{}",i);
    ///    }
    ///    println!();
    /// ```
    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize, String> {

        let _potion=self.position as usize;
        let len=self.data.len();

        if _potion>=len{
            return Err("End".to_string())
        }

        let need_cp_num=len-_potion;

        let bk = buf.len();
        buf.resize(bk + need_cp_num, 0);

        let sbuf = &mut buf[bk..];
        sbuf.copy_from_slice(&self.data[_potion..]);

        self.position +=need_cp_num as u64;
        Ok(need_cp_num)
    }

    ///```rust
    ///    use iostream::io::*;
    ///    let mut ms=MemoryStream::new();
    ///    let data:[u8;1024]=[55;1024];
    ///    ms.write_all(&data).unwrap();
    ///
    ///    ms.set_position(0).unwrap();
    ///
    ///    let mut data:Vec<u8>=vec![0;250];
    ///    let len=data.len();
    ///    let size = ms.read(&mut data, 0,len).unwrap();
    ///```
    fn read(&mut self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize, String> {
        let mut _offset = offset;
        let end = _offset + count;
        if end > buf.len() {
           return   Err(format!("offset+count greater than equal to or equal to buf length,\n buf length: {0}\n offset+count length： {1}",buf.len(),end))
        }

        if self.position >= self.data.len() as u64 {
            return Err("End".to_string())
        }

        let mut rcount=self.data.len()-self.position as usize;

        if rcount>count{
            rcount=count;
        }

        let targetbuf = &mut buf[offset..end];
        let source_buf = &self.data[self.position as usize..];

        unsafe {
            copy_nonoverlapping(source_buf.as_ptr(), targetbuf.as_mut_ptr(), rcount);
            self.position += rcount as u64;
            Ok(rcount)
        }
    }

    /// ```rust
    ///    use iostream::io::*;
    ///
    ///    let data:[u8;10]=[5;10];
    ///    let mut ms= MemoryStream::new_to(&data);
    ///
    ///    for s in 0..10 {
    ///    let d=  ms.read_byte().unwrap();
    ///    print!("{}",d);
    ///    }
    /// ```
    fn read_byte(&mut self) -> Result<u8, String> {
        if self.position < self.data.len() as u64 {
            let p = self.data.get(self.position as usize);
            match p {
                Some(i) =>
                    {
                        let x: u8 = i.clone();
                        self.position += 1;
                        Ok(x)
                    },
                None =>
                    {
                        return Err("End".to_string())
                    }
            }
        } else {
            Err("End".to_string())
        }
    }


    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<u64, String> {
        match origin {
            SeekOrigin::Begin =>
                {
                    self.position = offset as u64;

                    if self.position > self.data.len() as u64 {
                        self.data.resize(self.position as usize, 0);
                    }

                    return Ok(self.position)
                },
            SeekOrigin::End =>
                {
                    if offset == 0 {
                        self.position = self.data.len() as u64;
                        return Ok(self.position)
                    } else {
                        let p = (self.data.len() as i64 + offset) as u64;
                        self.position = p;
                        if self.position > self.data.len() as u64 {
                            self.data.resize(self.position as usize, 0);
                        }
                        return Ok(self.position)
                    }
                },
            SeekOrigin::Current => {
                let p = (self.position as i64 + offset) as u64;
                self.position = p;
                if self.position > self.data.len() as u64 {
                    self.data.resize(self.position as usize, 0);
                }
                return Ok(self.position)
            }
        }
    }

    ///```rust
    ///use iostream::io::*;
    ///
    ///let mut ms= MemoryStream::new();
    ///let data: [u8; 1024] = [2; 1024];
    ///ms.write_all(&data).unwrap();
    ///```
    fn write_all(&mut self, buf: &[u8]) -> Result<(), String> {
        unsafe {
            let current_index = self.position as usize;
            let source_len = self.data.len();
            let write_len = buf.len();
            let have_len = self.data.len() - current_index;
            let need_add = write_len - have_len;
            if need_add > 0 {
                self.data.resize(source_len + need_add, 0);
            }

            let source_buf = &mut self.data[current_index..];

            copy_nonoverlapping(buf.as_ptr(), source_buf.as_mut_ptr(), write_len);
            self.position += write_len as u64;
            Ok(())
        }
    }


    ///```rust
    ///use iostream::io::*;
    ///let mut ms= MemoryStream::new();
    ///let data2:[u8;100]=[2;100];
    ///ms.write(&data2,50,50).unwrap();
    ///```
    fn write(&mut self, buf: &[u8], offset: usize, count: usize) -> Result<usize, String> {
        let mut _offset = offset;
        let end = _offset + count;

        if end > buf.len() {
            return  Err(format!("offset+count greater than equal to or equal to buf length,\n buf length: {0}\n offset+count length： {1}",buf.len(),end))
        }
        let x = &buf[offset..end];

        let rt = self.write_all(x);

        match rt {
            Ok(()) =>
                {
                    Ok(count)
                },
            Err(e) =>
                {
                    Err(e)
                }
        }
    }

    fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }
}