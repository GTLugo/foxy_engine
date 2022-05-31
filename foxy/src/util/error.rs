//https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html
use thiserror::Error;
//use std::backtrace::Backtrace;

#[derive(Error, Debug)]
pub enum FoxyError {
  #[error("ERROR: {}", msg)]
  Error {
    msg: String,
    //backtrace: Backtrace,
  },
  #[error(transparent)]
  Anyhow { #[from] /*#[backtrace] */source: anyhow::Error,},
}

impl FoxyError {
  pub fn new(msg: &str) -> Self {
    FoxyError::Error {
      msg: msg.to_string(),
      //backtrace: Backtrace::capture()
    }
  }
}