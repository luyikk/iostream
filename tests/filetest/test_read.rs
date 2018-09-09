extern  crate iostream;
use iostream::io::*;
use std::io::Write;
use std::fs::remove_file;


fn make_file()
{
    let mut file = FileStream::new("1.data", FileMode::CreateNew, FileAccess::Write).expect("not make 1.data");
    let data: [u8; 1024] = [1; 1024];
    file.file.write_all(&data).expect("not write data to 1.data");
}

#[test]
fn read_test()
{
    make_file();



    let mut fs=FileStream::new("1.data",FileMode::Open,FileAccess::Read).expect("not read 1.data");

    let mut data=Vec::new();
    let lengt=  fs.read_all(&mut data).unwrap();
    println!("\n");
    for i in data.iter(){
        print!("{}",i);
    }
    println!("\n");
    assert_eq!(lengt,data.len());
    fs.set_position(0).unwrap();



    let mut readlengt=0;

    let mut data:[u8;100]=[0;100];

    let lengt=  fs.read(&mut data,50,20).unwrap();

    readlengt+=lengt;

    for i in data.iter(){
        print!("{}",i);
    }
    println!("\n");

    assert_eq!(lengt,20,"{}",lengt);

    assert_eq!(readlengt as u64 ,fs.position());


    let mut data=Vec::new();
    let lengt=  fs.read_all(&mut data).expect("");

    println!("{}",lengt);

    assert_eq!((lengt+readlengt) as u64,fs.length());



    remove_file("1.data").unwrap();
}


