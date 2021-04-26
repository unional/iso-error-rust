use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MultiError {
  errors: Vec<Box<dyn Error>>,
}

impl MultiError {
  pub fn new(errors: Vec<Box<dyn Error>>) -> Self {
    MultiError { errors }
  }
}

impl fmt::Display for MultiError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      &self
        .errors
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("\n")
    )
  }
}
impl Error for MultiError {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create() {
    let err = MultiError::new(vec![
      Box::new(std::ffi::CString::new(b"f\0oo".to_vec()).unwrap_err()),
      Box::new(std::ffi::CString::new(b"f\0oo".to_vec()).unwrap_err()),
    ]);

    println!("{}", err);
  }
}
