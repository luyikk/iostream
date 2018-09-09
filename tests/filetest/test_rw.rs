extern crate iostream;
use iostream::io::*;
use std::fs::{File,remove_file};


#[test]
fn test_copy(){
    {
        let mut fs1 = File::open_fs("5.data", FileMode::CreateNew, FileAccess::ReadWrite).unwrap();

        let i: [u8; 1024] = [55; 1024];
        fs1.write(&i, 0, i.len()).unwrap();
        fs1.flush().unwrap();

        let mut fs2 = File::open_fs("6.data", FileMode::CreateNew, FileAccess::Write).unwrap();
        fs1.copy_to(&mut fs2).unwrap();
        fs2.flush().unwrap();
    }


    let mut fs1 = File::open_fs("5.data", FileMode::Open, FileAccess::Read).unwrap();
    let mut fs2 =File::open_fs("6.data", FileMode::Open, FileAccess::Read).unwrap();
    let mut data1=Vec::new();
    fs1.read_all(&mut data1).unwrap();
    let mut data2:Vec<u8>=Vec::new();
    fs2.read_all(&mut data2).unwrap();
    assert_eq!(data1,data2);
    remove_file("5.data").unwrap();
    remove_file("6.data").unwrap();

}

#[test]
fn test_rw(){

    let mut fs = File::open_fs("7.data", FileMode::CreateNew, FileAccess::ReadWrite).unwrap();

    let pd:Vec<u8>=vec![1,2,3,4,5];

    fs.write_all(&pd).unwrap();

    let pr:Vec<u8>=vec![1,2,3,4,5,6,7,8,9,0];

    fs.write(&pr,5,5).unwrap();

    fs.set_position(0).unwrap();

    let mut rdata:Vec<u8>=vec![0;1024];
    let len=rdata.len();

    let size= fs.read(&mut rdata,0,len).unwrap();

    rdata.resize(size,0);

    println!("{:?}",rdata);

    fs.set_position(0).unwrap();

    let mut rdatac:Vec<u8>=Vec::new();

    println!("{}",fs.length());

    fs.read_all(&mut rdatac).unwrap();

    assert_eq!(rdata,rdatac);

    remove_file("7.data").unwrap();

}