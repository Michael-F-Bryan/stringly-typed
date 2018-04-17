//! A crate for updating values and indexing into Rust types at runtime.
//!
//! # Examples
//!
//! ```rust
//! # extern crate stringly_typed;
//! # #[macro_use]
//! # extern crate stringly_typed_derive;
//! use stringly_typed::{StringlyTyped, Value};
//!
//! #[derive(StringlyTyped)]
//! struct Outer {
//!   inner: Inner,
//! }
//!
//! #[derive(StringlyTyped)]
//! struct Inner {
//!   x: f64,
//!   y: i64,
//! }
//!
//! # fn run() -> Result<(), ::stringly_typed::UpdateError> {
//! let mut thing = Outer {
//!   inner: Inner {
//!     x: 3.14,
//!     y: 42,
//!   }
//! };
//!
//! let key = "inner.y";
//! let value = -7;
//! thing.set_value(key.split("."), Value::from(value))?;
//!
//! let got = thing.get_value(key.split("."))?;
//! assert_eq!(thing.inner.y, -7);
//! # Ok(())
//! # }
//! # fn main() { run().unwrap() }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

// create a "std"-like facade for when we're compiled as no_std
#[cfg(not(feature = "std"))]
mod std {
    pub use core::iter;
}

// re-export the StringlyTyped custom derive
#[allow(unused_imports)]
#[macro_use]
extern crate stringly_typed_derive;
#[doc(hidden)]
pub use stringly_typed_derive::*;

pub const DOUBLE_TYPE: &'static str = "double";
pub const INTEGER_TYPE: &'static str = "integer";
pub const STRING_TYPE: &'static str = "string";

/// The whole point.
pub trait StringlyTyped {
    fn get(&self, key: &str) -> Result<Value, UpdateError> {
        self.get_value(key.split("."))
    }

    fn set(&mut self, key: &str, value: Value) -> Result<(), UpdateError> {
        self.set_value(key.split("."), value)
    }

    fn set_value<K, S>(&mut self, keys: K, value: Value) -> Result<(), UpdateError>
    where
        K: IntoIterator<Item = S>,
        S: AsRef<str>;

    fn get_value<K, S>(&self, keys: K) -> Result<Value, UpdateError>
    where
        K: IntoIterator<Item = S>,
        S: AsRef<str>;

    fn data_type(&self) -> &'static str;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateError {
    TypeError {
        found: &'static str,
        expected: &'static str,
    },
    TooManyKeys {
        elements_remaning: usize,
    },
    UnknownField {
        valid_fields: &'static [&'static str],
    },
    CantSerialize { data_type: &'static str },
}

/// A dynamically typed value.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Double(f64),
    #[cfg(feature = "std")]
    String(String),
    #[doc(hidden)]
    __NonExhaustive,
}

impl Value {
    pub fn data_type(&self) -> &'static str {
        match *self {
            Value::Integer(_) => INTEGER_TYPE,
            Value::Double(_) => DOUBLE_TYPE,
            #[cfg(feature = "std")]
            Value::String(_) => STRING_TYPE,
            Value::__NonExhaustive => unreachable!(),
        }
    }
}

impl From<i64> for Value {
    fn from(other: i64) -> Value {
        Value::Integer(other)
    }
}

impl From<f64> for Value {
    fn from(other: f64) -> Value {
        Value::Double(other)
    }
}

#[cfg(feature = "std")]
impl From<String> for Value {
    fn from(other: String) -> Value {
        Value::String(other)
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a str> for Value {
    fn from(other: &'a str) -> Value {
        Value::String(other.to_string())
    }
}

macro_rules! impl_primitive_type {
    ($(#[$attr:meta])* $type:ty, $variant:ident, $data_type:expr) => {
        $(#[$attr])*
        impl StringlyTyped for $type {
            fn set_value<K, S>(&mut self, keys: K, value: Value) -> Result<(), UpdateError>
            where K: IntoIterator<Item = S>,
                  S: AsRef<str> 
            {
                let mut keys = keys.into_iter();
                
                if let Some(_) = keys.next() {
                    let elements_remaning = keys.count() + 1;
                    return Err(UpdateError::TooManyKeys { elements_remaning });
                }

                match value {
                    Value::$variant(v) => {
                        *self = v;
                        Ok(())
                    }
                    _ => {
                        let e = UpdateError::TypeError { 
                            expected: self.data_type(), 
                            found: value.data_type(),
                        };
                        Err(e)
                    }
                }
            }

            fn get_value<K, S>(&self, keys: K) -> Result<Value, UpdateError>
            where K: IntoIterator<Item = S>,
                S: AsRef<str>,
            {
                let mut keys = keys.into_iter();
                
                if let Some(_) = keys.next() {
                    let elements_remaning = keys.count() + 1;
                    return Err(UpdateError::TooManyKeys { elements_remaning });
                }

                Ok(self.clone().into())
            }

            fn data_type(&self) -> &'static str {
                $data_type
            }
        }
    };
}

impl_primitive_type!(i64, Integer, INTEGER_TYPE);
impl_primitive_type!(f64, Double, DOUBLE_TYPE);
impl_primitive_type!(#[cfg(feature = "std")] String, String, STRING_TYPE);

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn update_some_primitives() {
        let empty = iter::empty::<&str>();

        let mut integer: i64 = 42;
        integer
            .set_value(empty.clone(), Value::Integer(-7))
            .unwrap();
        assert_eq!(integer, -7);

        let mut float: f64 = 3.14;
        float.set_value(empty.clone(), Value::Double(42.0)).unwrap();
        assert_eq!(float, 42.0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn update_a_string() {
        let empty = iter::empty::<&str>();

        let mut string = String::from("before");
        let new_value = String::from("after");
        string
            .set_value(empty.clone(), new_value.clone().into())
            .unwrap();
        assert_eq!(string, new_value);
    }

    #[test]
    fn get_some_primitives() {
        let empty = iter::empty::<&str>();

        let integer: i64 = 42;
        let got = integer.get_value(empty.clone()).unwrap();
        assert_eq!(got, Value::from(integer));

        let float: f64 = 3.14;
        let got = float.get_value(empty.clone()).unwrap();
        assert_eq!(got, Value::from(float));
    }

    #[cfg(feature = "std")]
    #[test]
    fn get_a_string() {
        let empty = iter::empty::<&str>();

        let string = String::from("before");
        let got = string.get_value(empty.clone()).unwrap();
        assert_eq!(got, Value::from(string));
    }

    #[test]
    fn primitives_detect_type_errors() {
        let empty = iter::empty::<&str>();

        let mut integer: i64 = 42;
        let got = integer
            .set_value(empty.clone(), Value::Double(0.0))
            .unwrap_err();
        assert_eq!(
            got,
            UpdateError::TypeError {
                found: DOUBLE_TYPE,
                expected: INTEGER_TYPE,
            }
        );

        let mut float: f64 = 3.14;
        let got = float
            .set_value(empty.clone(), Value::Integer(0))
            .unwrap_err();
        assert_eq!(
            got,
            UpdateError::TypeError {
                found: INTEGER_TYPE,
                expected: DOUBLE_TYPE,
            }
        );
    }

    #[test]
    fn primitives_detect_over_indexing() {
        let key = "foo.bar".split(".");
        let mut n: i64 = 42;
        let should_be = UpdateError::TooManyKeys {
            elements_remaning: 2,
        };

        let got = n.set_value(key, Value::Integer(7)).unwrap_err();
        assert_eq!(got, should_be);
    }
}
