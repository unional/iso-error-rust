pub mod multi_error;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IsoError {
  pub name: String,
  pub message: String,
  pub errors: Option<Vec<IsoError>>,
  source_error: Option<Box<IsoError>>,
}

impl IsoError {
  pub fn new(name: &str, message: &str) -> Self {
    IsoError {
      name: name.to_string(),
      message: message.to_string(),
      errors: None,
      source_error: None,
    }
  }
  pub fn new_with_cause(name: &str, message: &str, errors: IsoError) -> Self {
    IsoError {
      name: name.to_string(),
      message: message.to_string(),
      source_error: Some(Box::new(errors)),
      errors: None,
    }
  }
  pub fn new_with_causes(name: &str, message: &str, errors: Vec<IsoError>) -> Self {
    IsoError {
      name: name.to_string(),
      message: message.to_string(),
      source_error: Some(Box::new(IsoError::from_multi(&errors))),
      errors: Some(errors),
    }
  }
  pub fn from_multi(errors: &Vec<IsoError>) -> Self {
    IsoError {
      name: "MultiError".to_string(),
      message: "Some message".to_string(),
      errors: None,
      source_error: None,
    }
  }
}

impl fmt::Display for IsoError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Error for IsoError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match &self.source_error {
      Some(err) => Some(err),
      None => None
    }
  }
}

fn source_helper(e: &IsoError) -> Option<&(dyn Error + 'static)> {
  match &e.errors {
    Some(error) => Some(error.get(0).unwrap()),
    None => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let actual = IsoError::new("MyErr", "some message");

    assert_eq!("MyErr", actual.name);
    assert_eq!("some message", actual.message);
    assert!(actual.errors.is_none());
  }

  #[test]
  fn new_with_causes() {
    let cause = IsoError::new("Internal", "internal");
    let actual = IsoError::new_with_causes("MyErr", "some message", vec![cause]);

    assert_eq!("MyErr", actual.name);
    assert_eq!("some message", actual.message);
    assert!(actual.errors.is_some());
  }

  #[test]
  fn source_with_no_cause() {
    let err = IsoError::new("MyErr", "some message");
    let actual = err.source();

    assert!(actual.is_none());
  }

  // #[test]
  // fn source_with_single_cause() {
  //   let cause = IsoError::new("Internal", "internal");
  //   let err = IsoError::new_with_causes("MyErr", "some message", vec![cause]);
  //   let actual = err.source().unwrap();

  //   assert_eq!("internal", actual.to_string());
  // }
}
