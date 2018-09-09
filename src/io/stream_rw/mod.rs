pub mod stream_reader;
pub mod stream_writer;

pub enum  EndianType{

    LittleEndian,
    BigEndian
}


fn swap_i16(i:&i16)->i16{
    let ff:i16=0xff;
    (i&ff)<<8|((i>>8)&ff)
}

fn swap_i32(i:&i32)->i32{
    let ff:i32=0xffff;
    let v=*i as i16;
    let c=(i>>0x10) as i16;

    let d1=  swap_i16(&v) as i32;
    let d2=swap_i16(&c) as i32;
    ((d1&ff)<<0x10)|(d2&ff)
}

fn swap_i64(i:&i64)->i64{
    let ff:i64= 0xffffffff;
    let v=*i as i32;
    let c=(i>>0x20) as i32;

    let d1=  swap_i32(&v) as i64;
    let d2=swap_i32(&c) as i64;
    ((d1&ff)<<0x20)|(d2&ff)
}

fn swap_u16(i:&u16)->u16{
    let ff:u16=0xff;
    (i&ff)<<8|((i>>8)&ff)
}

fn swap_u32(i:&u32)->u32{
    let ff:u32=0xffff;
    let v=*i as u16;
    let c=(i>>0x10) as u16;

    let d1=  swap_u16(&v) as u32;
    let d2=swap_u16(&c) as u32;
    ((d1&ff)<<0x10)|(d2&ff)
}

fn swap_u64(i:&u64)->u64{
    let ff:u64= 0xffffffff;
    let v=*i as u32;
    let c=(i>>0x20) as u32;

    let d1=  swap_u32(&v) as u64;
    let d2=swap_u32(&c) as u64;
    ((d1&ff)<<0x20)|(d2&ff)
}