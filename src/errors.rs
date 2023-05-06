use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    // panic!("dummy panic!")
    let f = File::open("dummy.txt");
    match f {
        Ok(file) => println!("file opened {:?}", file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => println!("file not found, try to create it"),
            _ => panic!("other errors {:?}", err)
        }
    }

    // let f = File::open("dummy.txt").unwrap_or_else(|err| {
    //     if err.kind() == ErrorKind::NotFound {
    //         println!("file not found, try to create it");
    //     } else {
    //         panic!("other errors {:?}", err)
    //     }
    // });

    let _f = open_a_file_demo("dummy.txt").expect("ERROR!!!");
}

fn open_a_file_demo(file_name: &str) -> Result<File, io::Error> {
    return Ok(File::open(file_name)?);
}