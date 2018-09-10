use iostream::io::*;


#[test]
fn test_i16(){

    let mut ms=MemoryStream::new();

    let w1:i16 = -32768;
    let w2:i16=32767;

    {
        let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
        ws.write_i16(&w1).unwrap();
        ws.write_i16(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from(&mut ms).unwrap();
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
        let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
        ws.write_u16(&w1).unwrap();
        ws.write_u16(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from(&mut ms).unwrap();
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
        let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
        ws.write_i32(&w1).unwrap();
        ws.write_i32(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from(&mut ms).unwrap();
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
        let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
        ws.write_u32(&w1).unwrap();
        ws.write_u32(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from(&mut ms).unwrap();
    let r1=rs.read_u32().unwrap();
    let r2=rs.read_u32().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}



#[test]
fn test_i64(){

    let mut ms=MemoryStream::new();

    let w1:i64 =-2147483648;
    let w2:i64=2147483647;

    {
        let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
        ws.write_i64(&w1).unwrap();
        ws.write_i64(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from(&mut ms).unwrap();
    let r1=rs.read_i64().unwrap();
    let r2=rs.read_i64().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}


#[test]
fn test_u64(){

    let mut ms=MemoryStream::new();

    let w1:u64 = 0;
    let w2:u64=4294967295;

    {
        let mut ws: StreamWriter = StreamWriter::from(&mut ms).unwrap();
        ws.write_u64(&w1).unwrap();
        ws.write_u64(&w2).unwrap();
    }

    ms.set_position(0).unwrap();
    let mut rs:StreamReader=StreamReader::from(&mut ms).unwrap();
    let r1=rs.read_u64().unwrap();
    let r2=rs.read_u64().unwrap();

    assert_eq!(w1,r1);
    assert_eq!(w2,r2);
}

#[test]
fn test_read_array()
{
    let mut ms=MemoryStream::new();
    let data:[u8;1024]=[5;1024];
    ms.write_all(&data).unwrap();

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let rt= rs.read_u8_array(1024).unwrap();
    assert_eq!(data[..],rt[..] );
}

#[test]
fn test_read_single()
{
    let mut ms=MemoryStream::new();
    let f1: f32 = 0.5;

    {
        let mut ws = StreamWriter::from(&mut ms).unwrap();
        ws.write_single(&f1).unwrap();
    }

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let f2= rs.read_single().unwrap();

    assert_eq!(f1,f2);
}

#[test]
fn test_read_double()
{
    let mut ms=MemoryStream::new();
    let f1: f64 = 0.5553214111111111;

    {
        let mut ws = StreamWriter::from(&mut ms).unwrap();
        ws.write_double(&f1).unwrap();
    }

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let f2= rs.read_double().unwrap();

    assert_eq!(f1,f2);
}

#[test]
fn test_wr_string(){
    let mut ms=MemoryStream::new();
    let str="aaaa1aaaa:)";

    {
        let mut ws = StreamWriter::from(&mut ms).unwrap();
        ws.write_string(str).unwrap();
    }

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let re= rs.read_string().unwrap();

    assert_eq!(str,&re);

    println!("{}",re);
}


#[test]
fn test_wr_line()
{
    let mut ms=MemoryStream::new();
    let str="aaaa1aaaa:)";
    {
        let mut ws = StreamWriter::from(&mut ms).unwrap();
        ws.write_line(&str).unwrap();
    }

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let re= rs.read_line().unwrap();

    assert_eq!(str,&re[..(re.len()-1)]);

    println!("{}",rs.base_stream.position());

    let re= rs.read_line().unwrap();

    println!("{}",re);
}


#[test]
fn test_wr_lines()
{
    let mut ms=MemoryStream::new();
    let str="aaaa1aaaa:)";
    {
        let mut ws = StreamWriter::from(&mut ms).unwrap();
        ws.write_line(&str).unwrap();
        ws.write_line(&str).unwrap();
        ws.write_line(&str).unwrap();
    }

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let re= rs.read_all_lines().unwrap();

    for x in re{
        assert_eq!(str,&x[..(x.len()-1)]);
        print!("{}",x);
    }


}

#[test]
fn test_wr_all_text()
{
    let mut ms=MemoryStream::new();

    let mut strlist:Vec<&str>=Vec::new();
    for _i in 0..10{
        strlist.push("aaaaaaaaaa1");
    }

    {
        let mut ws = StreamWriter::from(&mut ms).unwrap();
        ws.write_all_lines(&strlist).unwrap();
    }

    ms.set_position(0).unwrap();

    let mut rs=StreamReader::from(&mut ms).unwrap();
    let re= rs.read_all_text().unwrap();

    println!("{}",re);


}