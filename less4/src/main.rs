use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    // panic!("testing !");

    //  Unrecoverrable Errors: Panic
    // let v = vec![1, 2, 3];
    // v[99];

    let get_greet_text_result=File::open("hello.txt");

    let get_text=match get_greet_text_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // let greeting_file = File::open("hello.txt").unwrap(); // more verbose than match expressions, will return val if found or else err
    let mut greeting_file = File::open("hello.txt").expect("File not found"); // similar to unwrap but lets us customize the err logs
    //  In prod, most use expect than unwrap

    let mut username = String::new();

    match greeting_file.read_to_string(&mut username) {
        Ok(_) => Ok(username.clone()),
        Err(e) => Err(e),
    };
    println!("{username:}");
}
