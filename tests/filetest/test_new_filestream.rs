use iostream::io::*;
use std::fs::remove_file;


fn remove_file_s()
{
    let _r= remove_file("1.txt");
    let _r=remove_file("2.txt");
    let _r=remove_file("3.txt");
}

#[test]
fn test_create()
{
    {
        let r = FileStream::new("1.txt", FileMode::Create, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }
    {
        let r = FileStream::new("1.txt", FileMode::CreateNew, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }
    {
        let r = FileStream::new("3.txt", FileMode::Create, FileAccess::Write);
        if let Err(i) = r { panic!(i) }
    }
    {
        let r = FileStream::new("3.txt", FileMode::CreateNew, FileAccess::Write);
        if let Err(i) = r { panic!(i) }
    }

    remove_file_s();
}

#[test]
#[should_panic]
fn test_create_2()
{
    {
        let r = FileStream::new("2.txt", FileMode::Create, FileAccess::Read);
        if let Err(i) = r { panic!(i) }
    }
}

#[test]
#[should_panic]
fn test_create_3()
{
    {
        let r = FileStream::new("2.txt", FileMode::CreateNew, FileAccess::Read);
        if let Err(i) = r { panic!(i) }
    }
}

#[test]
fn test_open()
{
    remove_file_s();

    let r=FileStream::new("1.txt", FileMode::OpenOrCreate, FileAccess::ReadWrite);
    if let Err(i)=r{ panic!(i) }
    let r=FileStream::new("1.txt", FileMode::Open, FileAccess::ReadWrite);
    if let Err(i)=r{ panic!(i) }

    {
        let r = FileStream::new("2.txt", FileMode::CreateNew, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }

    let r=FileStream::new("2.txt", FileMode::Open, FileAccess::Read);
    if let Err(i)=r{ panic!(i) }

    let r=FileStream::new("3.txt", FileMode::OpenOrCreate, FileAccess::Write);
    if let Err(i)=r{ panic!(i) }
    let r=FileStream::new("3.txt", FileMode::Open, FileAccess::Write);
    if let Err(i)=r{ panic!(i) }

    remove_file_s();
}

#[test]
#[should_panic]
fn test_open_err()
{
    let r=FileStream::new("2.txt", FileMode::OpenOrCreate, FileAccess::Read);
    if let Err(i)=r{ panic!(i) }
}

#[test]
fn test_truncate()
{


    {
        let r = FileStream::new("2.txt", FileMode::CreateNew, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }
    {
        let r = FileStream::new("2.txt", FileMode::Truncate, FileAccess::Write);
        if let Err(i) = r { panic!(i) }
    }
    {
        let r = FileStream::new("2.txt", FileMode::Truncate, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }

    remove_file_s();
}

#[test]
#[should_panic]
#[ignore]
fn test_truncate_err()
{

    {
        let r = FileStream::new("2.txt", FileMode::CreateNew, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }

    {
        let r = FileStream::new("2.txt", FileMode::Truncate, FileAccess::Read);
        if let Err(i) = r { panic!(i) }
    }
    remove_file_s();
}

#[test]
fn test_append()
{

   


    {
        let r = FileStream::new("2.txt", FileMode::CreateNew, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }


    {
        let r = FileStream::new("2.txt", FileMode::Append, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }

    {
        let r = FileStream::new("2.txt", FileMode::Append, FileAccess::Write);
        if let Err(i) = r { panic!(i) }
    }

    remove_file_s();
}


#[test]
#[should_panic]
fn test_append_err()
{

    {
        let r = FileStream::new("2.txt", FileMode::CreateNew, FileAccess::ReadWrite);
        if let Err(i) = r { panic!(i) }
    }

    {
        let r = FileStream::new("2.txt", FileMode::Append, FileAccess::Read);
        if let Err(i) = r { panic!(i) }
    }

    remove_file_s();
}

