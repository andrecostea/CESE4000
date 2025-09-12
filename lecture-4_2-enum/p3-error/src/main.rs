use std::fs;
use std::path::Path;
use std::io;

#[derive(Debug)]
enum MyError {
    DoesntExist,
    Io(io::Error),
    TBI,
    // ...
}

// We are implementing the From trait to convert an io::Error into our own error type, MyError
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

fn check_if_file_exists(filename: &Path) -> bool {
    filename.exists() && filename.is_file()
}

fn read_from_file(filename: &Path) -> Result<String, MyError> {
    if !check_if_file_exists(filename) {
        return Err(MyError::DoesntExist)
    }

    // you can add more checks here, e.g. permissions, file size, etc.

    // USING TRAITS
    // let contents = fs::read_to_string(filename)?; // uses `From<io::Error>` above
    
    let res =  fs::read_to_string(filename);
    let content = match res {
        Ok(r)  => r,
        Err(e) => return Err(MyError::Io(e)),

    }; 
    Ok(content)
}

fn main() {
    // relative to the current working directory, the project's root
    let path: &Path = Path::new("myfile.txt");

    match read_from_file(path) {
        Ok(contents)         => println!("File contents:\n{}", contents),
        Err(MyError::DoesntExist)    => eprintln!("File does not exist."),
        Err(MyError::Io(e))   => eprintln!("I/O error: {}", e),
        TBI => eprintln!("Some features are yet to be implemented"),
    }
}