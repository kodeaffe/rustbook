use std::fs::File;
//use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    /*
    let mut f = File::open("hello.txt")?;
    f.read_to_string(&mut s)?;
     */
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


pub fn run() {
    /*
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    */
    /*
        let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
     */
//    let f = File::open("hello1.txt").unwrap();
//    let _ = File::open("hello1.txt").expect("Failed to open hello1.txt");
    read_username_from_file().unwrap();
}