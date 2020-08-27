# Rust data conversion demo

This is a demo to show how to use rust convention for converting different data type

For running the live testing, plz run: `cargo watch -c --exec 'test -- --nocapture'`

- `std::str::FromStr`: Any type `T` implemented this trait will be able to call `T::from_str(&str)`

- `std::convert::{From, Into}`: Any type `T` implemented the `From<U>` trait will be able to call:
    - `T::from(U)`
    - `U.into()`

# Examples
```rust
// Get back the `HttpServerConfig` from `&str`, as it implemented `FromStr` trait.
let test_connection_str = "https:www.rust-lang.org";
let temp_result = HttpServerConfig::from_str(test_connection_str);

// Get back `UdpServerConfig` from `HttpServerConfig`, as it implemented `From<HttpServerConfig>` trait.
let config: UdpServerConfig = temp_result.unwrap().into();
println!(
    "test_connection_str (into UdpServerConfig): {:?}",
    test_connection_str
);

println!("config: {:?}", &config)
assert_eq!(type_of(&config), "from_into_train_demo::UdpServerConfig");
assert_eq!(config.host, "www.rust-lang.org");
assert_eq!(config.port, 443);
```
