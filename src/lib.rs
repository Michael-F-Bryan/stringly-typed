//! A crate for updating values and indexing into structs by using ...

#![no_std]

pub const DOUBLE_TYPE: &'static str = "double";
pub const INTEGER_TYPE: &'static str = "integer";

pub trait StringlyTyped {
    fn set<K, S>(&mut self, keys: K, value: Value) -> Result<(), UpdateError>
    where K: IntoIterator<Item = S>,
          S: AsRef<str>;

    fn get<K, S>(&self, keys: K) -> Result<Value, UpdateError>
    where K: IntoIterator<Item = S>,
          S: AsRef<str>;

    fn data_type(&self) -> &'static str;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateError {
    TypeError { found: &'static str, expected: &'static str },
    TooManyKeys { elements_remaning: usize },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Double(f64),
}

impl Value {
    pub fn data_type(&self) -> &'static str {
        match *self {
            Value::Integer(_) => INTEGER_TYPE,
            Value::Double(_) => DOUBLE_TYPE,
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

macro_rules! impl_primitive_type {
    ($type:ty, $variant:ident, $data_type:expr) => {
        impl StringlyTyped for $type {
            fn set<K, S>(&mut self, keys: K, value: Value) -> Result<(), UpdateError>
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

            fn get<K, S>(&self, keys: K) -> Result<Value, UpdateError>
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::iter;

    #[test]
    fn update_some_primitives() {
        let empty = iter::empty::<&str>();

        let mut integer: i64 = 42;
        integer.set(empty.clone(), Value::Integer(-7)).unwrap();
        assert_eq!(integer, -7);

        let mut float: f64 = 3.14;
        float.set(empty.clone(), Value::Double(42.0)).unwrap();
        assert_eq!(float, 42.0);
    }

    #[test]
    fn get_some_primitives() {
        let empty = iter::empty::<&str>();

        let integer: i64 = 42;
        let got = integer.get(empty.clone()).unwrap();
        assert_eq!(got, Value::from(integer));

        let float: f64 = 3.14;
        let got = float.get(empty.clone()).unwrap();
        assert_eq!(got, Value::from(float));
    }

    #[test]
    fn primitives_detect_type_errors() {
        let empty = iter::empty::<&str>();

        let mut integer: i64 = 42;
        let got = integer.set(empty.clone(), Value::Double(0.0)).unwrap_err();
        assert_eq!(got, UpdateError::TypeError { found: DOUBLE_TYPE, expected: INTEGER_TYPE });

        let mut float: f64 = 3.14;
        let got = float.set(empty.clone(), Value::Integer(0)).unwrap_err();
        assert_eq!(got, UpdateError::TypeError { found: INTEGER_TYPE, expected: DOUBLE_TYPE });
    }

    #[test]
    fn primitives_detect_over_indexing() {
        let key = "foo.bar".split(".");
        let mut n: i64 = 42;
        let should_be = UpdateError::TooManyKeys { elements_remaning: 2 };

        let got = n.set(key, Value::Integer(7)).unwrap_err();
        assert_eq!(got, should_be);
    }
}