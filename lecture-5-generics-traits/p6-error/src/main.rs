use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum MyError {
    Error1,
    Error2,
    Error3,
    Error4,
    Error5,
    Error6,
    Io(io::Error),
}

 
impl From<io::Error> for MyError {
    fn from(err: io::Error) -> Self {
        MyError::Io(err)
    }
}

fn from_file(filename: &Path) -> Result<i32, MyError>{
    let res = fs::read_to_string(filename);
    /* let content = match res {
         Ok(i) => i,
        Err(e) => return Err(MyError::Io(e)),
    };  */
    let content = res?;

    Ok(1)
}

fn foo1(x: i32) -> Result<i32, MyError>{
    if true {
        Ok(x)
    } else {
        Err(MyError::Error1)
    }
}

fn foo2(x: i32) -> Result<i32, MyError>{
    if true {
        Ok(x)
    } else {
        Err(MyError::Error2)
    }
}

fn foo3(x: i32) -> Result<i32, MyError>{
    if true {
        Ok(x)
    } else {
        Err(MyError::Error3)
    }
}

fn foo4(x: i32) -> Result<i32, MyError>{
    if true {
        Ok(x)
    } else {
        Err(MyError::Error4)
    }
}

fn foo5(x: i32) -> Result<i32, MyError>{
    if true {
        Ok(x)
    } else {
        Err(MyError::Error5)
    }
}

fn main_fnc() -> Result<i32, MyError> {
    let x = 5;
    let res = match foo1(x){
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    let res = match foo2(res){
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    let res = match foo3(res){
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    let res = match foo4(res){
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    let res = match foo5(res){
        Ok(i) => i,
        Err(e) => return Err(e),
    };
    Ok(res)

}

fn main_fnc_compact() -> Result<i32, MyError> {
    let x = 5;
    let res = foo1(x)?;
    let res = foo2(res)?;
    let res = foo3(res)?;
    let res = foo4(res)?;
    let res = foo5(res)?;
    Ok(res)

}

fn main(){
    
}