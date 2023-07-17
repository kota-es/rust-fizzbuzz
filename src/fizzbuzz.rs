use std::fmt;

pub enum FizzBuzz {
  Fizz,
  Buzz,
  FizzBuzz,
  Number(String),
}

impl From<u32> for FizzBuzz {
  fn from(x: u32) -> FizzBuzz {
    match ((x % 3) == 0, (x % 5) == 0) {
      (true, true) => FizzBuzz::FizzBuzz,
      (true, _) => FizzBuzz::Fizz,
      (_, true) => FizzBuzz::Buzz,
      _ => FizzBuzz::Number(x.to_string()),
    }
  }
}

impl fmt::Display for FizzBuzz {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      FizzBuzz::Fizz => write!(f, "Fizz"),
      FizzBuzz::Buzz => write!(f, "Buzz"),
      FizzBuzz::FizzBuzz => write!(f, "FizzBuzz"),
      FizzBuzz::Number(x) => write!(f, "{x}"),
    }
  }
}