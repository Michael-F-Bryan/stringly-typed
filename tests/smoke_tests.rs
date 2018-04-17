// Notice we don't need to pull in the stringly_typed_derive crate
#[macro_use]
extern crate stringly_typed;

use stringly_typed::{StringlyTyped, UpdateError};

#[derive(StringlyTyped, Debug, Clone, PartialEq, Default)]
struct Outer {
  inner: Inner,
}

#[derive(StringlyTyped, Debug, Clone, PartialEq, Default)]
struct Inner {
  x: f64,
  y: i64,
}

// TODO: Add enum support
// #[derive(StringlyTyped, Debug, Clone, PartialEq, Default)]
// enum Enum {
//   First(u64),
//   Second(Inner),
//   Third {
//     left: u64,
//     right: Inner,
//   }
// }

#[test]
fn detect_when_key_is_too_short() {
  let thing = Outer::default();
  
  let err = thing.get("inner").unwrap_err();
  assert_eq!(err, UpdateError::CantSerialize { data_type: "Inner" });
}