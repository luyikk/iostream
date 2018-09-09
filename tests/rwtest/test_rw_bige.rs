use iostream::io::*;


#[test]
fn test_i16(){

    let mut ms=MemoryStream::new();

    let w1:i16 = -32768;
    let w2:i16=32767;

    {
        let mut ws: StreamWriter = StreamWriter::from_big_endian(&mut ms).unwrap();
        ws.write_i16(&w1).unwrap();
        ws.write_i16(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from_big_endian(&mut ms).unwrap();
    let r1=rs.read_i16().unwrap();
    let r2=rs.read_i16().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}


#[test]
fn test_u16(){

    let mut ms=MemoryStream::new();

    let w1:u16 = 0;
    let w2:u16=65535;

    {
        let mut ws: StreamWriter = StreamWriter::from_big_endian(&mut ms).unwrap();
        ws.write_u16(&w1).unwrap();
        ws.write_u16(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from_big_endian(&mut ms).unwrap();
    let r1=rs.read_u16().unwrap();
    let r2=rs.read_u16().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}



#[test]
fn test_i32(){

    let mut ms=MemoryStream::new();

    let w1:i32 =-2147483648;
    let w2:i32=2147483647;

    {
        let mut ws: StreamWriter = StreamWriter::from_big_endian(&mut ms).unwrap();
        ws.write_i32(&w1).unwrap();
        ws.write_i32(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from_big_endian(&mut ms).unwrap();
    let r1=rs.read_i32().unwrap();
    let r2=rs.read_i32().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}


#[test]
fn test_u32(){

    let mut ms=MemoryStream::new();

    let w1:u32 = 0;
    let w2:u32=4294967295;

    {
        let mut ws: StreamWriter = StreamWriter::from_big_endian(&mut ms).unwrap();
        ws.write_u32(&w1).unwrap();
        ws.write_u32(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from_big_endian(&mut ms).unwrap();
    let r1=rs.read_u32().unwrap();
    let r2=rs.read_u32().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}



#[test]
fn test_i64(){

    let mut ms=MemoryStream::new();

    let w1:i64 =-9223372036854775808;
    let w2:i64=9223372036854775807;

    {
        let mut ws: StreamWriter = StreamWriter::from_big_endian(&mut ms).unwrap();
        ws.write_i64(&w1).unwrap();
        ws.write_i64(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    {
        let mut rs: StreamReader = StreamReader::from_big_endian(&mut ms).unwrap();
        let r1 = rs.read_i64().unwrap();
        let r2 = rs.read_i64().unwrap();

        assert_eq!(w1, r1);
        assert_eq!(w2, r2);
    }
    ms.set_position(0).unwrap();


}


#[test]
fn test_u64(){

    let mut ms=MemoryStream::new();

    let w1:u64 = 0;
    let w2:u64=1844674407370955161;

    {
        let mut ws: StreamWriter = StreamWriter::from_big_endian(&mut ms).unwrap();
        ws.write_u64(&w1).unwrap();
        ws.write_u64(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from_big_endian(&mut ms).unwrap();
    let r1=rs.read_u64().unwrap();
    let r2=rs.read_u64().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);

    //close
}
