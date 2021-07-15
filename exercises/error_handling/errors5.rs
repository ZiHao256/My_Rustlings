// errors5.rs

// This program uses a completed version of the code from errors4.
// It won't compile right now! Why?
// Execute `rustlings hint errors5` for hints!

// I AM NOT DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;
#[derive(Debug)]
enum MyError{
    Parse(ParseIntError),
    Positive(CreationError),
}

impl From<ParseIntError> for MyError{
    fn from(err: ParseIntError) -> MyError{
        MyError::Parse(err)
    }
}
impl From<CreationError> for MyError{
    fn from(err: CreationError) -> MyError{
        MyError::Positive(err)
    }
}

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), MyError> {
    let pretend_user_input = "42";
    let x: i64 = try!(pretend_user_input.parse().map_err(MyError::Parse));
    println!("output={:?}", try!(PositiveNonzeroInteger::new(x).map_err(MyError::Positive)));
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
