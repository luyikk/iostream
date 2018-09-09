use iostream::io::*;
use std::fs::*;

#[test]
fn test_write_all()
{
    let mut ms = MemoryStream::new();

    let data: [u8; 1024] = [1; 1024];
    let mut rdata: Vec<u8> = Vec::new();
    ms.write_all(&data).unwrap();

    ms.set_position(0).unwrap();
    let len = ms.read_all(&mut rdata).unwrap();

    assert_eq!(len,data.len() );

    assert_eq!(&data[..],&rdata[..] );

    ms.clear();

    let data: [u8; 1024] = [2; 1024];

    ms.write_all(&data).unwrap();

    println!("{}",ms.position());

    let data2= ms.to_vec();

    assert_eq!(data[..],data2[..] );


}

#[test]
fn test_write()
{
    let mut ms = MemoryStream::new();
    let data: [u8; 50] = [1; 50];
    ms.write(&data,0,50).unwrap();
    let data2:[u8;100]=[2;100];
    ms.write(&data2,50,50).unwrap();

    let pdata=ms.to_vec();
    println!();
    for i in  pdata{
        print!("{}",i)
    }

    ms.seek(0,SeekOrigin::Begin).unwrap();

    let mut rdata:[u8;50]=[0;50];
    ms.read(&mut rdata,0,50).unwrap();

    let mut rdata2:[u8;50]=[0;50];
    ms.read(&mut rdata2,0,50).unwrap();

    assert_eq!(rdata[..],data[..]);

    assert_eq!(rdata2[..],data2[50..] );
}

#[test]
fn test_copy()
{

    let mut ms = MemoryStream::new();
    let mut ms2 = MemoryStream::new();

    let data: [u8; 50] = [49; 50];
    ms.write(&data,0,50).unwrap();

    let data2:[u8;100]=[50;100];
    ms.write(&data2,50,50).unwrap();

    ms.set_position(0).unwrap();

    ms.copy_to(&mut ms2).unwrap();

    ms.set_position(0).unwrap();
    let mut rdata:[u8;50]=[0;50];
    ms.read(&mut rdata,0,50).unwrap();
    let mut rdata2:[u8;50]=[0;50];
    ms.read(&mut rdata2,0,50).unwrap();

    println!();

    for i in rdata.iter()  {
        print!("{}",i)
    }

    for i in rdata2.iter()  {
        print!("{}",i)
    }
    println!();

    assert_eq!(rdata[..],data[..]);
    assert_eq!(rdata2[..],data2[50..] );

    let mut fs=File::open_fs("c.data",FileMode::CreateNew,FileAccess::ReadWrite).unwrap();

    ms.copy_to(&mut fs).unwrap();

    fs.set_position(0).unwrap();

    let mut fdata:Vec<u8>=Vec::new();
    fs.read_all(&mut fdata).unwrap();

    let msdata=ms.to_vec();

    assert_eq!(fdata,msdata);

    remove_file("c.data").unwrap();

}