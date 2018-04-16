// Notice we don't need to pull in the stringly_typed_derive crate
#[macro_use]
extern crate stringly_typed;

#[derive(StringlyTyped)]
struct Outer {
  inner: Inner,
}

#[derive(StringlyTyped)]
struct Inner {
  x: f64,
  y: i64,
}