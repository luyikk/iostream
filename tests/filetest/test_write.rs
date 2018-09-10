extern crate iostream;
use iostream::io::*;
use std::fs::remove_file;
use std::fs::File;


#[test]
fn write_test()
{
    let mut fs=File::open_fs("2.data",FileMode::CreateNew,FileAccess::Write)
        .expect("not create fs");

    let testdata:[u8;100]=[1;100];
    fs.write_all(&testdata).unwrap();
    fs.flush().unwrap();


    remove_file("2.data").unwrap();
}

#[test]
fn append_test()
{
    {
        let mut fs=File::open_fs("3.data",FileMode::CreateNew,FileAccess::Write).unwrap();

        let testdata:[u8;100]=[1;100];
        fs.write_all(&testdata).unwrap();
        fs.flush().unwrap();
    }


    let mut fs=File::open_fs("3.data",FileMode::Append,FileAccess::Write)
        .expect("not create fs");

    println!("{}",fs.position());
    assert_eq!(fs.position(),fs.length() );
    let testdata:[u8;1000]=[66;1000];
    let c= fs.write(&testdata,0,testdata.len()).unwrap();
    println!("{}",c);
    fs.flush().unwrap();




    remove_file("3.data").unwrap();
}

#[test]
fn append_fs_option(){

    File::write_all_text("b.txt","abccdeff\nffffffffffffff\naaaaaaaaaaaa\n").unwrap();
    let mut strlist:Vec<&str>=Vec::new();
    for _i in 0..10{
        strlist.push("aaaaaaaaaa1");
    }
    {
        File::append_all_lines("b.txt", &strlist).unwrap();
        File::append_line("b.txt", "close~~~!!!").unwrap();
        File::append_all_text("b.txt", ":hhhhhhhhhhhhhhhh").unwrap();
        File::write_all_text("b.txt","fxxxxxxxxxxxxxxxxx").unwrap();
    }


    let x= File::read_all_text("b.txt").unwrap();

    println!("{}",x);

    let lines=File::read_all_lines("b.txt").unwrap();


    for v in lines{
        print!("{}",v);
    }

    File::delete("b.txt").unwrap();

}