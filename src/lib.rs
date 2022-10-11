mod api;
pub mod session;

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
