use super::{Stream,SeekOrigin};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::ErrorKind;
use std::fs::remove_file;
use std::io::Write;
use std::io::Read;
use std::io;
use std::path::*;
use std::fs::rename;
use std::fs::copy;



#[derive(Debug)]

/// file stream create mode
pub enum FileMode{

    ///     Specifies that the operating system should create a new file. This requires System.Security.Permissions.FileIOPermissionAccess.Write
    ///     permission. If the file already exists, an System.IO.IOException exception is
    ///     thrown.
    CreateNew = 1,

    ///
    ///     Specifies that the operating system should create a new file. If the file already
    ///     exists, it will be overwritten. This requires System.Security.Permissions.FileIOPermissionAccess.Write
    ///     permission. FileMode.Create is equivalent to requesting that if the file does
    ///     not exist, use System.IO.FileMode.CreateNew; otherwise, use System.IO.FileMode.Truncate.
    ///     If the file already exists but is a hidden file, an System.UnauthorizedAccessException
    ///     exception is thrown.
    Create = 2,

    ///
    ///     Specifies that the operating system should open an existing file. The ability
    ///     to open the file is dependent on the value specified by the System.IO.FileAccess
    ///     enumeration. A System.IO.FileNotFoundException exception is thrown if the file
    ///     does not exist.
    Open = 3,

    ///
    ///     Specifies that the operating system should open a file if it exists; otherwise,
    ///     a new file should be created. If the file is opened with FileAccess.Read, System.Security.Permissions.FileIOPermissionAccess.Read
    ///     permission is required. If the file access is FileAccess.Write, System.Security.Permissions.FileIOPermissionAccess.Write
    ///     permission is required. If the file is opened with FileAccess.ReadWrite, both
    ///     System.Security.Permissions.FileIOPermissionAccess.Read and System.Security.Permissions.FileIOPermissionAccess.Write
    ///     permissions are required.
    OpenOrCreate = 4,

    ///
    ///     Specifies that the operating system should open an existing file. When the file
    ///     is opened, it should be truncated so that its size is zero bytes. This requires
    ///     System.Security.Permissions.FileIOPermissionAccess.Write permission. Attempts
    ///     to read from a file opened with FileMode.Truncate cause an System.ArgumentException
    ///     exception.
    Truncate = 5,

    ///
    ///     Opens the file if it exists and seeks to the end of the file, or creates a new
    ///     file. This requires System.Security.Permissions.FileIOPermissionAccess.Append
    ///     permission. FileMode.Append can be used only in conjunction with FileAccess.Write.
    ///     Trying to seek to a position before the end of the file throws an System.IO.IOException
    ///     exception, and any attempt to read fails and throws a System.NotSupportedException
    ///     exception.
    Append = 6
}

///  Defines constants for read, write, or read/write access to a file.
pub enum FileAccess {
    ///
    ///
    ///     Read access to the file. Data can be read from the file. Combine with Write for
    ///     read/write access.
    Read = 1,
    ///
    ///
    ///     Write access to the file. Data can be written to the file. Combine with Read
    ///     for read/write access.
    Write = 2,
    ///
    ///
    ///     Read and write access to the file. Data can be written to and read from the file.
    ReadWrite = 3
}


/// the file stream from disk file,you can use it, wr file
pub struct FileStream {
    pub file:File,
    position:u64,
    length:u64,
    can_write:bool,
    can_read:bool
}


