# Stringly-Typed Rust

A crate for updating values using a stringly-typed API.

## Typical Use Case

Imagine you're working on a system that uses a lot of runtime configuration to
alter the behaviour of the application. For whatever reason it isn't practical
to stop the entire system just to update a single configuration key, so you need
the ability to update configuration on the fly... How do you do it?

- Re-upload the entire configuration file for every change
- The caller sends just the key-value pair they want to update and you execute 
  the update by:
  - Serializing to a more loosly-typed form (e.g. `serde_json::Value`), make the
    update (e.g. `config["foo"]["bar"][3] = 42`), then deserialize back to the
    original type
  - Write a massive switch-case statement which will update different fields
    depending on the provided key (e.g. `match key { "foo.bar" => config.foo.bar = value }`)

In terms of difficulty the first option is quite nice. You just wrap your 
`config` with a `RWLock` and all your configuration update issues go away, 
however you now pay the price of serializing/deserializing and network transfer.
Plus it can feel awfully wasteful to copy around an entire file just to change
one key.

TODO: Mention how serialize-deserialize is expensive

TODO: Mention this is essentially automating the massive switch-case statement

TODO: Add an example

## Features

- [x] Works with `no_std`
- [ ] Supports enums
- [ ] Supports arrays

## Benchmarks

The main goal of this crate *isn't* performance, however it performs quite well
compared to the "usual" static assignment (i.e. normal Rust with the dot
operator), and still beats the serialize-update-deserialize method.

```text
test static_assign         ... bench:          81 ns/iter (+/- 6)
test stringly_update       ... bench:         167 ns/iter (+/- 12)
test serialize_deserialize ... bench:       1,243 ns/iter (+/- 343)
```

As with any benchmark, we're only comparing three contrived use cases so take
these numbers with a grain of salt.