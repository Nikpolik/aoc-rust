#![allow(dead_code)]

use std::str::FromStr;

pub trait StringUtils {
    fn safe_parse<T: FromStr>(&self) -> Option<T>;
}

impl StringUtils for String {
    fn safe_parse<T: FromStr>(&self) -> Option<T> {
        match self.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}

impl StringUtils for str {
    fn safe_parse<T: FromStr>(&self) -> Option<T> {
        match self.parse::<T>() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