impl FileStream{
    /// create filestream, path is file path, mode is open mode, access is wr mode
    /// # Examples
    ///```rust
    /// use iostream::io::*;
    /// use std::fs::*;
    /// let mut fs=FileStream::new("c.data",FileMode::CreateNew,FileAccess::ReadWrite).unwrap();
    ///```
   pub fn new(path:&str,mode:FileMode,access:FileAccess)->Result<FileStream,String>{

        let mut _res;
        let mut _file:File;
        let mut _position:u64;
        let mut _can_write=true;
        let mut _can_read=true;
        match access {
            FileAccess::Read=>
                {
                    _can_write=false;
                    _res= OpenOptions::new().read(true).write(false).open(path);

                   match mode {
                       FileMode::Open => {
                           match _res {
                               Ok(file) => {
                                   _file = file;
                                   let rt = _file.seek(SeekFrom::Start(0));

                                   match rt {
                                       Ok(i)=>_position=i,
                                       Err(e)=>return Err(format!("path:{0} open read seek err:{1}", path, e))
                                   }
                               },
                               Err(e) => return Err(format!("path:{0} open read err:{1}", path, e))
                           }
                       },

                       _ => { return Err(format!("{:?} with FileAccess: Read is invalid", mode)) }
                   }

                },
            FileAccess::ReadWrite=>
                {
                    match mode {
                        FileMode::Create =>
                            {
                                _res = OpenOptions::new().create(true).write(true).read(true).open(path);

                                match _res {
                                    Ok(file)=> {
                                        _file = file;
                                        let rt = _file.seek(SeekFrom::Start(0));

                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=>return Err(format!("path:{0} Create ReadWrite seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=> return Err(format!("path:{0} Create ReadWrite err:{1}", path, e))

                                }

                            },
                        FileMode::CreateNew =>
                            {

                                _res = OpenOptions::new().create_new(true).write(true).read(true).open(path);

                                match _res {
                                    Ok(file)=>{
                                        _file = file;
                                        let rt=_file.seek(SeekFrom::Start(0));

                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} CreateNew ReadWrite seek err:{1}", path, e))
                                        }

                                    },
                                    Err(e)=>{
                                        if e.kind()==ErrorKind::AlreadyExists{
                                            let rf= remove_file(path);
                                            if let Err(e)=rf{
                                                return Err(format!("path:{0} CreateNew  ReadWrite remove_file err:{1}", path, e))
                                            }
                                            else {
                                                _res = OpenOptions::new().create_new(true).write(true).read(true).open(path);

                                                match _res {
                                                    Ok(file)=> {
                                                        _file = file;
                                                        let rt = _file.seek(SeekFrom::Start(0));

                                                        match rt {
                                                            Ok(i)=>_position=i,
                                                            Err(e)=>return Err(format!("path:{0} CreateNew Create ReadWrite seek err:{1}", path, e))
                                                        }

                                                    },
                                                    Err(e)=> return Err(format!("path:{0} CreateNew Create ReadWrite err:{1}", path, e))

                                                }
                                            }
                                        }
                                        else{
                                            return Err(format!("path:{0} CreateNew  ReadWrite err:{1}", path, e))
                                        }
                                    }

                                }
                            },
                        FileMode::OpenOrCreate =>
                            {
                                _res = OpenOptions::new().write(true).read(true).open(path);

                                match _res {
                                    Ok(file) => {
                                        _file = file;
                                        let rt= _file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} OpenOrCreate ReadWrite seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e) => {
                                        if e.kind() == ErrorKind::NotFound {
                                            _res = OpenOptions::new().create(true).write(true).read(true).open(path);

                                            match _res {
                                                Ok(file) => {
                                                    _file = file;
                                                   let rt=  _file.seek(SeekFrom::Start(0));

                                                    match rt {
                                                        Ok(i)=>_position=i,
                                                        Err(e)=> return Err(format!("path:{0} OpenOrCreate ReadWrite create seek err:{1}", path, e))
                                                    }

                                                },
                                                Err(e) => {
                                                    return Err(format!("path:{0} OpenOrCreate ReadWrite create err:{1}", path, e))
                                                }
                                            }
                                        } else {
                                            return Err(format!("path:{0} OpenOrCreate ReadWrite err:{1}", path, e))
                                        }
                                    }
                                }
                            }
                        FileMode::Open =>
                            {
                                _res = OpenOptions::new().write(true).read(true).open(path);
                                match _res {
                                    Ok(file) => {
                                        _file = file;
                                        let rt = _file.seek(SeekFrom::Start(0));

                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=>  return Err(format!("path:{0} Open ReadWrite seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e) => {
                                        return Err(format!("path:{0} Open ReadWrite err:{1}", path, e))
                                    }
                                }
                            },
                        FileMode::Append =>
                            {
                                _res = OpenOptions::new().write(true).read(true).open(path);

                                match _res {
                                    Ok(file)=> {
                                        _file = file;
                                        let rt = _file.seek(SeekFrom::End(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} Append ReadWrite seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=>{

                                        if e.kind() == ErrorKind::NotFound {
                                            _res = OpenOptions::new().create(true).write(true).read(true).open(path);

                                            match _res {
                                                Ok(file) => {
                                                    _file = file;
                                                    let rt=  _file.seek(SeekFrom::End(0));

                                                    match rt {
                                                        Ok(i)=>_position=i,
                                                        Err(e)=> return Err(format!("path:{0} Append ReadWrite create seek err:{1}", path, e))
                                                    }

                                                },
                                                Err(e) => {
                                                    return Err(format!("path:{0} Append ReadWrite create err:{1}", path, e))
                                                }
                                            }
                                        } else {
                                            return Err(format!("path:{0} Append ReadWrite err:{1}", path, e))
                                        }


                                    }
                                }
                            },
                        FileMode::Truncate =>
                            {
                                _res = OpenOptions::new().truncate(true).write(true).read(true).open(path);

                                match _res {
                                    Ok(file)=>{
                                        _file = file;
                                        let rt= _file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} Truncate ReadWrite seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=>{
                                        return Err(format!("path:{0} Truncate ReadWrite err:{1}", path, e))
                                    }
                                }

                            }
                    }

                },
            FileAccess::Write=>
                {
                    _can_read=false;
                    match mode {
                        FileMode::Create =>
                            {
                                _res = OpenOptions::new().create(true).write(true).read(false).open(path);
                                match _res {
                                    Ok(file) => {
                                        _file = file;
                                        let rt = _file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} Create Write seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e) => {
                                        return Err(format!("path:{0} Create Write err:{1}", path, e))
                                    }
                                }
                            },
                        FileMode::CreateNew =>
                            {
                                _res = OpenOptions::new().create_new(true).write(true).read(false).open(path);

                                match _res {
                                    Ok(file)=> {
                                        _file = file;
                                        let rt = _file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} CreateNew Write seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=> {
                                        if e.kind() == ErrorKind::AlreadyExists {
                                            let rf = remove_file(path);
                                            if let Err(e) = rf {
                                                return Err(format!("path:{0} CreateNew  Write remove_file err:{1}", path, e))
                                            } else {
                                                _res = OpenOptions::new().create_new(true).write(true).read(true).open(path);
                                                match _res {
                                                    Ok(file) => {
                                                        _file = file;
                                                        let rt = _file.seek(SeekFrom::Start(0));
                                                        match rt {
                                                            Ok(i)=>_position=i,
                                                            Err(e)=> return Err(format!("path:{0} CreateNew Create Write seek err:{1}", path, e))
                                                        }
                                                    },
                                                    Err(e) => return Err(format!("path:{0} CreateNew Create Write err:{1}", path, e))
                                                }
                                            }
                                        } else {
                                            return Err(format!("path:{0} CreateNew  Write err:{1}", path, e))
                                        }
                                    }
                                }

                            },
                        FileMode::OpenOrCreate =>
                            {
                                _res = OpenOptions::new().write(true).read(false).open(path);
                                match _res {
                                    Ok(file)=>{
                                        _file = file;
                                        let rt= _file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} OpenOrCreate Write seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=>{
                                        if e.kind() == ErrorKind::NotFound {
                                            _res = OpenOptions::new().create(true).write(true).read(true).open(path);

                                            match _res {
                                                Ok(file)=> {
                                                    _file = file;
                                                    let rt = _file.seek(SeekFrom::Start(0));
                                                    match rt {
                                                        Ok(i)=>_position=i,
                                                        Err(e)=>  return Err(format!("path:{0} OpenOrCreate Write create seek err:{1}", path, e))
                                                    }
                                                },
                                                Err(e)=>{
                                                    return Err(format!("path:{0} OpenOrCreate Write err:{1}", path, e))
                                                }

                                            }
                                        }else {
                                            return Err(format!("path:{0} OpenOrCreate Write err:{1}", path, e))
                                        }
                                    }

                                }

                            }
                        FileMode::Open =>
                            {
                                _res = OpenOptions::new().write(true).read(false).open(path);

                                match _res {
                                    Ok(file)=>{
                                        _file = file;
                                        let rt= _file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} Open Write seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=>{
                                        return Err(format!("path:{0} Open Write err:{1}", path, e))
                                    }
                                }

                            },
                        FileMode::Append =>
                            {
                                _res = OpenOptions::new().write(true).read(false).open(path);

                                match _res {
                                    Ok(file) => {
                                        _file = file;
                                        let rt= _file.seek(SeekFrom::End(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} Append Write seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e) => {
                                        if e.kind() == ErrorKind::NotFound {
                                            _res = OpenOptions::new().create(true).write(true).open(path);

                                            match _res {
                                                Ok(file)=> {
                                                    _file = file;
                                                    let rt = _file.seek(SeekFrom::End(0));
                                                    match rt {
                                                        Ok(i)=>_position=i,
                                                        Err(e)=>  return Err(format!("path:{0} Append Write create seek err:{1}", path, e))
                                                    }
                                                },
                                                Err(e)=>{
                                                    return Err(format!("path:{0} Append Write err:{1}", path, e))
                                                }

                                            }
                                        }else {
                                            return Err(format!("path:{0} Append Write err:{1}", path, e))
                                        }
                                    }
                                }
                            },
                        FileMode::Truncate =>
                            {
                                _res = OpenOptions::new().truncate(true).write(true).read(false).open(path);

                                match _res {
                                    Ok(file)=>{
                                        _file = file;
                                        let rt=_file.seek(SeekFrom::Start(0));
                                        match rt {
                                            Ok(i)=>_position=i,
                                            Err(e)=> return Err(format!("path:{0} Truncate Write seek err:{1}", path, e))
                                        }
                                    },
                                    Err(e)=>{
                                        return Err(format!("path:{0} Truncate Write err:{1}", path, e))
                                    }
                                }
                            }
                    }
                }

        }


        let _length:u64;


        let rt=_file.seek(SeekFrom::End(0));
        match rt {
            Ok(i)=>_length=i,
            Err(e)=> return Err(format!("path:{0} not get length {1}", path, e))
        }

        let rt=_file.seek(SeekFrom::Start(_position));
        match rt {
            Ok(i)=>_position=i,
            Err(e)=> return Err(format!("path:{0} not re set position {1}", path, e))
        }




        Ok(FileStream{file:_file, position: _position ,length:_length,can_write:_can_write,can_read:_can_read})
    }
}

impl Stream for FileStream{



    fn set_position(&mut self, _potion: u64) ->Result<(),String>{
        let r= self.file.seek(SeekFrom::Start(_potion));

        match r {
            Ok(p)=> {
                self.position = p;
                Ok(())
            },
            Err(e)=>
                {
                    Err(format!("Err:{:#?}",e))
                }

        }

    }

    fn position(&mut self) -> u64 {
        self.position
    }

    fn length(&mut self) -> u64 {
        self.length
    }

    fn set_length(&mut self, length: u64)->Result<(),String> {

        let r=  self.file.set_len(length);

        match r {
            Ok(())=>{

                let r=self.file.seek(SeekFrom::Current(0));

                match r {
                    Ok(p)=>{
                        self.position=p;
                        self.length=length;
                        return Ok(());

                    },
                    Err(e)=>Err(format!("Err:{:#?}",e))
                }
            },
            Err(e)=>Err(format!("Err:{:#?}",e))

        }



    }


    fn can_write(&mut self) -> bool {
        self.can_write
    }

    fn can_read(&mut self) -> bool {
       self.can_read
    }

    fn can_seek(&mut self) -> bool {
       true
    }

    fn read_all(&mut self, buf: &mut Vec<u8>) -> Result<usize,String> {

        loop {
            let rt=  self.read_byte();


            match rt {
                Ok(i)=>buf.push(i),
                Err(e)=>
                    {
                        if e=="End"{
                            break;
                        }else {
                            return Err(e)
                        }
                    }
            }

        }

        Ok(buf.len())
    }

    fn read(&mut self, buf: &mut [u8], offset: usize, count: usize) -> Result<usize,String> {
        let mut _offset = offset;
        let end =_offset+count;

        if end > buf.len() {
            panic!("end Greater than equal to or equal to buf length ")
        }

        let x = &mut buf[offset..end];
        let r= self.file.read(x);

        match r {
            Ok(size)=>
                {
                    self.position += size as u64;
                    Ok(size)
                },
            Err(e)=>Err(format!("Err:{:#?}",e))
        }


    }

    fn read_byte(&mut self) ->Result<u8,String> {
        let mut i: [u8; 1] = [0; 1];
        let rt = self.file.read(&mut i);

        match rt {
            Ok(size)=>
                {
                    if size == 1 {
                        self.position += 1;
                        return Ok(i[0])
                    } else {
                        Err("End".to_string())
                    }
                },
            Err(e)=>
                {
                    Err(format!("{}",e))
                }
        }
    }

    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<u64,String>  {

        match origin {

            SeekOrigin::Current=>
                {

                    let r= self.file.seek(SeekFrom::Current(offset));
                    match r {
                        Ok(_potion)=>
                            {
                                self.position=_potion;
                                Ok(_potion)
                            },
                        Err(e)=>Err(format!("Err:{:#?}",e))
                    }

                },
            SeekOrigin::Begin=>
                {
                    let r=self.file.seek(SeekFrom::Start(offset as u64));
                    match r {
                        Ok(_potion)=>
                            {
                                self.position=_potion;
                                Ok(_potion)
                            },
                        Err(e)=>Err(format!("Err:{:#?}",e))
                    }

                },
            SeekOrigin::End=>
                {

                    let r=self.file.seek(SeekFrom::End(offset));
                    match r {
                        Ok(_potion)=>
                            {
                                self.position=_potion;
                                Ok(_potion)
                            },
                        Err(e)=>Err(format!("Err:{:#?}",e))
                    }
                }
        }
    }

    fn write_all(&mut self, buf:&[u8])->Result<(),String> {
       let r=  self.file.write_all(buf);
        if let Err(e)=r{
            return Err(format!("Err:{:#?}",e))
        }
        self.position+=buf.len() as u64;
        self.length+=buf.len() as u64;
        Ok(())
    }

    fn write(&mut self, buf:&[u8], offset: usize, count: usize)->Result<usize,String> {

        let mut _offset = offset;
        let end =_offset+count;

        if end > buf.len() {
            return  Err(format!("offset+count greater than equal to or equal to buf length,\n buf length: {0}\n offset+count length： {1}",buf.len(),end))
        }
        let x = &buf[offset..end];
        let r= self.file.write(x);
        

        match r {
            Ok(len)=>
                {
                    self.position+=len as u64;
                    self.length+=len as u64;
                    Ok(len)
                },
            Err(e)=>Err(format!("Err:{:#?}",e))
        }

    }

    fn flush(&mut self)->Result<(),String> {

        let r=self.file.flush();
        if let Err(e)=r{
            return Err(format!("{:#?}",e))
        }
        Ok(())
    }
}

///
/// from std::fs::File open file stream
///
pub trait FsOption {
    ///
    /// open filestream from path
    ///
    fn open_fs(path:&str,mode:FileMode,access:FileAccess)->Result<FileStream,String>;
    ///
    ///  read file all content to string
    ///
    fn read_all_text(path:&str)->Result<String,String>;

    ///
    /// read all line from file
    ///
    fn read_all_lines(path:&str)->Result<Vec<String>,String>;
    ///
    ///  write all string to path file
    ///
    fn write_all_text(path:&str,text:&str) -> Result<(), String>;
    ///
    ///  Append the all row to the path file. If it is not found, it will be created.
    ///
    fn append_all_text(path:&str,text:&str)->Result<(), String>;

    ///
    /// Append the row to the path file. If it is not found, it will be created.
    ///
    fn append_line(path:&str,text:&str)->Result<(), String>;

    ///
    ///  append the all line to the path file
    ///
    fn append_all_lines(path:&str,lines:&[&str])->Result<(), String>;

    ///
    /// delete file from path
    ///
    fn delete(path:&str)-> io::Result<()>;
    ///
    /// exists file
    ///
    fn is_exists(path:&str)->bool;

    ///
    /// re file name
    ///
    fn rename(source:&str,target:&str)->bool;

    ///
    /// copy to file
    ///
    fn copy_to(from:&str,to:&str)->io::Result<u64>;
}

/// use std::fs::File
///
/// # Examples
///```
/// extern  crate iostream;
/// use iostream::io::*;
/// use std::fs::*;
///
/// let mut fs= File::open_fs("7.data", FileMode::CreateNew, FileAccess::ReadWrite).unwrap();
///```
impl FsOption for File{

    fn open_fs(path: &str, mode: FileMode, access: FileAccess) ->Result<FileStream,String> {
        FileStream::new(path,mode,access)
    }

    fn read_all_text(path: &str) -> Result<String, String> {
        let mut fs = File::open_fs(path, FileMode::Open, FileAccess::Read)?;
        let mut rs = super::StreamReader::from(&mut fs)?;
        rs.read_all_text()
    }

    fn read_all_lines(path:&str)->Result<Vec<String>,String>{
        let mut fs = File::open_fs(path, FileMode::Open, FileAccess::Read)?;
        let mut rs = super::StreamReader::from(&mut fs)?;
        rs.read_all_lines()
    }

    fn write_all_text(path:&str,text:&str) -> Result<(), String> {
        let mut fs = File::open_fs(path, FileMode::OpenOrCreate, FileAccess::Write)?;
        let mut ws=super::StreamWriter::from(&mut fs)?;
        ws.write_all(text.as_bytes())
    }

    fn append_all_text(path:&str,text:&str)->Result<(), String>{
        let mut fs = File::open_fs(path, FileMode::Append, FileAccess::Write)?;
        let mut ws=super::StreamWriter::from(&mut fs)?;
        ws.write_all(text.as_bytes())
    }

    fn append_line(path:&str,text:&str)->Result<(), String>{
        let mut fs = File::open_fs(path, FileMode::Append, FileAccess::Write)?;
        let mut ws=super::StreamWriter::from(&mut fs)?;
        ws.write_line(text)
    }

    fn append_all_lines(path:&str,lines:&[&str])->Result<(), String>{
        let mut fs = File::open_fs(path, FileMode::Append, FileAccess::Write)?;
        let mut ws=super::StreamWriter::from(&mut fs)?;
        ws.write_all_lines(lines)
    }

    fn delete(path:&str)-> io::Result<()>{
        remove_file(path)
    }

    fn is_exists(path:&str)->bool{

       let path=  Path::new(path);

       if path.exists(){
           if path.is_file(){
               return true;
           }
       }
        false
    }

    fn rename(from:&str,to:&str)->bool {
        let rt = rename(from, to);

        match rt {
            Ok(())=> return true,
            Err(_e)=>return false
        }
    }

    fn copy_to(from:&str,to:&str)->io::Result<u64>{
       copy(from,to)
    }

}