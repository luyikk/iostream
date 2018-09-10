use iostream::io::*;


#[test]
fn test_reads(){
    let mut data:[u8;255]=[0;255];
    for i in 0..255{
        print!("{}",i);
        data[i]=i as u8;
    }

    let mut ms= MemoryStream::new_to(&data);

    for _s in 0..10 {
        let d=  ms.read_byte().unwrap();
        print!("{}",d);
    }

    println!();
    println!("{}",ms.position());

    ms.set_position(0).unwrap();

    print!("0,");
    for i in 0..25 {

        let mut data:Vec<u8>=vec![0;250];

        let _size = ms.read(&mut data, i*10, 10).unwrap();

        for i in data {
            if i!=0{
                print!("{},", i);
            }
        }
        println!();
    }
    println!();

    let mut p:Vec<u8>=Vec::new();

    ms.read_all(&mut p).unwrap();

    for i  in p {
        print!("{}",i);
    }

    println!();

    ms.set_position(0).unwrap();
    let mut p:Vec<u8>=vec![0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xff];

    ms.read_all(&mut p).unwrap();

    for i  in p {
        print!("{}",i);
    }
    println!();


}
